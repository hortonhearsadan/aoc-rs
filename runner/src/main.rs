use std::env;
use std::time::Instant;

const FNS: [fn(); 25] = [
    y21d1::main,
    y21d2::main,
    y21d3::main,
    y21d4::main,
    y21d5::main,
    y21d6::main,
    y21d7::main,
    y21d8::main,
    y21d9::main,
    y21d10::main,
    y21d11::main,
    y21d12::main,
    y21d13::main,
    y21d14::main,
    y21d15::main,
    y21d16::main,
    y21d17::main,
    y21d18::main,
    y21d19::main,
    y21d20::main,
    y21d21::main,
    y21d22::main,
    y21d23::main,
    y21d24::main,
    y21d25::main,
];

fn main() {
    let start_0 = Instant::now();

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        run_libs();
    } else {
        let day = args[1].parse::<usize>().unwrap();
        run_lib(FNS[day - 1], day)
    }

    let stop = start_0.elapsed().as_micros() as f64 / 1000.0;
    println!("Total Duration: {:.3}ms", stop);
}

fn run_libs() {
    for (i, f) in FNS.iter().enumerate() {
        run_lib(f, i + 1)
    }
}

fn run_lib(f: impl Fn(), i: usize) {
    println!("DAY {}", i);
    let start = Instant::now();
    f();
    let t = start.elapsed().as_micros() as f64 / 1000.0;
    println!("Duration: {:.3}ms", t);
}
