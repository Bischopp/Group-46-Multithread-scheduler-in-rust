# Rust Work-Stealing Fibonacci Benchmark

## Overview

This project implements a work-stealing algorithm to compare the performance of two different languages in generating Fibonacci numbers from the range 26 to 46. The comparison is made between Rust and C++ using the implemented algorithm.

The project structure consists of the following modules:
- `task_deque`: Implements a work-stealing task deque, including a `WorkerThread` that manages a task queue.
- `task`: Defines a trait `Task` that represents tasks to be executed.
- `worker_thread`: Contains the implementation of a worker thread that utilizes the work-stealing algorithm.

## Modules

### `task_deque`

#### `WorkerThread<T>`

- Represents a worker thread that performs tasks from a task queue.
- Each worker thread has a unique identifier (`id`) and a channel name (`channel`).
- The `WorkerThread` is designed to be generic over types implementing the `Task` trait.
- Worker threads can be forked to process tasks in parallel.
- Worker threads can be set to wait, making them inactive until explicitly reactivated.
- Provides methods to assign one or many tasks to the task queue.

### `task`

#### `Task` trait

- Defines a trait for tasks that can be processed by the worker threads.
- Requires implementing a `process` method for task execution.

### `task_deque`

#### `Queue<T>` and `BinHeapQueue<T>`

- Implement task queues using `VecDeque` and `BinaryHeap` respectively.
- Queues have unique identifiers (`id`) and support various operations like push, pop, peek, and sorting.
- Both queues can be initialized with a vector of tasks, and their capacity can be configured.

## Building the Project

To build the project, ensure you have the necessary dependencies installed, including the Rust compiler.

```bash
# Clone the repository
git clone https://github.com/your-username/fibonacci-benchmark.git
cd fibonacci-benchmark

# Build the project
cargo build --release
POPL Concepts Used include:

Ownership System:
Rust has a unique ownership system that includes ownership, borrowing, and lifetimes, ensuring memory safety without the need for a garbage collector.
In Rust, each value has a variable that is its "owner," and the ownership system prevents data races.

Concurrency Model:
Rust has a strong focus on safe concurrency and memory safety. The code uses the rayon crate for parallel processing, demonstrating Rust's approach to safe parallelism.

Pattern Matching and Enums:
Rust uses enums and pattern matching extensively, providing a powerful way to express and handle different states or options.
In the task_deque module, there are options for id, channel, and queue in the WorkerThread initialization, showcasing Rust's expressive enum-like options.

Trait System:
Rust's trait system is used to define and implement the Task trait, ensuring that types implementing the trait have a process method.

UUID Generation:
Rust uses the uuid crate for generating UUIDs. This is a third-party crate that provides a Rust-specific implementation for UUID generation.

Error Handling:
Rust encourages explicit error handling using the Result type. In the provided code, the unwrap method is used, which will panic if an error occurs. In a production setting, more robust error handling may be preferred.


Running the Code

To run the code, follow these steps:

Enter the directory code-external/benchmarks/sequential/fib
Compile and create an executable for main.cpp:
g++ -o exec ./main.cpp

Receive outputs for the Fibonacci numbers from 26 to 45. The runtime is almost 100 times slower compared to the Rust implementation.

For the Rust implementation, run the command:
cargo build


Cited references:

Reference for C++ implementation: https://github.com/rkuchumov/staccato
Reference for Rust implementation: https://github.com/rayon-rs/rayon/blob/master/

#Contributors (in alphabetical order):
Ishi Nigam
Jimit Shah
Mudit Jaju
Rishabh Raj
