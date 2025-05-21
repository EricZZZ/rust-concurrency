#![feature(coroutines, coroutine_trait, stmt_expr_attributes)]
use std::collections::VecDeque;
use std::fs::{File, OpenOptions};
use std::future::Future;
use std::io::Write;
use std::io::{self, BufRead, BufReader};
use std::ops::{Coroutine, CoroutineState};
use std::pin::Pin;
use std::task::Poll;
use std::time::{Duration, Instant};

type CoroutineHandle = Pin<Box<dyn Coroutine<(), Yield = (), Return = ()>>>;

struct WriteCoroutine {
    pub file_handle: File,
}

impl WriteCoroutine {
    fn new(path: &str) -> Self {
        Self {
            file_handle: OpenOptions::new()
                .create(true)
                .append(true)
                .open(path)
                .unwrap(),
        }
    }
}

impl Coroutine<i32> for WriteCoroutine {
    type Yield = ();
    type Return = ();
    fn resume(mut self: Pin<&mut Self>, arg: i32) -> CoroutineState<Self::Yield, Self::Return> {
        writeln!(self.file_handle, "{}", arg).unwrap();
        CoroutineState::Yielded(())
    }
}

struct ReadCoroutine {
    lines: io::Lines<BufReader<File>>,
}

impl ReadCoroutine {
    fn new(path: &str) -> io::Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let lines = reader.lines();

        Ok(Self { lines })
    }
}

impl Coroutine<()> for ReadCoroutine {
    type Yield = i32;
    type Return = ();
    fn resume(mut self: Pin<&mut Self>, _arg: ()) -> CoroutineState<Self::Yield, Self::Return> {
        match self.lines.next() {
            Some(Ok(line)) => {
                if let Ok(number) = line.parse::<i32>() {
                    CoroutineState::Yielded(number)
                } else {
                    CoroutineState::Complete(())
                }
            }
            Some(Err(_)) | None => CoroutineState::Complete(()),
        }
    }
}

#[allow(dead_code)]
struct CoroutineManager {
    reader: ReadCoroutine,
    writer: WriteCoroutine,
}

#[allow(dead_code)]
impl CoroutineManager {
    fn new(read_path: &str, write_path: &str) -> io::Result<Self> {
        let reader = ReadCoroutine::new(read_path)?;
        let writer = WriteCoroutine::new(write_path);
        Ok(Self { reader, writer })
    }
    fn run(&mut self) {
        let mut read_pin = Pin::new(&mut self.reader);
        let mut write_pin = Pin::new(&mut self.writer);

        while let CoroutineState::Yielded(number) = read_pin.as_mut().resume(()) {
            write_pin.as_mut().resume(number);
        }
        // loop {
        //     match read_pin.as_mut().resume(()) {
        //         CoroutineState::Yielded(number) => {
        //             write_pin.as_mut().resume(number);
        //         }
        //         CoroutineState::Complete(()) => {
        //             break;
        //         }
        //     }
        // }
    }
}

trait SymmetricCoroutine {
    type Input;
    type Output;

    fn resume_with_input(self: Pin<&mut Self>, input: Self::Input) -> Self::Output;
}

impl SymmetricCoroutine for ReadCoroutine {
    type Input = ();
    type Output = Option<i32>;

    fn resume_with_input(mut self: Pin<&mut Self>, _input: ()) -> Self::Output {
        if let Some(Ok(line)) = self.lines.next() {
            line.parse::<i32>().ok()
        } else {
            None
        }
    }
}

impl SymmetricCoroutine for WriteCoroutine {
    type Input = i32;
    type Output = ();
    fn resume_with_input(mut self: Pin<&mut Self>, input: Self::Input) -> Self::Output {
        writeln!(self.file_handle, "{}", input).unwrap();
    }
}

struct SleepCoroutine {
    pub start: Instant,
    pub duration: Duration,
}

impl SleepCoroutine {
    fn new(duration: Duration) -> Self {
        Self {
            start: Instant::now(),
            duration,
        }
    }
}

impl Coroutine<()> for SleepCoroutine {
    type Yield = ();
    type Return = ();

    fn resume(self: Pin<&mut Self>, _: ()) -> CoroutineState<Self::Yield, Self::Return> {
        if self.start.elapsed() >= self.duration {
            CoroutineState::Complete(())
        } else {
            CoroutineState::Yielded(())
        }
    }
}

impl Future for SleepCoroutine {
    type Output = ();
    fn poll(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        match Pin::new(&mut self).resume(()) {
            CoroutineState::Complete(_) => Poll::Ready(()),
            CoroutineState::Yielded(_) => {
                cx.waker().wake_by_ref();
                Poll::Pending
            }
        }
    }
}

#[allow(dead_code)]
struct Executor {
    coroutines: VecDeque<CoroutineHandle>,
}

#[allow(dead_code)]
impl Executor {
    fn new() -> Self {
        Self {
            coroutines: VecDeque::new(),
        }
    }
    fn add(&mut self, coroutine: CoroutineHandle) {
        self.coroutines.push_back(coroutine);
    }
    fn poll(&mut self) {
        println!("Polling {} coroutines", self.coroutines.len());
        let mut coroutine = self.coroutines.pop_front().unwrap();
        match coroutine.as_mut().resume(()) {
            CoroutineState::Yielded(_) => {
                self.coroutines.push_back(coroutine);
            }
            CoroutineState::Complete(_) => {}
        }
    }
}

fn main() {
    // let mut manager = CoroutineManager::new("numbers.txt", "output.txt").unwrap();
    // manager.run();

    // call a Coroutine from a Coroutine
    let mut reader = ReadCoroutine::new("numbers.txt").unwrap();
    let mut writer = WriteCoroutine::new("output.txt");
    loop {
        let number = Pin::new(&mut reader).resume_with_input(());
        if let Some(num) = number {
            Pin::new(&mut writer).resume_with_input(num);
        } else {
            break;
        }
    }

    // // Mimicking Async Behaviour with Coroutines
    // let mut sleep_coroutines = VecDeque::new();
    // sleep_coroutines.push_back(SleepCoroutine::new(Duration::from_secs(1)));
    // sleep_coroutines.push_back(SleepCoroutine::new(Duration::from_secs(1)));
    // sleep_coroutines.push_back(SleepCoroutine::new(Duration::from_secs(1)));

    // let mut counter = 0;
    // let start = Instant::now();

    // while counter < sleep_coroutines.len() {
    //     let mut coroutine = sleep_coroutines.pop_front().unwrap();
    //     match Pin::new(&mut coroutine).resume(()) {
    //         CoroutineState::Yielded(_) => {
    //             sleep_coroutines.push_back(coroutine);
    //         }
    //         CoroutineState::Complete(_) => {
    //             counter += 1;
    //         }
    //     }
    // }
    // println!("Took {:?}", start.elapsed());

    // Using Executor
    let mut executor = Executor::new();

    for _ in 0..3 {
        let coroutine = SleepCoroutine::new(Duration::from_secs(1));
        executor.add(Box::pin(coroutine));
    }
    let start = Instant::now();
    while !executor.coroutines.is_empty() {
        executor.poll();
    }
    println!("Took {:?}", start.elapsed());
}
