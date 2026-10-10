"""Post-hoc: a stricter access audit than usage.py, which compares absolute paths by string prefix
without normalizing them, so '/work/<run>/../<other run>/' or a relative climb out of the run's
directory would pass unflagged. This one normalizes paths, flags any climb above the run's
directory, home references and permission changes, and scans tool results too. Needs the
transcripts. Not part of the pre-registered scoring."""
import json, os, re

HERE = os.path.dirname(os.path.abspath(__file__))
SUB = "/Users/air/.claude/projects/-Volumes-part-one-Coding-Projects-lang-as/81c3387e-5192-4c2d-aca4-3fa6920cf370/subagents"
runs = json.load(open(os.path.join(HERE, "..", "private", "runs.json")))
ids = {r["id"] for r in runs}
PRIVATE = ("touchmark-eval-r2", "touchmark-round2", "expected.json", "refs.py", "eval/round2", "eval/private", "lang.as")
SHELL = (r"\.\./\.\.", r"\bcd\s+\.\.", r"~/", r"\$\{?HOME", r"\bchmod\b", r"\bsudo\b", r"\b(find|ls)\s+/(\s|$)", r"\bmdfind\b")


def flags(run):
    wd = run["dir"]
    found = set()
    for line in open(os.path.join(SUB, f"agent-{run['agent']}.jsonl")):
        for c in (json.loads(line).get("message") or {}).get("content") or []:
            if not isinstance(c, dict):
                continue
            if c.get("type") == "tool_use":
                for k, v in c["input"].items():
                    if k == "description" or not isinstance(v, str):
                        continue
                    for p in re.findall(r"(?:^|[\s\"'=(<>|;&])(/[\w.-][\w./-]*)", v):
                        n = os.path.normpath(p)
                        if not (n == wd or n.startswith(wd + "/") or n == "/work/touchmark/llms.txt"
                                or n.startswith(("/dev/", "/usr/bin/", "/bin/"))):
                            found.add(f"path {n}")
                    # a single '../' from a subdirectory back to the run's own files is fine
                    found.update(f"shell {pat}" for pat in SHELL if re.search(pat, v))
            elif c.get("type") == "tool_result":
                out = c["content"] if isinstance(c["content"], str) else " ".join(x.get("text", "") for x in c["content"] if isinstance(x, dict))
                found.update(f"result names {o}" for o in ids if o != run["id"] and o in out)
                found.update(f"result names {m}" for m in PRIVATE if m in out)
    return sorted(found)


for run in runs:
    if run.get("agent"):
        print(run["id"], flags(run) or "clean")
