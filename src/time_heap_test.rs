#![allow(dead_code)]

use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Eq, PartialEq, Debug)]
pub struct TestR {
    utime: u64,
}

impl TestR {
    pub fn new(utime: u64) -> Self {
        Self { utime }
    }
}

impl Ord for TestR {
    fn cmp(&self, other: &Self) -> Ordering {
        self.utime.cmp(&other.utime).reverse()
    }
}

impl PartialOrd for TestR {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.utime.cmp(&other.utime).reverse())
    }
}

pub struct THeap {
    heap: BinaryHeap<TestR>,
}

impl THeap {
    pub fn new() -> Self {
        Self {
            heap: BinaryHeap::new(),
        }
    }

    pub fn iter(&self) -> std::collections::binary_heap::Iter<'_, TestR> {
        self.heap.iter()
    }

    /// Add a request to the heap if the time has not passed.
    pub fn add(&mut self, req: TestR) {
        self.heap.push(req);
    }

    pub fn pop(&mut self) -> Option<TestR> {
        self.heap.pop()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_it_min() {
        let raw = [3, 1, 5, 8, 1, 2, 9];
        let mut heap = THeap::new();

        for val in raw.iter() {
            heap.add(TestR::new(val.to_owned()));
        }

        let mut extracted: Vec<u64> = Vec::new();
        // let extracted: Vec<_> = heap.iter().map(|e| e.utime).collect();
        while let Some(x) = heap.pop() {
            extracted.push(x.utime)
        }
        assert_eq!(extracted, vec![1, 1, 2, 3, 5, 8, 9]);
    }
}
