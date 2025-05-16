use std::thread;
use std::time::Duration;
use std::time::Instant;
use tokio::time::sleep;

async fn prep_coffee_mug() {
    sleep(Duration::from_millis(100)).await; // Simulate async work
    println!("Pouring milk...");
    thread::sleep(Duration::from_secs(3));
    println!("Milk poured.");
    println!("Putting instant coffee...");
    thread::sleep(Duration::from_secs(3));
    println!("Instant coffee put.");
}

async fn make_coffee() {
    println!("boiling kettle...");
    sleep(Duration::from_secs(10)).await;
    println!("kettle boiled.");
    println!("pouring boiled water...");
    thread::sleep(Duration::from_secs(3));
    println!("boiled water poured.");
}

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

    //
    // let person_one = tokio::task::spawn(async {
    //     let coffee_mug_step = prep_coffee_mug();
    //     let coffee_step = make_coffee();
    //     let toast_step = make_toast();
    //     tokio::join!(coffee_mug_step, coffee_step, toast_step);
    // });

    let person_one = tokio::task::spawn(async {
        prep_coffee_mug().await;
        make_coffee().await;
        make_toast().await;
    });

    person_one.await.unwrap();

    let elapsed_time = start_time.elapsed();
    println!("It took: {} seconds", elapsed_time.as_secs());
}
