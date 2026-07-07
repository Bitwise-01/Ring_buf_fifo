use ring_buf_fifo::RingBuffer;

fn main() {
    let mut rb = RingBuffer::new(5);
    let letters = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

    // load
    // for i in 0..5 {
    //     rb.push(i);
    // }

    for c in letters.chars() {
        rb.push(c);
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
