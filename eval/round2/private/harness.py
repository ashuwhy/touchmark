"""Round 2: prompts, test programs and scoring, with a time limit on every hidden test."""

import hashlib, json, os, pty, re, select, subprocess, sys, tempfile, time
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from gen import G, RUST_GEN, rust_call, tmk_fn
from tasks import TASKS, BY_NAME

TMK = os.path.expanduser("~/.local/bin/tmk")
REF = "/work/touchmark/llms.txt"
LIMIT = 1.0    # seconds per test, wall clock, on the scoring machine
SILENCE = 10.0  # no output for this long: the running test is stopped
EXPECTED = os.path.join(os.path.dirname(os.path.abspath(__file__)), "expected.json")

TY = {"touchmark": {"int": "int", "ints": "[int]"}, "rust": {"int": "i64", "ints": "&[i64]"}}
RET = {"touchmark": {"int": "int", "ints": "[int]"}, "rust": {"int": "i64", "ints": "Vec<i64>"}}
EXT = {"touchmark": "tmk", "rust": "rs"}


def signature(t, lang):
    ps = ", ".join(f"{n}: {TY[lang][ty]}" for n, ty in t["params"])
    return f"pub fn {t['name']}({ps}) -> {RET[lang][t['ret']]}"


def prompt(t, lang, workdir):
    if lang == "touchmark":
        tool = (f"Touchmark is a new compiled language. Its complete reference is the file {REF}; read it first. "
                f"The compiler is `tmk` (on PATH).")
        name = "Touchmark"
    else:
        tool = "Use Rust (stable, edition 2021). `rustc` and `cargo` are on PATH."
        name = "Rust"
    return f"""You are implementing one function for a production codebase, in {name}.

{tool}

Work only inside the directory {workdir} (it exists and is empty). Do not read or search files outside it{' other than the language reference' if lang == 'touchmark' else ''}.

Write your final solution to {workdir}/solution.{EXT[lang]}. It must define a public function with exactly this signature:

    {signature(t, lang)}

It may also define helper functions and types. It must not define `main`: the team calls your function from its own test program.

Specification:

{t['spec']}

Your function will be reviewed and then run against hidden tests covering the whole input domain described above, including its edge cases and inputs of the largest allowed sizes. It must return the specified result for every input the specification allows, and each test has a time limit of {LIMIT:g} second{'' if LIMIT == 1 else 's'}. You may write and run your own tests in other files inside your directory.

When you are finished, reply with the single word DONE."""


def key(t, k):
    return f"{t['name']}/{k}/" + hashlib.sha256(repr(t["tests"][k]).encode()).hexdigest()[:16]


def expected_all(names=None):
    """Reference results, computed once and cached (the shadow_pairs reference is quadratic)."""
    cache = json.load(open(EXPECTED)) if os.path.exists(EXPECTED) else {}
    for t in BY_NAME.values():
        if names and t["name"] not in names:
            continue
        for k, args in enumerate(t["tests"]):
            if key(t, k) not in cache:
                real = [a.values() if isinstance(a, G) else a for a in args]
                cache[key(t, k)] = t["ref"](*real)
                json.dump(cache, open(EXPECTED, "w"))
    return cache


def expected_lines(t, k, cache):
    r = cache[key(t, k)]
    if t["ret"] == "ints":
        return [str(len(r))] + [str(x) for x in r]
    return [str(r)]


def lit(v, lang):
    if isinstance(v, list):
        body = ", ".join(str(x) for x in v)
        return f"[{body}]" if lang == "touchmark" else f"&[{body}]"
    return str(v)


def call_code(t, args, lang, k):
    pre, parts = "", []
    for j, a in enumerate(args):
        if isinstance(a, G):
            if lang == "touchmark":
                pre += f"  let in{k}_{j} = test_input_{k}_{j}()\n"
                parts.append(f"in{k}_{j}")
            else:
                pre += f"    let in{k}_{j} = {rust_call(a)};\n"
                parts.append(f"&in{k}_{j}")
        else:
            parts.append(lit(a, lang))
    call = f"{t['name']}({', '.join(parts)})"
    if lang == "touchmark":
        if t["ret"] == "int":
            return pre + f"  io.print({call})\n"
        return pre + f"  let r{k} = {call}\n  io.print(r{k}.len)\n  for i in 0..r{k}.len {{\n    io.print(r{k}[i])\n  }}\n"
    if t["ret"] == "int":
        return pre + f"    println!(\"{{}}\", {call});\n"
    return pre + f"    let r{k} = {call};\n    println!(\"{{}}\", r{k}.len());\n    for x in r{k}.iter() {{ println!(\"{{}}\", x); }}\n"


def strip_main(src):
    """Remove a `main` function the agent may have left in, by brace matching."""
    m = re.search(r"(^|\n)\s*(pub\s+)?fn\s+main\s*\(", src)
    if not m:
        return src
    i = src.index("{", m.end())
    depth = 0
    j = i
    while j < len(src):
        if src[j] == "{":
            depth += 1
        elif src[j] == "}":
            depth -= 1
            if depth == 0:
                break
        j += 1
    return src[:m.start()] + "\n" + src[j + 1:]


def program(t, lang, src, ks):
    body = "".join(call_code(t, t["tests"][k], lang, k) for k in ks)
    if lang == "touchmark":
        gens = "".join("\n" + tmk_fn(a, f"test_input_{k}_{j}")
                       for k in ks for j, a in enumerate(t["tests"][k]) if isinstance(a, G))
        return strip_main(src) + "\n" + gens + "\nfn main() uses io {\n" + body + "}\n"
    gen = RUST_GEN if any(isinstance(a, G) for k in ks for a in t["tests"][k]) else ""
    return strip_main(src) + "\n#[allow(dead_code)]" + gen + "\n#[allow(unused)]\nfn main() {\n" + body + "}\n"


def build(lang, path, out):
    if lang == "touchmark":
        cmd = [TMK, "build", path, "-o", out]
    else:
        cmd = ["rustc", "-O", "--edition", "2021", "-A", "warnings", path, "-o", out]
    try:
        r = subprocess.run(cmd, capture_output=True, text=True, timeout=900)
    except subprocess.TimeoutExpired:
        return False, "build timed out"
    return r.returncode == 0, (r.stdout + r.stderr)[-3000:]


def run_timed(exe):
    """Run under a pseudo-terminal, so that C stdio flushes every line, and timestamp each line.
    Returns (lines, stamps in seconds from start, exit code or None if stopped for silence)."""
    master, slave = pty.openpty()
    t0 = time.monotonic()
    p = subprocess.Popen([exe], stdin=subprocess.DEVNULL, stdout=slave, stderr=subprocess.DEVNULL, close_fds=True)
    os.close(slave)
    lines, stamps, buf = [], [], b""
    stopped = False
    while True:
        ready, _, _ = select.select([master], [], [], SILENCE)
        if not ready:
            p.kill()
            stopped = True
            break
        try:
            chunk = os.read(master, 1 << 16)
        except OSError:
            break
        if not chunk:
            break
        now = time.monotonic() - t0
        buf += chunk
        while b"\n" in buf:
            line, buf = buf.split(b"\n", 1)
            lines.append(line.rstrip(b"\r").decode(errors="replace"))
            stamps.append(now)
    os.close(master)
    code = p.wait()
    return lines, stamps, None if stopped else code


def run_tests(t, lang, src, ks, cache, d):
    """Results and seconds for the tests ks, run in order in one program; a crash or a stop
    costs only the test it happened in, and the rest go on in a new program."""
    results, seconds = {}, {}
    build_log = ""
    todo = list(ks)
    while todo:
        tag = f"p{todo[0]}_{len(todo)}"
        path = os.path.join(d, f"{tag}.{EXT[lang]}")
        out = os.path.join(d, tag)
        open(path, "w").write(program(t, lang, src, todo))
        ok, log = build(lang, path, out)
        if not ok:
            build_log = log
            for k in todo:
                results[k] = "build"
            break
        lines, stamps, code = run_timed(out)
        pos, prev = 0, 0.0
        done = 0
        for k in todo:
            exp = expected_lines(t, k, cache)
            got = lines[pos:pos + len(exp)]
            if len(got) < len(exp):
                break
            end = stamps[pos + len(exp) - 1]
            seconds[k] = round(end - prev, 3)
            prev = end
            results[k] = "pass" if got == exp else "wrong"
            pos += len(exp)
            done += 1
        if done == len(todo):
            break
        k = todo[done]
        results[k] = "timeout" if code is None else ("crash" if code != 0 else "wrong")
        seconds[k] = None
        todo = todo[done + 1:]
    return results, seconds, build_log


def score(t, lang, src):
    """Every hidden test, with a timeout or a run over the limit re-run once alone."""
    cache = expected_all([t["name"]])
    ks = list(range(len(t["tests"])))
    with tempfile.TemporaryDirectory() as d:
        results, seconds, build_log = run_tests(t, lang, src, ks, cache, d)
        rerun = {}
        for k in ks:
            if results[k] == "timeout" or (results[k] == "pass" and seconds[k] > LIMIT):
                r2, s2, _ = run_tests(t, lang, src, [k], cache, d)
                rerun[k] = (r2[k], s2.get(k))
                if r2[k] == "timeout" or (r2[k] == "pass" and s2[k] > LIMIT):
                    results[k] = "timeout"
                else:
                    results[k], seconds[k] = r2[k], s2[k]
    out = [results[k] for k in ks]
    return {"passed": out.count("pass"), "total": len(ks), "results": out,
            "seconds": [seconds.get(k) for k in ks], "reruns": {str(k): v for k, v in rerun.items()},
            "build_log": build_log if "build" in out else ""}


if __name__ == "__main__":
    # harness.py expected [task...]          compute and cache reference results
    # harness.py score <task> <lang> <file>  score one solution
    # harness.py prompt <task> <lang> <dir>  print a prompt
    cmd = sys.argv[1]
    if cmd == "expected":
        expected_all(sys.argv[2:] or None)
    elif cmd == "score":
        _, _, name, lang, path = sys.argv
        print(json.dumps(score(BY_NAME[name], lang, open(path).read())))
    elif cmd == "prompt":
        _, _, name, lang, workdir = sys.argv
        print(prompt(BY_NAME[name], lang, workdir))
