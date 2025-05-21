#![feature(coroutines, coroutine_trait, stmt_expr_attributes)]

use rand::Rng;
use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::ops::{Coroutine, CoroutineState};
use std::pin::Pin;
use std::time::Instant;

struct WriteCoroutine {
    pub file_handle: File,
}

impl WriteCoroutine {
    fn new() -> Self {
        Self {
            file_handle: OpenOptions::new()
                .create(true)
                .append(true)
                .open("numbers.txt")
                .unwrap(),
        }
    }
}

impl Coroutine<i32> for WriteCoroutine {
    type Yield = ();
    type Return = ();
    fn resume(mut self: Pin<&mut Self>, arg: i32) -> CoroutineState<Self::Yield, Self::Return> {
        writeln!(self.file_handle, "{}", arg).unwrap();
        CoroutineState::Yielded(())
    }
}

#[allow(dead_code)]
fn append_number_to_file(n: i32) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("numbers.txt")?;
    writeln!(file, "{}", n)?;
    Ok(())
}

fn main() {
    let mut rng = rand::rng();
    let numbers: Vec<i32> = (0..200000).map(|_| rng.random()).collect();
    let mut coroutine = WriteCoroutine::new();

    let start = Instant::now();
    for &number in &numbers {
        // if let Err(e) = append_number_to_file(number) {
        //     eprintln!("Error writing to file: {}", e);
        // }
        Pin::new(&mut coroutine).resume(number);
    }
    let duration = start.elapsed();
    println!("Time elapsed in file operations is {:?}", duration);
}
