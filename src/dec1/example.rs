use ringbuf::RingBuffer;
use ringbuf::Producer;
use ringbuf::Consumer;

pub struct WindowOperator {
    ring_buffer : RingBuffer<u32>,
    producer : Producer<u32>,
    consumer : Consumer<u32>,
}

impl WindowOperator {
    pub fn new(window_size : usize) -> WindowOperator {
        ring_buffer : RingBuffer::<u32>::new(window_size),
        (producer, consumer ) = ring_buffer.split(),
    }

    pub fn add_and_evaluate(&mut self, item : u32) -> u32 {
        if self.producer.is_full() {
            self.consumer.pop();
        }
        self.producer.push(item).unwrap();
        let mut window_count = 0;
        self.consumer.for_each(|queued_item : &u32| {
            window_count += queued_item;
        });
        window_count;
    }
}



