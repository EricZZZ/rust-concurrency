use once_cell::sync::Lazy;
use std::cell::RefCell;
use std::future::Future;
use std::time::Duration;
use tokio::runtime::{Builder, Runtime};
use tokio::task::JoinHandle;
use tokio_util::task::LocalPoolHandle;

static RUNTIME: Lazy<Runtime> = Lazy::new(|| {
    Builder::new_multi_thread()
        .worker_threads(4)
        .max_blocking_threads(1)
        .on_thread_start(|| {
            println!("thread starting for runtime A");
        })
        .on_thread_stop(|| {
            println!("thread stopping for runtime A");
        })
        .thread_keep_alive(Duration::from_secs(60))
        .global_queue_interval(61)
        .on_thread_park(|| {
            println!("thread parked for runtime A");
        })
        .thread_name("our custom runtime A")
        .thread_stack_size(3 * 1024 * 1024)
        .enable_time()
        .build()
        .unwrap()
});

#[allow(dead_code)]
static HIGH_PRIORITY: Lazy<Runtime> = Lazy::new(|| {
    Builder::new_multi_thread()
        .worker_threads(2)
        .thread_name("High Priority Runtime")
        .enable_time()
        .build()
        .unwrap()
});

#[allow(dead_code)]
static LOW_PRIORITY: Lazy<Runtime> = Lazy::new(|| {
    Builder::new_multi_thread()
        .worker_threads(1)
        .thread_name("Low Priority Runtime")
        .enable_time()
        .build()
        .unwrap()
});

thread_local! {
    pub static COUNTER: RefCell<u32> = const { RefCell::new(1) };
}

async fn something(number: u32) -> u32 {
    // std::thread::sleep(Duration::from_secs(3));
    tokio::time::sleep(Duration::from_secs(3)).await;
    COUNTER.with(|counter| {
        *counter.borrow_mut() += 1;
        println!("Counter: {} for: {}", *counter.borrow(), number);
    });
    number
}

pub fn spawn_task<F, T>(future: F) -> JoinHandle<T>
where
    F: Future<Output = T> + Send + 'static,
    T: Send + 'static,
{
    RUNTIME.spawn(future)
}

#[allow(dead_code)]
async fn sleep_example() -> i32 {
    println!("sleeping for 2 seconds");
    tokio::time::sleep(Duration::from_secs(2)).await;
    println!("sleep done");
    20
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // let handle = spawn_task(sleep_example());
    // println!("spawned task");
    // println!("task status: {:?}", handle.is_finished());
    // std::thread::sleep(Duration::from_secs(3));
    // println!("task status: {:?}", handle.is_finished());
    // let result = RUNTIME.block_on(handle).unwrap();
    // println!("task result: {}", result);

    let pool = LocalPoolHandle::new(3);
    let one = pool.spawn_pinned_by_idx(
        || async {
            println!("one");
            something(1).await
        },
        0,
    );
    let two = pool.spawn_pinned_by_idx(
        || async {
            println!("two");
            something(2).await
        },
        0,
    );
    let three = pool.spawn_pinned_by_idx(
        || async {
            println!("three");
            something(3).await
        },
        0,
    );

    let result = async {
        let one = one.await.unwrap();
        let two = two.await.unwrap();
        let three = three.await.unwrap();
        one + two + three
    };
    println!("result: {}", result.await);
}
