/*
dependencies added to Cargo.toml
async-task = "4.4.0"
futures-lite = "1.12.0"
once_cell = "1.17.1"
flume = "0.10.14"
 */

use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;
use std::{future::Future, panic::catch_unwind, thread};

use async_task::{Runnable, Task};
use flume::{Receiver, Sender};
use futures_lite::future;
use once_cell::sync::Lazy;

#[macro_export]
macro_rules! spawn_task {
    ($future:expr) => {
        spawn_task_function($future, FutureType::Low)
    };
    ($future:expr, $order:expr) => {
        spawn_task_function($future, $order)
    };
}

#[macro_export]
macro_rules! join {
    ($($future:expr),*) => {
        {
            let results: Vec<_> = vec![$(future::block_on($future)),*];
            results
        }
    };
}

#[macro_export]
macro_rules! try_join {
    ($($future:expr),*) => {
        {
            let mut results = Vec::new();
            $(
                let result = catch_unwind(|| {
                    future::block_on($future)
                });
                results.push(result);
            )*
            results
        }
    };
}

#[derive(Debug, Clone, Copy)]
pub enum FutureType {
    High,
    Low,
}

pub fn spawn_task_function<F, T>(future: F, order: FutureType) -> Task<T>
where
    F: Future<Output = T> + Send + 'static,
    T: Send + 'static,
{
    static HIGH_CHANNEL: Lazy<(Sender<Runnable>, Receiver<Runnable>)> =
        Lazy::new(flume::unbounded::<Runnable>);
    static LOW_CHANNEL: Lazy<(Sender<Runnable>, Receiver<Runnable>)> =
        Lazy::new(flume::unbounded::<Runnable>);

    static HIGH_QUEUE: Lazy<flume::Sender<Runnable>> = Lazy::new(|| {
        let high_num = std::env::var("HIGH_NUM").unwrap().parse::<usize>().unwrap();
        for _ in 0..high_num {
            let high_receiver = HIGH_CHANNEL.1.clone();
            let low_receiver = LOW_CHANNEL.1.clone();
            thread::spawn(move || loop {
                match high_receiver.try_recv() {
                    Ok(runnable) => {
                        let _ = catch_unwind(|| runnable.run());
                    }
                    Err(_) => match low_receiver.try_recv() {
                        Ok(runnable) => {
                            let _ = catch_unwind(|| runnable.run());
                        }
                        Err(_) => {
                            thread::sleep(Duration::from_millis(100));
                        }
                    },
                }
            });
        }
        HIGH_CHANNEL.0.clone()
    });
    static LOW_QUEUE: Lazy<flume::Sender<Runnable>> = Lazy::new(|| {
        let low_num = std::env::var("LOW_NUM").unwrap().parse::<usize>().unwrap();
        for _ in 0..low_num {
            let high_receiver = HIGH_CHANNEL.1.clone();
            let low_receiver = LOW_CHANNEL.1.clone();

            thread::spawn(move || loop {
                match low_receiver.try_recv() {
                    Ok(runnable) => {
                        let _ = catch_unwind(|| runnable.run());
                    }
                    Err(_) => match high_receiver.try_recv() {
                        Ok(runnable) => {
                            let _ = catch_unwind(|| runnable.run());
                        }
                        Err(_) => {
                            thread::sleep(Duration::from_millis(100));
                        }
                    },
                }
            });
        }
        LOW_CHANNEL.0.clone()
    });

    let schedule_high = |runnable| HIGH_QUEUE.send(runnable).unwrap();
    let schedule_low = |runnable| LOW_QUEUE.send(runnable).unwrap();
    let schedule = match order {
        FutureType::High => schedule_high,
        FutureType::Low => schedule_low,
    };
    let (runnalbe, task) = async_task::spawn(future, schedule);
    runnalbe.schedule();
    task
}

pub struct CounterFuture {
    count: u32,
}

impl Future for CounterFuture {
    type Output = u32;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.count += 1;
        println!("polling with result: {}", self.count);
        thread::sleep(Duration::from_secs(1));
        if self.count < 3 {
            cx.waker().wake_by_ref();
            Poll::Pending
        } else {
            Poll::Ready(self.count)
        }
    }
}

pub async fn async_fn() {
    std::thread::sleep(Duration::from_secs(1));
    println!("async fn");
}

pub struct Runtime {
    high_num: usize,
    low_num: usize,
}

impl Runtime {
    pub fn new() -> Self {
        let num_cores = std::thread::available_parallelism().unwrap().get();
        Self {
            high_num: num_cores - 2,
            low_num: 1,
        }
    }
    pub fn with_high_num(mut self, num: usize) -> Self {
        self.high_num = num;
        self
    }
    pub fn with_low_num(mut self, num: usize) -> Self {
        self.low_num = num;
        self
    }
    pub fn run(&self) {
        std::env::set_var("HIGH_NUM", self.high_num.to_string());
        std::env::set_var("LOW_NUM", self.low_num.to_string());

        let high = spawn_task_function(async {}, FutureType::High);
        let low = spawn_task_function(async {}, FutureType::Low);
        join!(high, low);
    }
}

impl Default for Runtime {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct BackgroundProcess;

impl Future for BackgroundProcess {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!("Background process running");
        std::thread::sleep(Duration::from_secs(1));
        cx.waker().wake_by_ref();
        Poll::Pending
    }
}
