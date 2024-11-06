use std::{
    env, fs, io,
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
};

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();
    let num: i32 = match args.get(1) {
        Some(n) => n.parse().unwrap(),
        None => 1,
    };
    let count = Arc::new(Mutex::new(0));
    let mut handles: Vec<JoinHandle<()>> = Vec::new();
    for _ in 0..num {
        let count = Arc::clone(&count);
        let handle = thread::spawn(move || {
            for _ in fs::read_to_string("big.txt").unwrap().lines() {
                let mut num = count.lock().unwrap();
                *num += 1;
            }
        });
        handles.push(handle);
    }
    for ele in handles {
        ele.join().unwrap();
    }
    println!("{}", count.lock().unwrap());

    Ok(())
}
