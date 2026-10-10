"""Score the gold solutions: score_gold.py <lang> [task...]"""
import json, sys, time
import harness as h

lang = sys.argv[1]
names = sys.argv[2:] or [t["name"] for t in h.TASKS]
for name in names:
    t0 = time.time()
    s = h.score(h.BY_NAME[name], lang, open(f"gold/{name}.{h.EXT[lang]}").read())
    secs = [x for x in s["seconds"] if x is not None]
    bad = [(k, r) for k, r in enumerate(s["results"]) if r != "pass"]
    print(f"{name}: {s['passed']}/{s['total']} max test {max(secs) if secs else None}s, scored in {time.time() - t0:.0f}s",
          bad[:6], s["build_log"][-800:] if s["build_log"] else "", flush=True)
