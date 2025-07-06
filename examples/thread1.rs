use anyhow::Result;
use std::time::Duration;
use std::{sync::mpsc, thread};

const NUM_PRODUCERS: usize = 4;

#[derive(Debug)]
struct Msg {
    idx: usize,
    value: i32,
}
fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel();

    //create producers
    for i in 0..NUM_PRODUCERS {
        let tx = tx.clone();
        thread::spawn(move || producer(i, tx));
    }
    drop(tx);

    //create consumer
    let consumer = thread::spawn(move || {
        for msg in rx {
            println!("consumer {:?}", msg);
        }
        println!("consumer done");
        let s = "secret".to_string();
        s
    });

   let secret = consumer.join().unwrap();
    println!("secret: {}", secret);
    Ok(())
}

fn producer(idx: usize, tx: mpsc::Sender<Msg>) -> Result<()> {
    loop {
        let value = rand::random::<i32>();
        tx.send(Msg::new(idx, value))?;
        let sleep_time = rand::random::<u8>() as u64 * 10;
        thread::sleep(Duration::from_millis(sleep_time));
        if rand::random::<u8>() % 5 == 0 {
            println!("producer {} is done", idx);
            break;
        }
    }
    Ok(())
}

impl Msg {
    fn new(idx: usize, value: i32) -> Self {
        Self { idx, value }
    }
}
