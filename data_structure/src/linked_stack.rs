use crate::SinglyLinkedList;
use std::fmt::Display;

pub struct LinkedStack<T>
where
    T: Default + Display,
{
    stack: SinglyLinkedList<T>,
    size: usize,
}

impl<T> LinkedStack<T>
where
    T: Default + Display,
{
    pub fn new() -> LinkedStack<T> {
        LinkedStack {
            stack: SinglyLinkedList::new(),
            size: 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    // TODO: can not self referenced cost O(n) time
    pub fn push(&mut self, data: T) {
        let mut cursor = &mut self.stack;
        while let Some(ref mut next) = cursor.next {
            cursor = next;
        }
        cursor.insert(data);
        self.size += 1;
    }

    // TODO: singly linked list cost O(2n-1) time
    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            let mut cursor = &mut self.stack;
            for _ in 0..self.size {
                cursor = cursor.next_mut().unwrap();
            }
            let mut data = T::default();
            std::mem::swap(&mut data, &mut cursor.data);

            let mut cursor = &mut self.stack;
            for _ in 0..self.size - 1 {
                cursor = cursor.next_mut().unwrap();
            }
            cursor.delete();
            self.size -= 1;

            Some(data)
        }
    }
}

impl<T> std::fmt::Debug for LinkedStack<T>
where
    T: Display + Default,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "len({}) : {:?}", self.size, self.stack)
    }
}
