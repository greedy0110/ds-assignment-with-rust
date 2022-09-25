use as1::arraryutility::create_random_array_f64;
use as2::arrayofaverage::array_of_average;
use bench::bench;

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