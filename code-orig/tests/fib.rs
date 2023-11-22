use std::time::{Instant};
use wsqshed::task::Task;
use wsqshed::worker_thread::WorkerThread;
use std::fs::OpenOptions;
use std::io::{self, Write};
use csv::Writer;

#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Debug)]
struct Job<T> {
    value: T,
    func: fn(T),
}

impl<T> Job<T> {
    fn new(value: T, func: fn(T)) -> Self {
        Self { value, func }
    }
}

impl<T> Task for Job<T>
where
    T: Copy,
{
    fn process(&mut self) {
        (self.func)(self.value)
    }
}

fn print_res<T: std::fmt::Debug>(item: T) {
    println!("{:?}", item)
}

fn fibonacci(n: u64) -> u64 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return fibonacci(n - 1) + fibonacci(n - 2);
    }
}

fn main() {
    let start_time = Instant::now();

    let fib_numbers: Vec<Job<u64>> = (0..30)
        .map(|x| Job::new(fibonacci(x), print_res))
        .collect();

    let mut worker_thread = WorkerThread::from(Vec::from(fib_numbers));
    worker_thread.fork();
    worker_thread.wait();

    // Wait for the worker_thread to finish
    rayon::scope(|s| {
        s.spawn(|_| {
            worker_thread.wait();
        });
    });

    let elapsed_time = start_time.elapsed();

    // Write or append the runtime to the CSV file
    let mut file = OpenOptions::new().create(true).append(true).open("tests\tests.csv").expect("Error opening file");
    let mut writer = Writer::from_writer(file);
    writer.write_record(&[format!("{:?}", elapsed_time)]).expect("CSV write error");

    println!("Total runtime: {:?}", elapsed_time);
}
