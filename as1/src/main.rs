use std::time::Instant;
use ds::{maxsort::*, arraryutility::create_random_array};

fn main() {
    bench_maxsort(10);
    bench_maxsort(100);
    bench_maxsort(1000);
    bench_maxsort(10000);
}

fn bench_maxsort(n: usize) {
    println!("{}", "=".repeat(10));
    println!("bench maxsort start size = {}", n);
    bench("shift", || maxsort_shift_size(n));
    bench("swap", || maxsort_swap_size(n));
    println!("bench maxsort end");
    println!("{}", "=".repeat(10));
}

fn maxsort_shift_size(n: usize) {
    let mut v = create_random_array(n, i32::MIN, i32::MAX);
    max_sort_shift(&mut v);
}

fn maxsort_swap_size(n: usize) {
    let mut v = create_random_array(n, i32::MIN, i32::MAX);
    max_sort_swap(&mut v);
}

fn bench<F>(test_name: &str, f: F)
where
    F: FnOnce(),
{
    let before = Instant::now();

    f();

    let elapsed = before.elapsed();
    println!("{} Elapsed : {:.2?}", test_name, elapsed);
}