"""Large test inputs, generated inside the test program instead of written as literals.

G(n, seed, lo, hi, mode, param) expands identically here, in Rust (one function) and in
Touchmark (one function per input, with every constant a literal so that its contract is proved
by linear arithmetic). Every value lies in [lo, hi]. Modes:
  0  uniform
  1  non-decreasing: steps drawn from [0, 2 * (hi - lo) / n], capped at hi
  2  non-increasing, the mirror of 1
  3  runs: a new uniform value with probability 1/param, else the previous one
  4  distinct: lo + 2048 * q + i, so (value - lo) % 2048 == i (needs n <= 2048, n <= hi - lo + 1)
  5  triangle wave: lo + |i % (2 * param) - param|, capped at hi
  6  uniform multiples of param
Draws use a 31-bit LCG and keep its top 15 bits, three per draw (45 random bits)."""

M31 = 2147483648


class G:
    def __init__(self, n, seed, lo, hi, mode=0, param=0):
        assert 1 <= n and lo <= hi and 1 <= seed < M31
        assert mode != 4 or (n <= 2048 and n <= hi - lo + 1)
        assert mode not in (3, 5, 6) or param >= 1
        self.n, self.seed, self.lo, self.hi, self.mode, self.param = n, seed, lo, hi, mode, param

    def __repr__(self):
        return f"G({self.n}, {self.seed}, {self.lo}, {self.hi}, {self.mode}, {self.param})"

    def step(self):
        return max(0, 2 * (self.hi - self.lo) // self.n) + 1

    def multiples(self):
        first = -(-self.lo // self.param)
        return first, self.hi // self.param - first + 1

    def values(self):
        n, lo, hi, p = self.n, self.lo, self.hi, self.param
        span = hi - lo + 1
        x = self.seed

        def draw():
            nonlocal x
            parts = []
            for _ in range(3):
                x = (x * 1103515245 + 12345) % M31
                parts.append(x // 65536)
            return parts[0] * 1073741824 + parts[1] * 32768 + parts[2]

        out = []
        if self.mode == 0:
            out = [lo + draw() % span for _ in range(n)]
        elif self.mode == 1:
            v = lo
            for _ in range(n):
                v = min(hi, v + draw() % self.step())
                out.append(v)
        elif self.mode == 2:
            v = hi
            for _ in range(n):
                v = max(lo, v - draw() % self.step())
                out.append(v)
        elif self.mode == 3:
            cur = lo
            for i in range(n):
                change, fresh = draw(), draw()
                if i == 0 or change % p == 0:
                    cur = lo + fresh % span
                out.append(cur)
        elif self.mode == 4:
            slots = (hi - lo) // 2048 + 1
            for i in range(n):
                q = draw() % slots
                if lo + 2048 * q + i > hi:
                    q = 0
                out.append(lo + 2048 * q + i)
        elif self.mode == 5:
            out = [min(hi, lo + abs(i % (2 * p) - p)) for i in range(n)]
        elif self.mode == 6:
            first, count = self.multiples()
            out = [p * (first + draw() % count) for _ in range(n)]
        return out


RUST_GEN = """
fn test_input(n: usize, seed: i64, lo: i64, hi: i64, mode: i64, p: i64) -> Vec<i64> {
    let mut x = seed;
    let mut draw = || {
        let mut r = 0i64;
        for k in 0..3 {
            x = (x * 1103515245 + 12345) % 2147483648;
            r += (x / 65536) << (30 - 15 * k);
        }
        r
    };
    let span = hi - lo + 1;
    let step = (2 * (hi - lo) / n as i64).max(0) + 1;
    let mut out = Vec::with_capacity(n);
    let mut v = if mode == 2 { hi } else { lo };
    for i in 0..n as i64 {
        match mode {
            0 => v = lo + draw() % span,
            1 => v = hi.min(v + draw() % step),
            2 => v = lo.max(v - draw() % step),
            3 => {
                let (change, fresh) = (draw(), draw());
                if i == 0 || change % p == 0 {
                    v = lo + fresh % span;
                }
            }
            4 => {
                let mut q = draw() % ((hi - lo) / 2048 + 1);
                if lo + 2048 * q + i > hi {
                    q = 0;
                }
                v = lo + 2048 * q + i;
            }
            5 => v = hi.min(lo + (i % (2 * p) - p).abs()),
            _ => {
                let first = (lo + p - 1).div_euclid(p);
                v = p * (first + draw() % (hi.div_euclid(p) - first + 1));
            }
        }
        out.push(v);
    }
    out
}
"""


def rust_call(g):
    return f"test_input({g.n}, {g.seed}, {g.lo}, {g.hi}, {g.mode}, {g.param})"


DRAW = """    x = (x * 1103515245 + 12345) % 2147483648
    let {v}1 = x / 65536
    x = (x * 1103515245 + 12345) % 2147483648
    let {v}2 = x / 65536
    x = (x * 1103515245 + 12345) % 2147483648
    let {v} = {v}1 * 1073741824 + {v}2 * 32768 + x / 65536
"""


def tmk_fn(g, name):
    """A Touchmark function with no parameters returning the input, its range and length proved.
    Public, so the compiler takes its contract as written instead of inferring one."""
    n, lo, hi, p = g.n, g.lo, g.hi, g.param
    span = hi - lo + 1
    head = (f"pub fn {name}() -> [int]\n  ensures result.len == {n}\n"
            f"  ensures forall t in 0..result.len: {lo} <= result[t] && result[t] <= {hi}\n")
    inv = ["0 <= x && x < 2147483648", "a.len == i", f"forall t in 0..a.len: {lo} <= a[t] && a[t] <= {hi}"]
    pre = ""
    if g.mode == 0:
        body = DRAW.format(v="r") + f"    a.push({lo} + r % {span})\n"
    elif g.mode in (1, 2):
        pre = f"  var v = {lo if g.mode == 1 else hi}\n"
        inv.append(f"{lo} <= v && v <= {hi}")
        if g.mode == 1:
            body = DRAW.format(v="r") + f"    v = v + r % {g.step()}\n    if v > {hi} {{ v = {hi} }}\n    a.push(v)\n"
        else:
            body = DRAW.format(v="r") + f"    v = v - r % {g.step()}\n    if v < {lo} {{ v = {lo} }}\n    a.push(v)\n"
    elif g.mode == 3:
        pre = f"  var cur = {lo}\n"
        inv.append(f"{lo} <= cur && cur <= {hi}")
        body = (DRAW.format(v="change") + DRAW.format(v="fresh") +
                f"    if i == 0 || change % {p} == 0 {{ cur = {lo} + fresh % {span} }}\n    a.push(cur)\n")
    elif g.mode == 4:
        head += "  ensures forall s in 0..result.len: forall t in 0..result.len: s != t ==> result[s] != result[t]\n"
        inv.append(f"forall t in 0..a.len: (a[t] - ({lo})) % 2048 == t")
        body = (DRAW.format(v="r") + f"    var q = r % {(hi - lo) // 2048 + 1}\n"
                f"    if {lo} + 2048 * q + i > {hi} {{ q = 0 }}\n    a.push({lo} + 2048 * q + i)\n")
    elif g.mode == 5:
        body = (f"    var v = i % {2 * p} - {p}\n    if v < 0 {{ v = -v }}\n    v = {lo} + v\n"
                f"    if v > {hi} {{ v = {hi} }}\n    a.push(v)\n")
    else:
        first, count = g.multiples()
        body = DRAW.format(v="r") + f"    a.push({p} * ({first} + r % {count}))\n"
    invs = "".join(f"    invariant {s}\n" for s in inv)
    return (head + "{\n  var a: [int] = []\n  var x = " + str(g.seed) + "\n" + pre +
            f"  for i in 0..{n}\n" + invs + "  {\n" + body + "  }\n  a\n}\n")
