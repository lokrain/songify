//! EventRing:
//! - Simple ring buffer for T.
//! - Not thread-safe; caller enforces single producer/consumer if needed.

use alloc::vec::Vec;

/// Generic ring buffer.
///
/// API is minimal and explicit; no hidden blocking or atomics.
#[derive(Debug)]
pub struct EventRing<T> {
    buf: Vec<Option<T>>,
    head: usize,
    tail: usize,
    len: usize,
}

impl<T> EventRing<T> {
    pub fn with_capacity(cap: usize) -> Self {
        assert!(cap > 0, "capacity must be > 0");
        let mut buf = Vec::with_capacity(cap);
        for _ in 0..cap {
            buf.push(None);
        }
        Self { buf, head: 0, tail: 0, len: 0 }
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    #[must_use]
    pub fn is_full(&self) -> bool {
        self.len == self.buf.len()
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn push(&mut self, value: T) -> Result<(), T> {
        if self.is_full() {
            return Err(value);
        }
        self.buf[self.tail] = Some(value);
        self.tail = (self.tail + 1) % self.buf.len();
        self.len += 1;
        Ok(())
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        let v = self.buf[self.head].take();
        self.head = (self.head + 1) % self.buf.len();
        self.len -= 1;
        v
    }
}
