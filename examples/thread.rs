use anyhow::{anyhow, Result};
use std::{sync::mpsc, thread, time::Duration};

const NUM_PRODUCERS: usize = 4;

#[allow(dead_code)]
#[derive(Debug)]
struct Msg {
    idx: usize,
    val: usize,
}

fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel();

    // 创建 producer 线程
    for i in 0..NUM_PRODUCERS {
        let tx = tx.clone();
        thread::spawn(move || producer(i, tx));
    }

    // 释放 tx，否则 rx 无法停止
    drop(tx);

    // 创建 consumer 线程
    let consumer = thread::spawn(move || {
        let mut ret = 0;
        for msg in rx {
            println!("consumer: {:?}", msg);
            ret += msg.val;
        }
        println!("consumer exit");
        ret
    });

    let ret = consumer
        .join()
        .map_err(|e| anyhow!("Thread join error: {:?}", e))?;

    println!("ret: {}", ret);

    Ok(())
}

fn producer(idx: usize, tx: mpsc::Sender<Msg>) -> Result<()> {
    loop {
        let value = rand::random::<u8>() as usize;
        tx.send(Msg::new(idx, value))?;
        let sleep_time = rand::random::<u8>() as u64 * 10;
        thread::sleep(Duration::from_millis(sleep_time));
        // random exit the producer
        if rand::random::<u8>() % 5 == 0 {
            println!("producer {} exit", idx);
            break;
        }
    }
    // more things to do
    Ok(())
}

impl Msg {
    fn new(idx: usize, val: usize) -> Self {
        Self { idx, val }
    }
}
