use std::time::Instant;

fn main() {
    bench(|| println!("hello main"))
}

fn bench<F>(f: F)
where
    F: FnOnce(),
{
    let before = Instant::now();

    f();

    let elapsed = before.elapsed();
    println!("Elapsed : {:.2?}", elapsed);
}
