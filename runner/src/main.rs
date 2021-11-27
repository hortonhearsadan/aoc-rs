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
    y21d1::main();
    let t = start.elapsed().as_micros() as f64 / 1000.0;
    println!("Duration: {:.3}ms", t);

    println!("DAY 2");
    let start = Instant::now();
    y21d2::main();
    let t = start.elapsed().as_micros() as f64 / 1000.0;
    println!("Duration: {:.3}ms", t);

    println!("DAY 3");
    let start = Instant::now();
    y21d3::main();
    let t = start.elapsed().as_micros() as f64 / 1000.0;
    println!("Duration: {:.3}ms", t);

    println!("DAY 4");
    let start = Instant::now();
    y21d4::main();
    let t = start.elapsed().as_micros() as f64 / 1000.0;
    println!("Duration: {:.3}ms", t);

    println!("DAY 5");
    let start = Instant::now();
    y21d5::main();
    let t = start.elapsed().as_micros() as f64 / 1000.0;
    println!("Duration: {:.3}ms", t);

    println!("DAY 6");
    let start = Instant::now();
    y21d6::main();
    let t = start.elapsed().as_micros() as f64 / 1000.0;
    println!("Duration: {:.3}ms", t);

    println!("DAY 7");
    let start = Instant::now();
    y21d7::main();
    let t = start.elapsed().as_micros() as f64 / 1000.0;
    println!("Duration: {:.3}ms", t);

    println!("DAY 8");
    let start = Instant::now();
    y21d8::main();
    let t = start.elapsed().as_micros() as f64 / 1000.0;
    println!("Duration: {:.3}ms", t);

    println!("DAY 9");
    let start = Instant::now();
    y21d9::main();
    let t = start.elapsed().as_micros() as f64 / 1000.0;
    println!("Duration: {:.3}ms", t);

    println!("DAY 10");
    let start = Instant::now();
    y21d10::main();
    let t = start.elapsed().as_micros() as f64 / 1000.0;
    println!("Duration: {:.3}ms", t);

    println!("DAY 11");
    let start = Instant::now();
    y21d11::main();
    let t = start.elapsed().as_micros() as f64 / 1000.0;
    println!("Duration: {:.3}ms", t);

    println!("DAY 12");
    let start = Instant::now();
    y21d12::main();
    let t = start.elapsed().as_micros() as f64 / 1000.0;
    println!("Duration: {:.3}ms", t);

    println!("DAY 13");
    let start = Instant::now();
    y21d13::main();
    let t = start.elapsed().as_micros() as f64 / 1000.0;
    println!("Duration: {:.3}ms", t);

    println!("DAY 14");
    let start = Instant::now();
    y21d13::main();
    let t = start.elapsed().as_micros() as f64 / 1000.0;
    println!("Duration: {:.3}ms", t);

    println!("DAY 15");
    let start = Instant::now();
    y21d15::main();
    let t = start.elapsed().as_micros() as f64 / 1000.0;
    println!("Duration: {:.3}ms", t);

    println!("DAY 16");
    let start = Instant::now();
    y21d16::main();
    let t = start.elapsed().as_micros() as f64 / 1000.0;
    println!("Duration: {:.3}ms", t);

    println!("DAY 17");
    let start = Instant::now();
    y21d17::main();
    let t = start.elapsed().as_micros() as f64 / 1000.0;
    println!("Duration: {:.3}ms", t);

    println!("DAY 18");
    let start = Instant::now();
    y21d18::main();
    let t = start.elapsed().as_micros() as f64 / 1000.0;
    println!("Duration: {:.3}ms", t);

    println!("DAY 19");
    let start = Instant::now();
    y21d19::main();
    let t = start.elapsed().as_micros() as f64 / 1000.0;
    println!("Duration: {:.3}ms", t);

    println!("DAY 20");
    let start = Instant::now();
    y21d20::main();
    let t = start.elapsed().as_micros() as f64 / 1000.0;
    println!("Duration: {:.3}ms", t);

    println!("DAY 21");
    let start = Instant::now();
    y21d21::main();
    let t = start.elapsed().as_micros() as f64 / 1000.0;
    println!("Duration: {:.3}ms", t);

    println!("DAY 22");
    let start = Instant::now();
    y21d22::main();
    let t = start.elapsed().as_micros() as f64 / 1000.0;
    println!("Duration: {:.3}ms", t);

    println!("DAY 23");
    let start = Instant::now();
    y21d23::main();
    let t = start.elapsed().as_micros() as f64 / 1000.0;
    println!("Duration: {:.3}ms", t);

    println!("DAY 24");
    let start = Instant::now();
    y21d24::main();
    let t = start.elapsed().as_micros() as f64 / 1000.0;
    println!("Duration: {:.3}ms", t);

    println!("DAY 25");
    let start = Instant::now();
    y21d25::main();
    let t = start.elapsed().as_micros() as f64 / 1000.0;
    println!("Duration: {:.3}ms", t);
}
