# Web Server Performance Comparison

This document summarizes the benchmark results for three different web server implementations in Rust, tested under two distinct scenarios: a CPU-bound (low-latency) task and an I/O-bound (high-latency) task.

The tests were conducted using `wrk` with the following command structure:
`wrk -t4 -c100 -d10s <URL>`

---

## Test 1: CPU-Bound Scenario (Serving a small local file)

In this test, we benchmarked the servers by requesting a small HTML file (`/`). This simulates a low-latency, CPU-intensive workload where the server's speed in handling raw requests is the primary factor.

### Results

| Server Model | Requests/sec (RPS) | Avg Latency | Notes |
| :--- | :--- | :--- | :--- |
| **Single-Thread** | ~1600 | 6.33ms | High number of read errors |
| **Multi-Thread** | ~1606 | **2.63ms** | High number of read errors |
| **Async** | **~1617** | 5.66ms | High number of read errors |

### Visualization (RPS)

```
Single-Thread: ■■■■■■■■■■■■■■■■ (1600)
Multi-Thread:  ■■■■■■■■■■■■■■■■ (1606)
Async:         ■■■■■■■■■■■■■■■■ (1617)
```

### Analysis

In this scenario, all three models performed similarly in terms of raw request throughput. The high number of socket read errors indicates that under this intense, low-latency load, the servers struggled to keep up with `wrk`'s aggressive connection rate. This test did not highlight the key differences between the models because the task did not involve waiting for I/O.

---

## Test 2: I/O-Bound Scenario (Simulating a slow operation)

This test is designed to reveal the true power of different concurrency models. We benchmarked the `/sleep` endpoint, which forces the server to wait for a fixed duration before responding. This simulates real-world I/O operations like database queries or external API calls.

### Results

| Server Model | Requests/sec (RPS) | Total Requests (in 10s) |
| :--- | :--- | :--- |
| **Single-Thread** | 0.20 | 2 |
| **Multi-Thread** | 0.79 | 8 |
| **Async** | **9.92** | **100** |

### Visualization (Total Requests Completed)

```
Single-Thread: ■ (2)

Multi-Thread:  ■■■■ (8)

Async:         ■■■■■■■■■■■■■■■■■■■■■■■■■ (100)
```

### Analysis

The results here are dramatically different and clearly illustrate the strengths and weaknesses of each approach:

*   **Single-Thread:** It can only handle one request at a time. While it was busy waiting, all other connections timed out. It could only serve two requests sequentially in the 10-second window.
*   **Multi-Thread:** With a pool of 4 threads, it could handle 4 requests concurrently. After 5 seconds, those 4 threads became free and handled the next 4 requests, successfully processing a total of 8 requests. Its capacity is strictly limited by the number of threads.
*   **Async:** The asynchronous server shines here. When a task awaited the sleep, it yielded control, allowing the underlying thread to immediately start processing other requests. It successfully initiated the "wait" for all 100 concurrent connections without blocking, and thus completed all 100 requests when their timers expired. This demonstrates its superior ability to handle high-concurrency I/O-bound workloads with minimal resources.

## Conclusion

While all models perform similarly for simple, fast, CPU-bound tasks, the **asynchronous model is vastly superior for applications involving I/O-bound operations**, which is the most common scenario for web services. It provides the highest throughput and the most efficient resource utilization.
