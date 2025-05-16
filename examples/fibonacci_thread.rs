use std::thread;
use std::time::Instant;

fn fibonacci(n: u64) -> u64 {
    if n == 0 || n == 1 {
        return n;
    }
    fibonacci(n - 1) + fibonacci(n - 2)
}

fn main() {
    //在2核服务器上执行，虽然启用了4个线程，但并没有快4倍，具体执行结果还是要看CPU的配置
    //4 * fibonacci(40) in 4.803429869s
    //4 threads fibonacci(40) took 2.472912978s
    let start = Instant::now();
    let _ = fibonacci(40);
    let _ = fibonacci(40);
    let _ = fibonacci(40);
    let _ = fibonacci(40);
    let duration = start.elapsed();
    println!("4 * fibonacci(40) in {:?}", duration);

    let start = Instant::now();
    let mut handles = vec![];
    for _ in 0..4 {
        let handle = thread::spawn(|| fibonacci(40));
        handles.push(handle);
    }

    for handle in handles {
        let _ = handle.join();
    }
    let duration = start.elapsed();
    println!("4 threads fibonacci(40) took {:?}", duration);
}
