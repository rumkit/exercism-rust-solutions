use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    let mut out = HashSet::new();

    for m in 2..=(sum / 2).isqrt() {
        for n in 1..m {
            let base = 2 * m * (m + n);

            if sum.is_multiple_of(base) {
                let k = sum / base;

                let a = k * (m * m - n * n);
                let b = k * (2 * m * n);
                let c = k * (m * m + n * n);

                let mut t = [a, b, c];
                t.sort_unstable();
                out.insert(t);
            }
        }
    }

    out
}
