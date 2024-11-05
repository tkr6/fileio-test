use std::{env, fs, io, sync::mpsc, thread};

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();
    let num: i32 = match args.get(1) {
        Some(n) => n.parse().unwrap(),
        None => 1,
    };

    let (tx, rx) = mpsc::channel();

    for _ in 0..num - 1 {
        let tx1 = tx.clone();
        thread::spawn(move || {
            for _ in fs::read_to_string("big.txt").unwrap().lines() {
                tx1.send(1).unwrap();
            }
        });
    }
    thread::spawn(move || {
        for _ in fs::read_to_string("big.txt").unwrap().lines() {
            tx.send(1).unwrap();
        }
    });

    let mut count = 0;
    for received in rx {
        count += received;
    }
    println!("{}", count);

    Ok(())
}
