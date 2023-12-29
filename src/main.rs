use std::process::Command;
use std::time::Duration;

const SAMPLES: u64 = 10;

fn main() {
    println!("| Day | Time |");
    println!("| :---: | :---: |");
    let mut total = 0;
    for day in 1..=25 {
        let elapsed = run(day, SAMPLES);
        total += elapsed;
        println!(
            "| [Day {day}](./src/bin/{day:0>2}.rs) | {:.2?} |",
            Duration::from_nanos(elapsed)
        )
    }
    println!("**Total: {:.2?}**", Duration::from_nanos(total));
}

fn run(day: u32, samples: u64) -> u64 {
    let day = format!("{day:0>2}");
    let args = ["run", "--bin", &day, "--release", "--quiet"];
    let mut elapsed = 0;
    for _ in 0..samples {
        let cmd = Command::new("cargo").args(args).output().unwrap();
        let output = String::from_utf8(cmd.stdout.clone()).unwrap();
        let time = output
            .lines()
            .last()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap();
        let (time, unit) = time.split_at(time.chars().position(char::is_alphabetic).unwrap());
        let time = time.parse::<f64>().unwrap();

        let mul = match unit {
            "s" => 1_000_000_000.0,
            "ms" => 1_000_000.0,
            "µs" => 1_000.0,
            "ns" => 1.0,
            _ => unreachable!()
        };
        elapsed += (time * mul) as u64;
    }
    elapsed / samples
}
