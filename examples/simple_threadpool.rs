use std::thread;
use std::time::Duration;
use threadpool::ThreadPool;

fn main() {
    // 1. 创建一个线程池，包含 4 个线程
    let pool = ThreadPool::new(4);

    // 2. 提交 10 个任务到线程池
    for i in 0..10 {
        // 使用 move 将 i 的所有权移入闭包
        pool.execute(move || {
            println!("任务 {} 正在由线程 {:?} 执行。", i, thread::current().id());
            // 模拟一些耗时的工作
            thread::sleep(Duration::from_secs(1));
            println!("任务 {} 完成。", i);
        });
    }

    // 3. 等待所有任务完成
    // 当 pool 离开作用域时，它的 Drop 实现会自动调用 join()。
    // 我们也可以显式调用 pool.join()。
    println!("所有任务已提交。等待它们完成。");
    pool.join();
    println!("所有任务都已完成！");
}
