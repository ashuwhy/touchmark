// State before car i is placed, measured from car i's release time:
// fuel already drawn from dispenser 0 (dispenser 1's draw follows from the
// prefix sum) and how long each dispenser stays busy. Release times never go
// backwards, so every earlier car has started and drawn its fuel by then, and
// no busy time exceeds the largest demand (20).
const MAX_BUSY: usize = 20;
const UNREACHED: i64 = i64::MAX;

pub fn min_max_waiting_time(demand: &[i64], fuel: &[i64]) -> i64 {
    let cap0 = fuel[0] as usize;
    let cap1 = fuel[1] as usize;
    let span = MAX_BUSY + 1;
    let idx = |drawn0: usize, busy0: usize, busy1: usize| (drawn0 * span + busy0) * span + busy1;
    let size = (cap0 + 1) * span * span;

    let mut worst = vec![UNREACHED; size];
    worst[idx(0, 0, 0)] = 0;

    let mut best = -1;
    let mut drawn_total = 0usize;

    for (i, &d) in demand.iter().enumerate() {
        let d = d as usize;
        let mut next = vec![UNREACHED; size];
        let mut stuck = UNREACHED;
        let mut moved = false;

        for drawn0 in 0..=cap0.min(drawn_total) {
            let drawn1 = drawn_total - drawn0;
            if drawn1 > cap1 {
                continue;
            }
            for busy0 in 0..=MAX_BUSY {
                for busy1 in 0..=MAX_BUSY {
                    let w = worst[idx(drawn0, busy0, busy1)];
                    if w == UNREACHED {
                        continue;
                    }
                    let fits0 = drawn0 + d <= cap0;
                    let fits1 = drawn1 + d <= cap1;
                    if !fits0 && !fits1 {
                        stuck = stuck.min(w);
                        continue;
                    }
                    if fits0 {
                        let cell = &mut next[idx(drawn0 + d, d, busy1.saturating_sub(busy0))];
                        *cell = (*cell).min(w.max(busy0 as i64));
                        moved = true;
                    }
                    if fits1 {
                        let cell = &mut next[idx(drawn0, busy0.saturating_sub(busy1), d)];
                        *cell = (*cell).min(w.max(busy1 as i64));
                        moved = true;
                    }
                }
            }
        }

        // Some branch stopped at car i; a later stop serves more cars and
        // overrides it, so only the furthest stopping point survives.
        if stuck != UNREACHED {
            best = if i == 0 { -1 } else { stuck };
        }
        if !moved {
            return best;
        }
        worst = next;
        drawn_total += d;
    }

    // Every branch still alive served all n cars.
    worst.into_iter().min().unwrap()
}
