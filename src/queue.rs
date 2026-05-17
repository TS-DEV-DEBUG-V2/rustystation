use std::collections::VecDeque;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Queue<T> {
    data: VecDeque<T>,
    capacity: usize,
}

impl<T> Queue<T> {
    #[inline]
    pub fn push(&mut self, value: T) {
        if !self.full() {
            self.data.push_back(value);
        }
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    #[inline(always)]
    pub fn has_data(&self) -> bool {
        !self.data.is_empty()
    }

    #[inline(always)]
    pub fn full(&self) -> bool {
        self.data.len() >= self.capacity
    }

    #[inline(always)]
    pub fn has_space(&self) -> bool {
        self.data.len() < self.capacity
    }

    #[inline]
    pub fn clear(&mut self) {
        self.data.clear();
    }

    #[inline]
    pub fn data(&mut self) -> &mut VecDeque<T> {
        &mut self.data
    }
}

impl Queue<u8> {
    pub fn new(capacity: usize) -> Queue<u8> {
        Queue {
            data: VecDeque::with_capacity(capacity),
            capacity,
        }
    }

    #[inline]
    pub fn pop(&mut self) -> u8 {
        self.data.pop_front().unwrap_or(0)
    }
}

impl Queue<u16> {
    pub fn new(capacity: usize) -> Queue<u16> {
        Queue {
            data: VecDeque::with_capacity(capacity),
            capacity,
        }
    }

    #[inline]
    pub fn pop(&mut self) -> u16 {
        self.data.pop_front().unwrap_or(0)
    }
}

impl Queue<u32> {
    pub fn new(capacity: usize) -> Queue<u32> {
        Queue {
            data: VecDeque::with_capacity(capacity),
            capacity,
        }
    }

    #[inline]
    pub fn pop(&mut self) -> u32 {
        self.data.pop_front().unwrap_or(0)
    }
}
