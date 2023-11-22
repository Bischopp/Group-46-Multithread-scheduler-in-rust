use names::{Generator, Name};
use uuid::Uuid;
use rayon::prelude::*;

use crate::task_deque::Queue;
use crate::task::Task;

pub struct WorkerThread<T> {
    pub id: Uuid,
    pub channel: String,
    pub queue: Queue<T>,
    pub active: bool,
}

impl<T> Default for WorkerThread<T>
where
    T: Task + Ord + std::marker::Send + 'static,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> WorkerThread<T>
where
    T: Task + Ord + std::marker::Send + 'static,
{
    fn generate_name() -> String {
        let mut generator = Generator::with_naming(Name::Numbered);
        generator.next().unwrap()
    }

    pub fn init(id: Option<Uuid>, channel: Option<String>, queue: Option<Queue<T>>) -> Self {
        let id = match id {
            Some(id) => id,
            None => Uuid::new_v4(),
        };

        let channel = match channel {
            Some(channel) => channel,
            None => Self::generate_name(),
        };

        let queue = match queue {
            Some(queue) => queue,
            None => Queue::new(),
        };

        Self {
            id,
            channel,
            active: true,
            queue,
        }
    }

    pub fn new() -> Self {
        Self::init(None, None, None)
    }

    pub fn from(vec: Vec<T>) -> Self {
        let queue = Queue::from(vec);

        Self::init(None, None, Some(queue))
    }

    pub fn fork(&mut self) {
        let worker_thread = std::mem::take(self);

        rayon::spawn(move || {
            let tasks: Vec<_> = worker_thread.queue.into_sorted_vec();

            rayon::join(|| {
                // Process all tasks in parallel
                tasks.into_par_iter().for_each(|mut item| {
                    rayon::spawn(move || item.process());
                });
            }, || {
                // Placeholder closure, you can add another operation here if needed
            });
        });
    }

    pub fn wait(&mut self) {
        self.active = false;
    }

    pub fn assign_one(&mut self, task: T) {
        self.queue.push(task);
        self.fork();
    }

    pub fn assign_many(&mut self, vec: Vec<T>) {
        for task in vec {
            self.queue.push(task);
        }
        self.fork();
    }
}