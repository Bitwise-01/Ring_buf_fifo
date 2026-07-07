#[derive(Debug)]
pub struct RingBuffer<T> {
    buf: Vec<T>,
    head: usize,
    tail: usize,
    count: usize,
    size: usize,
}

impl<T: Default> RingBuffer<T> {
    pub fn new(size: usize) -> Self {
        let mut buf = Vec::with_capacity(size);
        // This tells Rust: "Fill the slots by running this closure () -> None"
        buf.resize_with(size, || T::default());

        Self {
            buf,
            head: 0,
            tail: 0,
            count: 0,
            size,
        }
    }

    pub fn push(&mut self, value: T) {
        self.buf[self.head] = value;
        self.head = (self.head + 1) % self.size;

        if self.count < self.size {
            self.count += 1;
        } else {
            self.tail = (self.tail + 1) % self.size;
        }
    }

    pub fn pull(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        self.count -= 1;

        let old_value = std::mem::take(&mut self.buf[self.tail]);
        self.tail = (self.tail + 1) % self.size;
        Some(old_value)
    }

    pub fn peek(&self) -> Option<&T> {
        if self.is_empty() {
            return None;
        }

        Some(&self.buf[self.tail])
    }

    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    pub fn is_full(&self) -> bool {
        self.count >= self.size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Each test must have the #[test] attribute above it
    #[test]
    fn test_push_and_pull() {
        let mut rb = RingBuffer::new(3);

        rb.push(10);
        rb.push(20);

        // assert_eq! checks if the two values are exactly equal
        assert_eq!(rb.pull(), Some(10));
        assert_eq!(rb.pull(), Some(20));
        assert_eq!(rb.pull(), None); // Buffer should be empty now
    }

    #[test]
    fn test_peek_does_not_consume() {
        let mut rb = RingBuffer::new(3);
        rb.push(100);

        // Peeking should let us see the value via reference
        assert_eq!(rb.peek(), Some(&100));
        // Peeking a second time should still return the same value
        assert_eq!(rb.peek(), Some(&100));
        // Pulling should still work perfectly afterward
        assert_eq!(rb.pull(), Some(100));
    }

    #[test]
    fn test_is_empty_and_is_full() {
        let mut rb = RingBuffer::<f32>::new(2);

        assert!(rb.is_empty());
        assert!(!rb.is_full());

        rb.push(1.0);
        assert!(!rb.is_empty());
        assert!(!rb.is_full());

        rb.push(2.0);
        assert!(!rb.is_empty());
        assert!(rb.is_full());
    }
}
