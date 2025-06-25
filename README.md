## Rust 并发与异步

### Rust 中线程的使用

### Examples

*   **a_raw_syscall.rs**: 使用内联汇编在Linux和macOS上进行原始系统调用。
*   **aa_os_threads.rs**: Rust中基本的OS线程创建和加入。
*   **ac_assembly_dereference.rs**: 使用内联汇编解引用原始指针的示例。
*   **async_blocking.rs**: 比较阻塞和非阻塞睡眠，并展示如何在异步上下文中使用`spawn_blocking`处理CPU密集型任务。
*   **async_file.rs**: 使用自定义`Future`实现异步文件写入。
*   **async_web_server.rs**: 使用`tokio`的简单异步Web服务器。
*   **b_normal_syscall.rs**: 展示如何使用标准库的`libc`包装器进行系统调用。
*   **coffee_toast.rs**: 使用`tokio::join!`并发运行多个异步任务的实际示例。
*   **communicating_with_process.rs**: 从标准输入读取的简单示例。
*   **controlling_coroutines.rs**: 控制协程执行的示例。
*   **coroutines_base.rs**: 使用协程将数字写入文件的基本示例。
*   **evenbus.rs**: 使用`tokio`和`mutex`实现一个简单的事件总线。
*   **fibonacci_thread.rs**: 比较顺序执行和使用线程并行执行CPU密集型任务的性能。
*   **futures.rs**: 使用`Future`、`Pin`和`timeout`的示例。
*   **graceful_shutdowns.rs**: 展示如何在`tokio`应用程序中处理优雅关闭。
*   **listening_socket_with_mio.rs**: 使用`mio`创建非阻塞TCP服务器的简单示例。
*   **mredis.rs**: 使用`tokio`的简单类Redis服务器实现。
*   **multi_thread_web_server.rs**: 一个简单的多线程Web服务器。
*   **networking_into_own_async.rs**: 如何将自定义异步运行时与`hyper`集成的示例。
*   **own_async_queue.rs**: 实现一个带有优先级队列的自定义异步运行时。
*   **reactive_programming.rs**: 使用future模拟恒温器的反应式编程的简单示例。
*   **shared_state.rs**: 使用`Mutex`和`Arc`在线程之间共享状态的示例。
*   **sharing_data_between_futures.rs**: 展示如何使用`Arc<Mutex<T>>`在future之间共享数据。
*   **simple_generator.rs**: 使用协程作为生成器从文件中读取数字的简单示例。
*   **single_thread_web_server.rs**: 一个简单的单线程Web服务器。
*   **thread.rs**: 使用通道在线程之间进行通信的示例。
*   **thread1.rs**: 创建和使用线程的基本示例。
*   **tokio_runtime_setup.rs**: 展示如何配置`tokio`运行时。
*   **two_coroutines.rs**: 两个协程相互通信的示例。
*   **unsafe_with_thread.rs**: 在`tokio`应用程序中使用`UnsafeCell`在线程之间共享数据的示例。
