

pub struct RingBuffer<T, const N: usize>
{
    buffer: [T; N],
    head: usize,
    tail: usize,
}


impl<T: Default + Copy, const N: usize> RingBuffer<T, N> {
    pub fn new() -> Self {
        Self {
            buffer: [T::default(); N],
            head: 0,
            tail: 0,
        }
    }

    pub fn store_value(&mut self, v: T) {
        let i = self.next_index(self.head);
        if i != self.tail {
            self.buffer[self.head] = v;
            self.head = i;
        }
    }

    pub fn clear(&mut self) {
        self.head = 0;
        self.tail = 0;
    }
    
    pub fn read(&mut self) -> Option<T> {
        if self.tail == self.head {
            None
        } else {
            let v = self.buffer[self.tail];
            self.tail = self.next_index(self.tail);
            Some(v)
        }
    }

    fn next_index(&self, index: usize) -> usize {
        (index + 1) % N
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn ring_buffer_new() {
        let ring_buffer: RingBuffer::<u8, 10> = RingBuffer::new();
        
    }
}
