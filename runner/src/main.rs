use std::time::Instant;

fn main() {
    let start_0 = Instant::now();

    run_libs();

    let stop = start_0.elapsed().as_micros() as f64 / 1000.0;
    println!("Total Duration: {:.3}ms", stop);
}

fn run_libs() {
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

    println!("DAY 11");
    let start = Instant::now();
    y20d11::main();
    let t = start.elapsed().as_micros() as f64 / 1000.0;
    println!("Duration: {:.3}ms", t);

    println!("DAY 12");
    let start = Instant::now();
    y20d12::main();
    let t = start.elapsed().as_micros() as f64 / 1000.0;
    println!("Duration: {:.3}ms", t);

    println!("DAY 13");
    let start = Instant::now();
    y20d13::main();
    let t = start.elapsed().as_micros() as f64 / 1000.0;
    println!("Duration: {:.3}ms", t);
}
