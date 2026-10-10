"""Round 2 numbers from results.json, as the protocol defines them: report.py [--markdown]"""
import json, math, os, sys
from collections import defaultdict

from scipy.stats import fisher_exact, hypergeom

here = os.path.dirname(os.path.abspath(__file__))
runs = json.load(open(os.path.join(here, "runs.json")))
res = json.load(open(os.path.join(here, "results.json")))
LANGS = ("touchmark", "rust")


def defective(r):
    return any(x != "pass" for x in r["results"])


def wrong(r):
    return any(x not in ("pass", "timeout") for x in r["results"])


def stratified_p(rows):
    """Exact conditional test of a common odds ratio of 1, tasks as strata: Touchmark's defective
    count per task is hypergeometric given the task's margins; the total is their convolution."""
    dist = [1.0]
    obs = 0
    for n_t, d_t, n_r, d_r in rows:
        m, n = d_t + d_r, n_t + n_r
        pmf = [hypergeom(n, m, n_t).pmf(x) for x in range(n_t + 1)]
        new = [0.0] * (len(dist) + n_t)
        for i, a in enumerate(dist):
            for x, b in enumerate(pmf):
                new[i + x] += a * b
        dist = new
        obs += d_t
    return min(1.0, sum(p for p in dist if p <= dist[obs] * (1 + 1e-9)))


def counted(trials=(1, 2, 3)):
    return {k: v for k, v in res.items() if v["trial"] in trials}


def main():
    c = counted()
    by = defaultdict(list)
    for k, r in c.items():
        by[(r["task"], r["lang"])].append(r)
    tasks = sorted({r["task"] for r in c.values()}, key=lambda t: [x["task"] for x in runs].index(t))
    n = {l: sum(1 for r in c.values() if r["lang"] == l) for l in LANGS}
    d = {l: sum(defective(r) for r in c.values() if r["lang"] == l) for l in LANGS}
    w = {l: sum(wrong(r) for r in c.values() if r["lang"] == l) for l in LANGS}
    print(f"counted runs: touchmark {n['touchmark']}, rust {n['rust']}")
    if not n["touchmark"]:
        calibration(c)
        return
    if not n["rust"]:
        return
    p = fisher_exact([[d["touchmark"], n["touchmark"] - d["touchmark"]], [d["rust"], n["rust"] - d["rust"]]])[1]
    rows = [(len(by[(t, "touchmark")]), sum(map(defective, by[(t, "touchmark")])),
             len(by[(t, "rust")]), sum(map(defective, by[(t, "rust")]))) for t in tasks]
    ps = stratified_p(rows)
    pw = fisher_exact([[w["touchmark"], n["touchmark"] - w["touchmark"]], [w["rust"], n["rust"] - w["rust"]]])[1]
    print(f"defective: touchmark {d['touchmark']}/{n['touchmark']}, rust {d['rust']}/{n['rust']}, Fisher p = {p:.4f}, stratified p = {ps:.4f}")
    print(f"correctness only: touchmark {w['touchmark']}/{n['touchmark']}, rust {w['rust']}/{n['rust']}, Fisher p = {pw:.4f}")
    tests = {l: (sum(r["passed"] for r in c.values() if r["lang"] == l), sum(r["total"] for r in c.values() if r["lang"] == l)) for l in LANGS}
    print("tests passed:", {l: f"{a}/{b} ({100 * a / b:.1f}%)" for l, (a, b) in tests.items()})
    s = {l: {m: sum(r[m] for r in c.values() if r["lang"] == l) for m in ("T_ctx", "T_proc", "T_work", "tool_calls")} for l in LANGS}
    for m in ("T_ctx", "T_proc", "T_work", "tool_calls"):
        print(f"{m}: touchmark {s['touchmark'][m] / n['touchmark']:.0f}, rust {s['rust'][m] / n['rust']:.0f} per run, ratio {s['touchmark'][m] / s['rust'][m]:.2f}")
    secs = {l: sorted(r.get("seconds_run", r.get("seconds")) for r in c.values() if r["lang"] == l and isinstance(r.get("seconds_run", r.get("seconds")), (int, float))) for l in LANGS}
    for l in LANGS:
        if secs[l]:
            print(f"wall seconds {l}: median {secs[l][len(secs[l]) // 2]}")
    fewer = d["touchmark"] * 2 <= d["rust"] and p < 0.05
    worse = d["touchmark"] > d["rust"] and p < 0.05
    print("decision, defects:", "Touchmark fewer" if fewer else "Touchmark worse" if worse else "no difference shown")
    print("decision, tokens: T_work ratio", f"{s['touchmark']['T_work'] / s['rust']['T_work']:.2f}",
          "within 2.0" if s["touchmark"]["T_work"] <= 2 * s["rust"]["T_work"] else "above 2.0")
    print()
    print("| Task | Touchmark defective | Rust defective | Touchmark tests | Rust tests | Touchmark T_work | Rust T_work |")
    print("|---|---|---|---|---|---|---|")
    for t in tasks:
        cells = []
        for l in LANGS:
            rs = by[(t, l)]
            cells.append(f"{sum(map(defective, rs))}/{len(rs)}")
        for l in LANGS:
            rs = by[(t, l)]
            cells.append(f"{sum(r['passed'] for r in rs)}/{sum(r['total'] for r in rs)}")
        for l in LANGS:
            rs = by[(t, l)]
            cells.append(f"{sum(r['T_work'] for r in rs) / max(1, len(rs)):,.0f}")
        print(f"| `{t}` | " + " | ".join(cells) + " |")
    print()
    for k in sorted(c, key=lambda k: [x["id"] for x in runs].index(k)):
        r = c[k]
        bad = {x: r["results"].count(x) for x in set(r["results"]) if x != "pass"}
        extra = f" standalone {r['standalone_check']}" if "standalone_check" in r else ""
        print(f"{k}: {r['passed']}/{r['total']} {bad or ''}{extra} T_work {r['T_work']} calls {r['tool_calls']} outside {r['outside_paths']}")


def run_row(k, r):
    # with_error is left out: in Rust it also counts rustc failing to make a temp dir beside -o /dev/null
    return (f"| `{k}` | {r['passed']}/{r['total']} | {r['T_work']:,} | {r['T_ctx']:,} | {r['T_proc']:,} | {r['tool_calls']} | "
            f"{r['causes']['compiler']['runs']} | {r['wall_s']:.0f} | {r.get('solution_tokens', 0)} |")


def calibration(c):
    """The round stopped after the Rust calibration (protocol, runs, step 2)."""
    rs = [c[k] for k in sorted(c, key=lambda k: [x["id"] for x in runs].index(k))]
    d = sum(map(defective, rs))
    print(f"Rust calibration: {d}/{len(rs)} defective; stop rule (0 or 1 defective) {'fired' if d <= 1 else 'not fired'}")
    print(f"tests passed: {sum(r['passed'] for r in rs)}/{sum(r['total'] for r in rs)}")
    for m in ("T_work", "T_ctx", "T_proc", "solution_tokens"):
        print(f"{m} mean {sum(r.get(m, 0) for r in rs) / len(rs):,.0f}")
    print(f"tool calls mean {sum(r['tool_calls'] for r in rs) / len(rs):.1f}, "
          f"compiler runs mean {sum(r['causes']['compiler']['runs'] for r in rs) / len(rs):.1f}")
    walls = sorted(r["wall_s"] for r in rs)
    print(f"wall seconds median {walls[len(walls) // 2]:.0f}, range {walls[0]:.0f} to {walls[-1]:.0f}")
    print(f"slowest hidden test {max(max(r['seconds']) for r in rs):.3f} s")
    print()
    print(HEAD)
    print("|---|---|---|---|---|---|---|---|---|")
    for k in sorted(c, key=lambda k: [x["id"] for x in runs].index(k)):
        print(run_row(k, c[k]))


HEAD = "| Run | Tests | T_work | T_ctx | T_proc | Tool calls | Compiler runs | Wall s | Solution tokens |"


def pilot():
    p = counted(trials=(0,))
    print()
    print("Pilot, not counted:")
    print(HEAD)
    print("|---|---|---|---|---|---|---|---|---|")
    for k in sorted(p):
        print(run_row(k, p[k]))
        print(f"    tokens by cause: {p[k]['causes']['tokens']}")


if __name__ == "__main__":
    main()
    pilot()
