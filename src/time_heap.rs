#![allow(dead_code)]

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::time::Duration;

/// Currently only supports one request per unique Instant
pub struct TimingHeap {
    heap: BinaryHeap<Request>,
}

impl TimingHeap {
    pub fn new() -> Self {
        TimingHeap {
            heap: BinaryHeap::new(),
        }
    }

    pub fn iter(&self) -> std::collections::binary_heap::Iter<'_, Request> {
        self.heap.iter()
    }

    /// Add a request to the heap if the time has not passed.
    pub fn add(&mut self, req: Request) -> Result<(), TimeAddError> {
        self.heap.push(req);
        Ok(())
    }

    pub fn peak(&self) -> Option<&Request> {
        self.heap.peek()
    }

    pub fn pop(&mut self) -> Option<Request> {
        self.heap.pop()
    }
}

#[derive(Eq, PartialEq, Debug)]
pub struct Request {
    utime: Duration,
}

impl Request {
    pub fn new(utime: Duration) -> Self {
        Request { utime }
    }
}

impl Ord for Request {
    fn cmp(&self, other: &Self) -> Ordering {
        // TODO: Review altering order vs wrapped in reverse for min heap.
        self.utime
            .as_millis()
            .cmp(&other.utime.as_millis())
            .reverse()
    }
}

impl PartialOrd for Request {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.utime.cmp(&other.utime).reverse())
    }
}

// TODO: Add this to request
#[derive(Debug, Eq, PartialEq, PartialOrd)]
pub enum Type {
    Email,
    SMS,
}

#[derive(Debug)]
pub enum TimeAddError {
    TimeAlreadyPassed,
}

#[derive(Eq)]
struct RequestContainer {
    time: u32,
    events: Vec<Request>,
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

#[cfg(test)]
mod tests {
   use super::*;

    #[test]
    fn pops_as_min_heap() {
        let raw = [3, 1, 5, 8, 1, 2, 9];
        let mut heap = TimingHeap::new();

        for val in raw.iter() {
            heap.add(Request::new(Duration::from_secs(val.to_owned()))).unwrap();
        }

        let mut extracted: Vec<u64> = Vec::new();
        while let Some(x) = heap.pop() {
            extracted.push(x.utime.as_secs())
        }
        assert_eq!(extracted, vec![1, 1, 2, 3, 5, 8, 9]);
    }
}
