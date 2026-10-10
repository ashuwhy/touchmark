"""Add what the transcripts alone can give to results.json, so report.py works without them:
wall time (first to last transcript entry), the final solution's size in o200k tokens and the
per-cause token breakdown. Run after score_all.py, while the transcripts exist."""
import datetime, json, os
import tiktoken
import harness as h
from usage import causes

SUB = "/Users/air/.claude/projects/-Volumes-part-one-Coding-Projects-lang-as/81c3387e-5192-4c2d-aca4-3fa6920cf370/subagents"
enc = tiktoken.get_encoding("o200k_base")
runs = {r["id"]: r for r in json.load(open("runs.json"))}
res = json.load(open("results.json"))


def stamp(entry):
    return datetime.datetime.fromisoformat(entry["timestamp"].replace("Z", "+00:00"))


for k, rec in res.items():
    r = runs[k]
    path = os.path.join(SUB, f"agent-{r['agent']}.jsonl")
    entries = [e for e in map(json.loads, open(path)) if "timestamp" in e]
    rec["wall_s"] = round((stamp(entries[-1]) - stamp(entries[0])).total_seconds(), 1)
    sol = os.path.join(r["dir"], "solution." + h.EXT[r["lang"]])
    if os.path.exists(sol):
        rec["solution_tokens"] = len(enc.encode(open(sol).read(), disallowed_special=()))
    rec["causes"] = causes(path)
    print(k, rec["wall_s"], rec.get("solution_tokens"), rec["causes"]["compiler"])

json.dump(res, open("results.json", "w"), indent=1)
