"""Post-hoc: run Rust solutions on fresh random inputs, against the brute forces (small inputs)
and the fast references (medium inputs). Not part of the pre-registered scoring.
difftest.py <seed> <solution.rs>..."""
import os, random, subprocess, sys, tempfile
PRIVATE = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "private")
sys.path.insert(0, PRIVATE)
from tasks import TASKS

SEED = int(sys.argv[1])
PATHS = sys.argv[2:]
src = open(os.path.join(PRIVATE, "check_refs.py")).read()
src = src[:src.index("for name, (fast, slow, make) in cases.items():")]
ns = {"__name__": "check_refs_gen"}
sys.argv = ["check_refs", str(SEED)]
exec(compile(src, "check_refs.py", "exec"), ns)
rng, cases = ns["rng"], ns["cases"]

KEY = {"palindromic_subarray_sum": "pal_sum", "widest_fence": "fence", "min_max_waiting_time": "waiting",
       "count_ratio_subarrays": "ratio", "elevator_penalty": "elevator", "max_valid_splits": "splits",
       "min_ops_subset_sum": "subset_ops", "distant_subarrays": "distant", "shadow_pairs": "shadow",
       "largest_power": "largest_power", "longest_negation_subarray": "negation"}


def a(n, lo, hi):
    return [rng.randint(lo, hi) for _ in range(n)]


def few(n, vals):
    return [rng.choice(vals) for _ in range(n)]


BIG = 10**9
# medium inputs near the value bounds, checked against the fast references
MEDIUM = {
    "palindromic_subarray_sum": lambda: (rng.choice([a(rng.randint(1, 400), 1, BIG), few(rng.randint(1, 400), [BIG, BIG - 1]),
                                                     ns["pal_arr"](400)]),),
    "widest_fence": lambda: (rng.choice([a(rng.randint(1, 300), 1, BIG), few(rng.randint(1, 300), [1, 2, 3, BIG // 2, BIG]),
                                         a(rng.randint(1, 300), 1, 40)]),),
    "min_max_waiting_time": lambda: (a(rng.randint(1, 50), 1, rng.choice([3, 20])), a(2, 1, 50)),
    "count_ratio_subarrays": lambda: (a(rng.randint(1, 400), 1, rng.choice([4, BIG])), rng.randint(1, BIG), rng.randint(1, BIG)),
    "elevator_penalty": lambda: (lambda n: (n, rng.randrange(n), rng.sample(range(n), rng.randint(1, min(n, 60)))))(rng.choice([rng.randint(1, 200), BIG])),
    "max_valid_splits": lambda: (rng.choice([few(rng.randint(2, 400), [2, 4, 6, 12, 3 * BIG // 10, BIG]), a(rng.randint(2, 400), 1, BIG),
                                             [rng.choice([1, 2, 3, 6]) * 166_666_666 for _ in range(rng.randint(2, 400))]]),),
    "min_ops_subset_sum": lambda: (a(rng.randint(1, 100), 1, rng.choice([7, 500])), rng.randint(1, 5000)),
    "distant_subarrays": lambda: (rng.choice([a(rng.randint(1, 400), -BIG, BIG), few(rng.randint(1, 400), [-BIG, BIG, 0, 1])]),
                                  rng.randint(-BIG, BIG), rng.choice([0, 1, rng.randint(0, BIG), BIG])),
    "shadow_pairs": lambda: (rng.choice([a(rng.randint(3, 400), 1, BIG), a(rng.randint(3, 400), 1, 5)]),),
    "largest_power": lambda: (rng.choice([a(rng.randint(1, 400), 0, 32767), few(rng.randint(1, 400), [32767, 16383, 21845, 10922, 0])]),),
    "longest_negation_subarray": lambda: (a(rng.randint(1, 400), -100_000, 100_000), rng.randint(1, 3000)),
}

DRIVER = """
fn main() {
    use std::io::{Read, Write};
    let mut s = String::new();
    std::io::stdin().read_to_string(&mut s).unwrap();
    let mut out = String::new();
    for line in s.lines() {
        let p: Vec<Vec<i64>> = line.split('|').map(|x| x.split_whitespace().map(|v| v.parse().unwrap()).collect()).collect();
        out.push_str(&format!("{:?}\\n", CALL));
    }
    std::io::stdout().write_all(out.as_bytes()).unwrap();
}
"""


def line(args):
    return "|".join(" ".join(map(str, x)) if isinstance(x, list) else str(x) for x in args)


def check(t, path, tmp):
    call = t["name"] + "(" + ", ".join(f"&p[{i}]" if ty == "ints" else f"p[{i}][0]" for i, (_, ty) in enumerate(t["params"])) + ")"
    prog = os.path.join(tmp, t["name"] + ".rs")
    open(prog, "w").write(open(path).read() + DRIVER.replace("CALL", call))
    exe = prog[:-3]
    b = subprocess.run(["rustc", "-O", "--edition", "2021", "-A", "warnings", "-o", exe, prog], capture_output=True, text=True)
    if b.returncode:
        return "build failed: " + b.stderr[:300]
    fast, slow, make = cases[KEY[t["name"]]]
    inputs = [(make(), slow) for _ in range(2000)] + [(MEDIUM[t["name"]](), t["ref"]) for _ in range(300)]
    r = subprocess.run([exe], input="\n".join(line(x) for x, _ in inputs) + "\n", capture_output=True, text=True, timeout=600)
    if r.returncode:
        return "crashed: " + r.stderr[-300:]
    got = r.stdout.split("\n")
    bad = [(x, g, str(f(*x))) for (x, f), g in zip(inputs, got) if g != str(f(*x))]
    return f"{len(inputs) - len(bad)}/{len(inputs)}" + (f" first mismatch {bad[0]}" if bad else "")


by_name = {t["name"]: t for t in TASKS}
with tempfile.TemporaryDirectory() as tmp:
    for path in PATHS:
        if os.path.basename(path) == "solution.rs":
            name = os.path.basename(os.path.dirname(path)).split("-", 1)[1].rsplit("-", 1)[0]
        else:
            name = os.path.basename(path)[:-3]
        print(path, check(by_name[name], path, tmp), flush=True)
