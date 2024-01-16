struct Queue<T> {
    queue: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue { queue: Vec::new() }
    }

    pub fn enqueue(&mut self, data: T) {
        self.queue.push(data)
    }

    pub fn dequeue(&mut self) {
        self.queue.remove(0);
    }

    pub fn display(&mut self) {
        unimplemented!()
    }

    pub fn length(&mut self) -> usize {
        self.queue.len()
    }
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
}

#[cfg(test)]
mod test {

    use super::Queue;
    #[test]
    fn name() {
        let mut queue: Queue<i32> = Queue::new();
        assert_eq!(queue.is_empty(), true);
        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);
        queue.enqueue(4);
        assert_eq!(queue.length(), 4);
        queue.dequeue();
        assert_eq!(queue.length(), 3);
    }
}
