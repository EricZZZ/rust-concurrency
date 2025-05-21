use std::future::Future;
use std::task::Poll;
use std::thread;
use std::time::Duration;
use std::time::Instant;
use tokio::time::sleep;

struct Food {
    name: String,
    make_time_secs: i32,
}

impl Food {
    fn new(name: &str, make_time_secs: i32) -> Self {
        Self {
            name: name.to_string(),
            make_time_secs,
        }
    }
}

impl Future for Food {
    type Output = String;
    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        println!("making {}, {}s left", self.name, self.make_time_secs);
        self.make_time_secs -= 1;

        // sleep(Duration::from_secs(1));
        std::thread::sleep(Duration::from_secs(1));
        if self.make_time_secs > 0 {
            cx.waker().wake_by_ref();
            Poll::Pending
        } else {
            Poll::Ready(self.name.clone())
        }
    }
}

#[allow(dead_code)]
async fn prep_coffee_mug() {
    sleep(Duration::from_millis(100)).await; // Simulate async work
    println!("Pouring milk...");
    thread::sleep(Duration::from_secs(3));
    println!("Milk poured.");
    println!("Putting instant coffee...");
    thread::sleep(Duration::from_secs(3));
    println!("Instant coffee put.");
}

#[allow(dead_code)]
async fn make_coffee() {
    println!("boiling kettle...");
    sleep(Duration::from_secs(10)).await;
    println!("kettle boiled.");
    println!("pouring boiled water...");
    thread::sleep(Duration::from_secs(3));
    println!("boiled water poured.");
}

#[allow(dead_code)]
async fn make_toast() {
    println!("putting bread in toaster...");
    sleep(Duration::from_secs(10)).await;
    println!("bread toasted.");
    println!("buttering toasted bread...");
    thread::sleep(Duration::from_secs(5));
    println!("toasted bread buttered.");
}

#[tokio::main(flavor = "multi_thread", worker_threads = 1)]
async fn main() {
    let start_time = Instant::now();
    // let coffee_mug_step = prep_coffee_mug();
    // let coffee_step = make_coffee();
    // let toast_step = make_toast();

    // // Use tokio::join! to run the tasks concurrently
    // tokio::join!(coffee_mug_step, coffee_step, toast_step,);

    // let person_one = tokio::task::spawn(async {
    //     let coffee_mug_step = prep_coffee_mug();
    //     let coffee_step = make_coffee();
    //     let toast_step = make_toast();
    //     tokio::join!(coffee_mug_step, coffee_step, toast_step);
    // });

    // let person_one = tokio::task::spawn(async {
    //     prep_coffee_mug().await;
    //     make_coffee().await;
    //     make_toast().await;
    // });

    let coffee = Food::new("咖啡", 2);
    let coffee_mug = Food::new("咖啡杯", 2);
    let toast = Food::new("吐司", 2);
    let person_one = tokio::spawn(async {
        tokio::join!(coffee, coffee_mug, toast);
    });

    person_one.await.unwrap();

    let elapsed_time = start_time.elapsed();
    println!("It took: {} seconds", elapsed_time.as_secs());
}
