use std::process::Command;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    for i in (1..=5).into_iter() {
        println!("{}", i);
        let c = Command::new("bash")
            .arg("-c")
            .arg(format!("target/release/y20d{}", i))
            .output()
            .unwrap();
        println!("{:?}", String::from_utf8(c.stdout).unwrap());
    }
    let t = start.elapsed().as_micros() as f64 / 1000.0;
    println!("Duration: {:.3}ms", t);
}
