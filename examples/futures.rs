use std::future::Future;
use std::pin::Pin;
use std::ptr;
use std::task::{Context, Poll};
use std::time::Duration;

use tokio::time::timeout;

async fn slow_task() -> &'static str {
    tokio::time::sleep(Duration::from_secs(10)).await;
    "Slow Task completed"
}

struct SelfReferential {
    data: String,
    self_pointer: *const String,
}

impl SelfReferential {
    fn new(data: String) -> SelfReferential {
        let mut sr = SelfReferential {
            data,
            self_pointer: ptr::null(),
        };
        sr.self_pointer = &sr.data as *const String;
        sr
    }
    fn print(&self) {
        unsafe {
            println!("{}", *self.self_pointer);
        }
    }
}

struct CounterFuture {
    count: u32,
}

impl Future for CounterFuture {
    type Output = u32;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.count += 1;
        println!("polling with result: {}", self.count);
        std::thread::sleep(Duration::from_secs(1));
        if self.count < 3 {
            cx.waker().wake_by_ref();
            Poll::Pending
        } else {
            Poll::Ready(self.count)
        }
    }
}

#[tokio::main]
async fn main() {
    // let counter_one = CounterFuture { count: 0 };
    // let counter_two = CounterFuture { count: 0 };
    // let handle_one: JoinHandle<u32> = tokio::task::spawn(async move { counter_one.await });
    // let handle_two: JoinHandle<u32> = tokio::task::spawn(async move { counter_two.await });
    // tokio::join!(handle_one, handle_two);

    // Pin
    let mut first = SelfReferential::new("first".to_string());
    let mut second = SelfReferential::new("second".to_string());
    unsafe {
        ptr::swap(&mut first, &mut second);
    }
    first.print();

    let duration = Duration::from_secs(3);
    let result = timeout(duration, slow_task()).await;
    match result {
        Ok(value) => println!("Task completed successfully: {}", value),
        Err(_) => println!("Task timed out"),
    }
    // “Sharing data between Futures”
}
