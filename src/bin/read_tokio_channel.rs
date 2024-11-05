use std::{env, fs, io};

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();
    let num: i32 = match args.get(1) {
        Some(n) => n.parse().unwrap(),
        None => 1,
    };

    let (tx, mut rx) = tokio::sync::mpsc::channel(32);

    for _ in 0..num - 1 {
        let tx1 = tx.clone();
        tokio::spawn(async move {
            for _ in fs::read_to_string("big.txt").unwrap().lines() {
                tx1.send(1).await.unwrap();
            }
        });
    }
    tokio::spawn(async move {
        for _ in fs::read_to_string("big.txt").unwrap().lines() {
            tx.send(1).await.unwrap();
        }
    });

    let mut count = 0;
    while let Some(message) = rx.recv().await {
        count += message;
    }
    println!("{}", count);

    Ok(())
}
