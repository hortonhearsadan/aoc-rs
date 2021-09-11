use std::time::Instant;
use y20d1;
use y20d10;
use y20d2;
use y20d3;
use y20d4;
use y20d5;
use y20d6;
use y20d7;
use y20d8;
use y20d9;

fn main() {
    let start_0 = Instant::now();

    println!("DAY 1");
    let start = Instant::now();
    y20d1::main();
    let t = start.elapsed().as_micros() as f64 / 1000.0;
    println!("Duration: {:.3}ms", t);

    println!("DAY 2");
    let start = Instant::now();
    y20d2::main();
    let t = start.elapsed().as_micros() as f64 / 1000.0;
    println!("Duration: {:.3}ms", t);

    println!("DAY 3");
    let start = Instant::now();
    y20d3::main();
    let t = start.elapsed().as_micros() as f64 / 1000.0;
    println!("Duration: {:.3}ms", t);

    println!("DAY 4");
    let start = Instant::now();
    y20d4::main();
    let t = start.elapsed().as_micros() as f64 / 1000.0;
    println!("Duration: {:.3}ms", t);

    println!("DAY 5");
    let start = Instant::now();
    y20d5::main();
    let t = start.elapsed().as_micros() as f64 / 1000.0;
    println!("Duration: {:.3}ms", t);

    println!("DAY 6");
    let start = Instant::now();
    y20d6::main();
    let t = start.elapsed().as_micros() as f64 / 1000.0;
    println!("Duration: {:.3}ms", t);

    println!("DAY 7");
    let start = Instant::now();
    y20d7::main();
    let t = start.elapsed().as_micros() as f64 / 1000.0;
    println!("Duration: {:.3}ms", t);

    println!("DAY 8");
    let start = Instant::now();
    y20d8::main();
    let t = start.elapsed().as_micros() as f64 / 1000.0;
    println!("Duration: {:.3}ms", t);

    println!("DAY 9");
    let start = Instant::now();
    y20d9::main();
    let t = start.elapsed().as_micros() as f64 / 1000.0;
    println!("Duration: {:.3}ms", t);

    println!("DAY 10");
    let start = Instant::now();
    y20d10::main();
    let t = start.elapsed().as_micros() as f64 / 1000.0;
    println!("Duration: {:.3}ms", t);

    let stop = start_0.elapsed().as_micros() as f64 / 1000.0;
    println!("Total Duration: {:.3}ms", stop);
}
