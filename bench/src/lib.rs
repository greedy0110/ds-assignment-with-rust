//! Bench
//! `bench` is for performance checking.

use std::time::Instant;

pub fn bench<F>(test_name: &str, f: F)
where
    F: Fn(),
{
    let before = Instant::now();

    f();

    let elapsed = before.elapsed();
    println!("{} Elapsed : {:.2?}", test_name, elapsed);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bench00() {
        bench("a", || {});
    }

    // #[test]
    // fn batch_bench00() {
    //     batch_bench(("a", || { /* do something 1 */}), ("b", || { /* do something 2 */}), ("c", || { /* do something 3 */}));
    // }
}
