use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll},
    time::Duration,
};

#[derive(Debug)]
enum CounterType {
    Increment,
    Decrement,
}

struct SharedData {
    counter: i32,
}

impl SharedData {
    fn increment(&mut self) {
        self.counter += 1;
    }
    fn decrement(&mut self) {
        self.counter -= 1;
    }
}

struct CounterFuture {
    counter_type: CounterType,
    data_reference: Arc<Mutex<SharedData>>,
    count: u32,
}

impl Future for CounterFuture {
    type Output = u32;
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        std::thread::sleep(Duration::from_secs(1));
        let mut guard = match self.data_reference.try_lock() {
            Ok(guard) => guard,
            Err(error) => {
                println!("error for {:?}: {}", self.counter_type, error);
                cx.waker().wake_by_ref();
                return Poll::Pending;
            }
        };
        let value = &mut *guard;
        match self.counter_type {
            CounterType::Increment => {
                value.increment();
                println!("after increment: {}", value.counter);
            }
            CounterType::Decrement => {
                value.decrement();
                println!("after decrement: {}", value.counter);
            }
        }
        std::mem::drop(guard);
        self.count += 1;
        if self.count < 3 {
            cx.waker().wake_by_ref();
            Poll::Pending
        } else {
            Poll::Ready(self.count)
        }
    }
}

async fn count(
    count: u32,
    data: Arc<tokio::sync::Mutex<SharedData>>,
    counter_type: CounterType,
) -> u32 {
    for _ in 0..count {
        let mut data = data.lock().await;
        match counter_type {
            CounterType::Increment => {
                data.increment();
                println!("after increment: {}", data.counter);
            }
            CounterType::Decrement => {
                data.decrement();
                println!("after decrement: {}", data.counter);
            }
        }
        std::mem::drop(data);
        std::thread::sleep(Duration::from_secs(1));
    }
    count
}

#[tokio::main]
async fn main() {
    // // low level shared data between futures
    // let shared_data = Arc::new(Mutex::new(SharedData { counter: 0 }));
    // let shared_data_clone = shared_data.clone();
    // let counter_one = CounterFuture {
    //     counter_type: CounterType::Increment,
    //     data_reference: shared_data_clone,
    //     count: 0,
    // };
    // let counter_two = CounterFuture {
    //     counter_type: CounterType::Decrement,
    //     data_reference: shared_data,
    //     count: 0,
    // };
    // let handle_one: JoinHandle<u32> = tokio::task::spawn(async move { counter_one.await });
    // let handle_two: JoinHandle<u32> = tokio::task::spawn(async move { counter_two.await });
    // tokio::join!(handle_one, handle_two);

    // high level shared data between futures
    let shared_data = Arc::new(tokio::sync::Mutex::new(SharedData { counter: 0 }));
    let shared_data_clone = shared_data.clone();
    let handle_one =
        tokio::task::spawn(
            async move { count(3, shared_data_clone, CounterType::Increment).await },
        );
    let handle_two =
        tokio::task::spawn(async move { count(3, shared_data, CounterType::Decrement).await });
    let _ = tokio::join!(handle_one, handle_two);
}
