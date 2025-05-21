## Rust 并发与异步

### Rust 中线程的使用

## 示例 (Examples)

### `a_raw_syscall.rs`
Demonstrates how to make raw system calls (like `write` to `stdout`) using inline assembly for both Linux and macOS (aarch64). It bypasses standard library abstractions for syscalls.

### `aa_os_threads.rs`
Shows basic usage of operating system threads in Rust using `std::thread::spawn`. It illustrates creating threads, running tasks concurrently, and using `join` to wait for thread completion. It also shows a simple way to chain thread creation.

### `ac_assembly_dereference.rs`
Illustrates how to dereference a raw pointer using inline assembly for x86_64 and aarch64 architectures. It includes an example of trying to dereference a potentially invalid memory address.

### `async_blocking.rs`
Explores different ways to handle blocking tasks in an asynchronous Tokio runtime, comparing `tokio::time::sleep` vs `std::thread::sleep`, `tokio::task::spawn_blocking`, Rayon for parallelism, and standard library threads, with benchmarks.

### `async_file.rs`
Demonstrates asynchronous file writing using a custom `AsyncWriteFuture` and `tokio::task::spawn` to write concurrently to two log files, showing that entry order is not guaranteed.

### `b_normal_syscall.rs`
Shows how to make a system call (`write`) using standard C library functions linked into Rust, a more "normal" way compared to raw inline assembly.

### `coffee_toast.rs`
Models making coffee and toast concurrently using Tokio, defining async functions with both async and blocking delays, and can use `tokio::join!` or `tokio::task::spawn`.

### `communicating_with_process.rs`
A simple program that prints its process ID and echoes lines from `stdin` to `stdout`, demonstrating basic inter-process communication.

### `fibonacci_thread.rs`
Calculates Fibonacci numbers recursively and compares sequential execution time with concurrent execution using `std::thread::spawn` for CPU-bound tasks.

### `futures.rs`
Explores `Future` trait concepts: `tokio::time::timeout`, a `SelfReferential` struct (hinting at `Pin` needs), and a custom `CounterFuture` implementing `Future` manually with `cx.waker().wake_by_ref()`.

### `listening_socket_with_mio.rs`
Demonstrates a simple TCP server and client using `mio` for low-level, non-blocking I/O. It defines a `ServerFuture` that manually implements `Future` using `mio::Poll` and runs it on a custom runtime.

### `mredis.rs`
A miniature Redis-like server using Tokio, handling multiple client connections asynchronously and responding with `+OK\r\n`. Uses `tracing` for logging.

### `networking_into_own_async.rs`
A complex example integrating `hyper` with a custom asynchronous runtime. It defines custom stream, connector, and executor types to make HTTP/S requests using `smol` for TCP and `async-native-tls` for TLS.

### `own_async_queue.rs`
Implements a custom asynchronous task runtime with a priority-based scheduler using `async-task`, `flume` channels, and worker threads.

### `shared_state.rs`
Demonstrates sharing state across threads using `Arc<Mutex<T>>` for a counter, `AtomicU8` for lock-free atomic operations on a struct field, and contrasts it with `Mutex`-protected fields.

### `sharing_data_between_futures.rs`
Focuses on sharing data between Tokio tasks, contrasting a manual `Future` with `std::sync::Mutex` (commented out) with the preferred `tokio::sync::Mutex` within async functions.

### `thread.rs`
Implements a multi-producer, single-consumer scenario using `std::thread` and `std::sync::mpsc` channels, where producers send messages with random values and delays.

### `thread1.rs`
Covers `std::thread` fundamentals: spawning threads, moving ownership of data into threads, and using `mpsc::channel` for basic message passing.

## 依赖库 (Dependencies)

### `rayon = "1.10.0"`
Rayon is a data parallelism library for Rust. It makes it easy to convert sequential computations into parallel ones.
*Usage:* Seen in `async_blocking.rs` with `par_iter().sum()` to perform a parallel sum calculation.

### `anyhow = "1.0.98"`
Anyhow is a library for flexible error handling in Rust applications, providing `anyhow::Result<T>` and the `anyhow!` macro.
*Usage:* Used in several examples (`shared_state.rs`, `thread.rs`, `networking_into_own_async.rs`) for concise error management.

### `rand = "0.9.1"`
Rand is a library for generating random numbers in Rust.
*Usage:* Used in `thread.rs` to generate random values for messages and random sleep times.

### `tokio = { version = "1.44.2", features = [...] }`
Tokio is an asynchronous runtime for Rust, providing tools for I/O-bound tasks, networking, timers, and synchronization.
*Features enabled:* `rt`, `rt-multi-thread`, `macros`, `net`, `io-util`, `time`, `sync`.
*Usage:* Extensively used in many examples (`async_blocking.rs`, `async_file.rs`, `mredis.rs`, etc.) as the primary asynchronous runtime.

### `tracing = "0.1.40"`
Tracing is a framework for instrumenting Rust programs to collect structured, event-based diagnostic information.
*Usage:* Used with `tracing-subscriber` in `mredis.rs` for logging.

### `tracing-subscriber = { version = "0.3.18", features = ["env-filter"] }`
Provides tools to collect and process `tracing` data. The `env-filter` feature allows runtime configuration of active traces.
*Usage:* Used in `mredis.rs` to initialize the logging system.

### `futures-util = "0.3"`
A utility library for working with futures in Rust, providing helper functions and combinators.
*Usage:* Seen in `async_file.rs` (`futures_util::future::join_all`) to await multiple file writing futures.

### `async-task = "4.4.0"`
A crate for creating and managing individual asynchronous tasks, often used by custom executor implementations.
*Usage:* Core component in `own_async_queue.rs` for building the custom priority-based executor.

### `futures-lite = "2.6.0"`
A lightweight alternative to `futures-util`, providing essential future combinators and utilities.
*Usage:* Used for `future::block_on` and other utilities in `listening_socket_with_mio.rs`, `own_async_queue.rs`, and `networking_into_own_async.rs`.

### `once_cell = "1.21.3"`
Provides types for safe, one-time initialization of data, like `Lazy` for static variables.
*Usage:* Used in `own_async_queue.rs` to lazily initialize static channels and worker queues.

### `flume = "0.11.1"`
A fast and safe multi-producer, multi-consumer (MPMC) channel library.
*Usage:* Used in `own_async_queue.rs` as the underlying channel mechanism for task queues in the custom executor.

### `hyper = { version = "0.14.26", features = [...] }`
Hyper is a fast and correct HTTP library for Rust, providing both client and server capabilities.
*Features enabled:* `client`, `runtime`, `http1`, `http2`.
*Usage:* Used in `networking_into_own_async.rs` to build an HTTP client integrated with a custom async runtime.

### `smol = "2.0.2"`
Smol is a small, simple, and fast asynchronous runtime.
*Usage:* Used in `networking_into_own_async.rs` to provide `Async<TcpStream>` for `hyper` integration.

### `async-native-tls = "0.5.0"`
Provides an asynchronous interface for TLS using native OS libraries.
*Usage:* Used in `networking_into_own_async.rs` to establish TLS connections for HTTPS.

### `http = "0.2.9"`
Provides shared types for HTTP (e.g., `Uri`, `Request`, `Response`). It's a foundational crate for HTTP libraries.
*Usage:* Used in `networking_into_own_async.rs`, often via `hyper`, for HTTP type definitions.

### `mio = { version = "0.8.8", features = [...] }`
Mio (Metal I/O) is a low-level, non-blocking I/O library, abstracting over platform-specific I/O selectors (epoll, kqueue, IOCP).
*Features enabled:* `net`, `os-poll`.
*Usage:* Used in `listening_socket_with_mio.rs` to build a basic non-blocking TCP server and client with manual event polling.
