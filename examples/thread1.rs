use anyhow::{anyhow, Result};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() -> Result<()> {
    let handle = thread::spawn(move || {
        for i in 0..10 {
            println!("Hello from thread {}!", i);
            thread::sleep(Duration::from_millis(10));
        }
    });

    handle
        .join()
        .map_err(|e| anyhow!("Thread join error: {:?}", e))?;

    for i in 0..10 {
        println!("Hello from main thread {}!", i);
        thread::sleep(Duration::from_millis(10));
    }

    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        // 这里的 move 是为了将 v 移动到新线程中
        println!("Here's a vector: {:?}", v);
    });

    handle
        .join()
        .map_err(|e| anyhow!("Thread join error: {:?}", e))?;

    // drop(v); // 由于 v 的所有权已经被移动到新线程中，在主线程中无法再使用 v

    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            thread::sleep(Duration::from_millis(1000));
            tx.send(val).unwrap();
        }
    });
    // recv() 会阻塞当前线程，直到接收到消息
    // let received = rx.recv()?;
    for received in rx {
        println!("Got: {}", received);
    }
    Ok(())
}
