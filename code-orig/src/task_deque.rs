use std::collections::VecDeque;

use std::collections::BinaryHeap;

use uuid::Uuid;

pub struct Queue<T> {
    pub id: Uuid,
    deque: VecDeque<T>,
}

impl<T> Default for Queue<T>
where
    T: Ord,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Iterator for Queue<T>
where
    T: Ord,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.pop()
    }
}

impl<T> Queue<T>
where
    T: Ord,
{
    pub fn init(id: Uuid, vec: Vec<T>) -> Self {
        Self {
            id,
            deque: VecDeque::from(vec),
        }
    }

    pub fn new() -> Self {
        let id = Uuid::new_v4();
        let vec = Vec::new();

        Self::init(id, vec)
    }

    pub fn from(vec: Vec<T>) -> Self {
        let id = Uuid::new_v4();

        Self::init(id, vec)
    }

    pub fn with_capacity(capacity: usize) -> Self {
        let id = Uuid::new_v4();
        Self {
            id,
            deque: VecDeque::with_capacity(capacity),
        }
    }

    pub fn capacity(&self) -> usize {
        self.deque.capacity()
    }

    pub fn push(&mut self, value: T) {
        self.deque.push_back(value);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.deque.pop_front()
    }

    pub fn peek(&self) -> Option<&T> {
        self.deque.front()
    }

    pub fn into_sorted_vec(mut self) -> Vec<T> {
        let mut vec = Vec::new();
        while let Some(item) = self.deque.pop_front() {
            vec.push(item);
        }
        vec.sort();
        vec
    }

    pub fn len(&self) -> usize {
        self.deque.len()
    }

    pub fn is_empty(&self) -> bool {
        self.deque.is_empty()
    }
}

pub struct BinHeapQueue<T> {
    pub id: Uuid,
    heap: BinaryHeap<T>,
}

impl<T> Default for BinHeapQueue<T>
where
    T: Ord,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Iterator for BinHeapQueue<T>
where
    T: Ord,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.pop()
    }
}

impl<T> BinHeapQueue<T>
where
    T: Ord,
{
    pub fn init(id: Uuid, vec: Vec<T>) -> Self {
        Self {
            id,
            heap: BinaryHeap::from(vec),
        }
    }

    pub fn new() -> Self {
        let id = Uuid::new_v4();
        let vec = Vec::new();

        Self::init(id, vec)
    }

    pub fn from(vec: Vec<T>) -> Self {
        let id = Uuid::new_v4();

        Self::init(id, vec)
    }

    pub fn with_capacity(capacity: usize) -> Self {
        let id = Uuid::new_v4();
        Self {
            id,
            heap: BinaryHeap::with_capacity(capacity),
        }
    }

    pub fn capacity(&self) -> usize {
        self.heap.capacity()
    }

    pub fn push(&mut self, value: T) {
        self.heap.push(value)
    }

    pub fn pop(&mut self) -> Option<T> {
        self.heap.pop()
    }

    pub fn peek(&self) -> Option<&T> {
        self.heap.peek()
    }

    pub fn into_sorted_vec(self) -> Vec<T> {
        self.heap.into_sorted_vec()
    }

    pub fn len(&self) -> usize {
        self.heap.len()
    }

    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }
}
