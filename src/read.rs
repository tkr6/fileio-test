use std::{env, fs, io};

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();
    let num: i32 = match args.get(1) {
        Some(n) => n.parse().unwrap(),
        None => 1,
    };
    let mut count = 0;
    for _ in 0..num {
        for _ in fs::read_to_string("big.txt")?.lines() {
            count += 1;
        }
    }
    println!("{}", count);

    Ok(())
}
