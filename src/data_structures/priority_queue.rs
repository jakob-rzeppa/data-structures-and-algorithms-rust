use std::collections::VecDeque;

#[derive(Default)]
pub struct PriorityQueue<E: PartialOrd> {
    data: VecDeque<E>,
}

impl<E: PartialOrd> PriorityQueue<E> {
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn enque(&mut self, element: E) {
        // Find index
        let index = self.data.iter().position(|e| e > &element);

        self.data.push_front(element);

        // Swap to the right index
        for i in 0..index.unwrap_or(self.data.len() - 1) {
            self.data.swap(i, i + 1);
        }
    }

    pub fn deque(&mut self) -> Option<E> {
        self.data.pop_front()
    }
}

#[cfg(test)]
mod tests {
    use crate::data_structures::priority_queue::PriorityQueue;

    #[test]
    fn test_priority_queue() {
        let mut priority_queue = PriorityQueue::default();

        priority_queue.enque(5);
        priority_queue.enque(3);
        priority_queue.enque(4);
        priority_queue.enque(1);
        priority_queue.enque(2);

        assert_eq!(priority_queue.deque(), Some(1));
        assert_eq!(priority_queue.deque(), Some(2));
        assert_eq!(priority_queue.deque(), Some(3));
        assert_eq!(priority_queue.deque(), Some(4));
        assert_eq!(priority_queue.deque(), Some(5));
        assert_eq!(priority_queue.deque(), None);
    }
}
