use std::thread::sleep;

fn main() {
    println!("So, we start the program here!");
    let t1 = std::thread::spawn(|| {
        sleep(std::time::Duration::from_millis(200));
        println!("The long running tasks finish last!");
    });

    let t2 = std::thread::spawn(|| {
        sleep(std::time::Duration::from_millis(100));
        println!("We can chain callbacks...");
        let t3 = std::thread::spawn(|| {
            sleep(std::time::Duration::from_millis(50));
            println!("... and even more callbacks!");
        });

        t3.join().unwrap();
    });
    println!("The tasks run concurrently!");

    t1.join().unwrap();
    t2.join().unwrap();
}
