use as1::arraryutility::create_random_array_f64;
use as2::arrayofaverage::array_of_average;
use std::time::Instant;

// TODO: bench 모듈 분리 (별도 PR)

fn main() {
    bench_aoa(10);
    bench_aoa(100);
    bench_aoa(1000);
    bench_aoa(10000);
}

fn bench_aoa(n: usize) {
    println!("{}", "=".repeat(10));
    println!("bench array of average start size = {}", n);
    bench("aoa", || bench_array_of_average(n));
    println!("bench array of average end");
    println!("{}", "=".repeat(10));
}

fn bench_array_of_average(n: usize) {
    array_of_average(&create_random_array_f64(n, 0.0, 10.0));
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
