use std::{
    env, fs, io,
    sync::{
        atomic::{AtomicI32, Ordering},
        Arc,
    },
};

use tokio::task::JoinHandle;

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();
    let num: i32 = match args.get(1) {
        Some(n) => n.parse().unwrap(),
        None => 1,
    };
    let count = Arc::new(AtomicI32::new(0));
    let mut join_handles: Vec<JoinHandle<()>> = Vec::new();
    for _ in 0..num {
        let count = Arc::clone(&count);
        let handle = tokio::spawn(async move {
            for _ in fs::read_to_string("big.txt").unwrap().lines() {
                count.fetch_add(1, Ordering::SeqCst);
            }
        });
        join_handles.push(handle);
    }
    for ele in join_handles {
        ele.await?;
    }
    println!("{}", count.load(Ordering::SeqCst));

    Ok(())
}
