# Ring Buf Fifo
A Ring buffer lib implementation in Rust. This is just for practice.


### Example 

```rust
use ring_buf_fifo::RingBuffer;

fn main() {
    let mut rb = RingBuffer::new(5);
   
    // load
    for i in 0..5 {
        rb.push(i);
    }

    // print
    for _ in 0..6 {
        if let Some(peeked) = rb.peek() {
            println!("Peek: {}", peeked);
        }
        if let Some(value) = rb.pull() {
            println!("{}", value);
        }
    }
}
```