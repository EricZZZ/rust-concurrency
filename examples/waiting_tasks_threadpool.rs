use std::thread;
use std::time::{Duration, Instant};
use threadpool::ThreadPool;

fn main() {
    let pool = ThreadPool::new(4);
    let start_time = Instant::now();

    println!("提交 5 个任务到 4 个线程的池中。");

    for i in 1..=5 {
        pool.execute(move || {
            let now = Instant::now();
            println!(
                "任务 {} 开始执行。 距离程序启动: {:.2} 秒。线程 ID: {:?}",
                i,
                now.duration_since(start_time).as_secs_f32(),
                thread::current().id()
            );
            thread::sleep(Duration::from_secs(2)); // 使用2秒代替10秒，方便演示
            println!("  -> 任务 {} 完成。", i);
        });
    }

    pool.join();
    println!(
        "所有任务完成。总耗时: {:.2} 秒",
        start_time.elapsed().as_secs_f32()
    );
}
