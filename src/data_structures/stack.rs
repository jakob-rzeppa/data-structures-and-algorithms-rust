use std::mem;

struct Node<T> {
    next: Option<Box<Node<T>>>,
    data: T,
}

impl<T> Node<T> {
    fn new(data: T) -> Self {
        Self {
            data,
            next: None,
        }
    }
}

struct Stack<T> {
    head: Option<Box<Node<T>>>,
}

impl<T: Clone> Stack<T> {
    fn new() -> Self {
        Self {
            head: None
        }
    }

    fn as_vec(&self) -> Vec<T> {
        let mut vec: Vec<T> = Vec::new();

        let mut head = self.head.as_ref();
        while let Some(node) = head {
            vec.push(node.data.clone());
            head = node.next.as_ref();
        }

        vec
    }

    fn push(&mut self, data: T) {
        let tmp = mem::replace(&mut self.head, Some(Box::new(Node::new(data))));

        self.head.as_mut().unwrap().next = tmp;
    }

    fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            let data = node.data.clone();
            self.head = node.next;
            data
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn basic() {
        let mut stack: Stack<i32> = Stack::new();

        stack.push(1);
        stack.push(2);
        stack.push(3);

        assert_eq!(stack.as_vec(), vec![3, 2, 1]);

        stack.pop();

        assert_eq!(stack.as_vec(), vec![2, 1]);

        stack.pop();
        stack.pop();

        assert_eq!(stack.as_vec(), vec![]);
    }
}