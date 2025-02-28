use anyhow::{anyhow, Ok, Result};
use std::sync::{
    atomic::{AtomicU8, Ordering},
    Arc, Mutex,
};

#[derive(Debug)]
struct Person {
    name: String,
    age: AtomicU8,
}

#[derive(Debug)]
struct PsersonMutex {
    name: String,
    age: Mutex<u8>,
}

fn main() -> Result<()> {
    let m = Mutex::new(5);
    {
        let mut num = m.lock().map_err(|e| anyhow!("Lock error: {:?}", e))?;
        *num = 6;
    }
    println!("m = {:?}", m);

    // 多个线程同时访问共享状态
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        let counter = counter.clone();
        let handle = std::thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result: {}", *counter.lock().unwrap());

    let p = Arc::new(Person::new(String::from("Alice"), 20));
    let mut handles = vec![];
    for _ in 0..10 {
        let p = p.clone();
        let handle = std::thread::spawn(move || {
            // 年龄增加 1
            let age = &p.age;
            age.fetch_add(1, Ordering::Relaxed);
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!(
        "Person.age: {:?}, Person.age: {:?}",
        p.name,
        p.age.load(Ordering::Relaxed)
    );

    // 通过使用 Mutex 来保护共享状态
    let p = Arc::new(PsersonMutex::new(String::from("Bob"), 20));
    let mut handles = vec![];
    for _ in 0..10 {
        let p = p.clone();
        let handle = std::thread::spawn(move || {
            // 年龄增加 1
            let age = &p.age;
            let mut num = age.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!(
        "Person_Mutex.age: {:?}, Person_Mutex.age: {:?}",
        p.name,
        p.age.lock().unwrap()
    );

    Ok(())
}

impl Person {
    fn new(name: String, age: u8) -> Self {
        Self {
            name,
            age: AtomicU8::new(age),
        }
    }
}

impl PsersonMutex {
    fn new(name: String, age: u8) -> Self {
        Self {
            name,
            age: Mutex::new(age),
        }
    }
}
