use rayon::prelude::*;
//use std::time::Duration;
use tokio::time::Duration;

#[allow(dead_code)]
async fn sleep_then_print(timer: i32) {
    println!("Start timer {}.", timer);

    // 使用标准库thread，线程会睡眠3秒，线程阻塞。
    //std::thread::sleep(Duration::from_secs(3));
    // 使用tokio time sleep，非阻塞，线程同步执行
    tokio::time::sleep(Duration::from_secs(3)).await;
    println!("Timer {} done.", timer);
}

async fn parallel_sum_with_rayon(nums: Vec<i32>) -> i32 {
    let (send, recv) = tokio::sync::oneshot::channel();

    rayon::spawn(move || {
        let sum = nums.par_iter().sum();
        let _ = send.send(sum);
    });

    recv.await.expect("Failed to receive sum")
}

async fn parallel_sum(nums: Vec<i32>) -> i32 {
    // 使用tokio 阻塞线程执行计算任务
    tokio::task::spawn_blocking(move || {
        let mut sum = 0;
        for num in nums {
            sum += num;
        }
        sum
    })
    .await
    .expect("Failed to spawn blocking task")
}

fn parallel_sum_thread(nums: Vec<i32>) -> i32 {
    // 使用tokio 阻塞线程执行计算任务
    std::thread::spawn(move || {
        let mut sum = 0;
        for num in nums {
            sum += num;
        }
        sum
    })
    .join()
    .expect("Failed to spawn blocking task")
}

#[tokio::main]
async fn main() {
    //println!("Hello World!");

    //no .await here!
    //std::thread::sleep(Duration::from_secs(5));

    //println!("Five seconds later...");

    // tokio::join!(
    //     sleep_then_print(1),
    //     sleep_then_print(2),
    //     sleep_then_print(3),
    // );

    // tokio 中的使用阻塞线程
    // let blocking_task = tokio::task::spawn_blocking(|| {
    //     println!("Blocking task is running.");
    //     std::thread::sleep(Duration::from_secs(5));
    //     println!("Blocking task is done.");
    // });

    // blocking_task.await.unwrap();
    // 记录运行时间
    let start = std::time::Instant::now();
    let nums = vec![1; 1024 * 1024 * 1024];
    println!("Sum: {}", parallel_sum(nums).await);
    let end = std::time::Instant::now();
    println!("Tokio blocking Time elapsed: {:?}", end - start);
    // 记录运行时间
    let start = std::time::Instant::now();
    let nums = vec![1; 1024 * 1024 * 1024];
    println!("Sum: {}", parallel_sum_with_rayon(nums).await);
    let end = std::time::Instant::now();
    println!("Tokio blocking with rayon Time elapsed: {:?}", end - start);
    // 记录运行时间
    let start = std::time::Instant::now();
    let nums = vec![1; 1024 * 1024 * 1024];
    println!("Sum: {}", parallel_sum_thread(nums));
    let end = std::time::Instant::now();
    println!("std Thread  Time elapsed: {:?}", end - start);
}
