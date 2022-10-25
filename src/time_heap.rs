#![allow(dead_code)]

use std::cmp::Ordering;
use std::collections::BinaryHeap;

// A u32 that stores a unix timestamp
pub type T32 = u32;

pub fn connect() {
    println!("Don't way a tonne")
}

/// Currently only supports one request per unique Instant
pub struct TimingHeap {
    heap: BinaryHeap<Request>,
}

#[derive(Eq)]
struct RequestContainer {
    time: u32,
    events: Vec<Request>
}

impl Ord for RequestContainer {
    fn cmp(&self, other: &Self) -> Ordering {
        self.time.cmp(&other.time)
    }
}

impl PartialOrd for RequestContainer {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for RequestContainer {
    fn eq(&self, other: &Self) -> bool {
        self.time == other.time
    }
}

#[derive(Eq)]
pub struct Request {
    utime: T32,
    ntype: Type,
}

impl Request {
    pub fn new(at_time: T32) -> Self {
        Request {
            utime: at_time, 
            ntype: Type::Email
        }
    }
}
#[derive(Eq, PartialEq, PartialOrd)]
pub enum Type {
    Email,
    SMS,
}

impl Ord for Request {
    fn cmp(&self, other: &Self) -> Ordering {
        self.utime.cmp(&other.utime)
    }
}

impl PartialOrd for Request {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Request {
    fn eq(&self, other: &Self) -> bool {
        self.utime == other.utime
    }
}

impl TimingHeap {
    pub fn new() -> Self {
        TimingHeap {
            heap: BinaryHeap::new(),
        }
    }

    pub fn add(&mut self, req: Request) {
        self.heap.push(req)
    }

    pub fn peak(&self) -> Option<&Request> {
        self.heap.peek()
    }

    pub fn pop(&mut self) -> Option<Request> {
        self.heap.pop()
    }
}
