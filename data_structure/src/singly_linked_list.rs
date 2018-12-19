use std::fmt::Display;

pub struct SinglyLinkedList<T>
where
    T: Display + Default,
{
    pub data: T,
    pub next: Option<Box<SinglyLinkedList<T>>>,
}

impl<T> SinglyLinkedList<T>
where
    T: Display + Default,
{
    pub fn new() -> SinglyLinkedList<T> {
        SinglyLinkedList {
            data: T::default(),
            next: None,
        }
    }

    pub fn next(&self) -> Option<&SinglyLinkedList<T>> {
        match self.next {
            Some(ref next) => Some(next),
            _ => None,
        }
    }

    pub fn next_mut(&mut self) -> Option<&mut SinglyLinkedList<T>> {
        match self.next {
            Some(ref mut next) => Some(next),
            _ => None,
        }
    }

    /// insert a new node after cursor
    pub fn insert(&mut self, data: T) {
        let mut node = Box::new(SinglyLinkedList {
            data: data,
            next: None,
        });
        std::mem::swap(&mut self.next, &mut node.next);
        self.next = Some(node);
    }

    /// delete node after cursor
    pub fn delete(&mut self) {
        let mut next = None;
        if self.next().is_some() {
            std::mem::swap(&mut self.next, &mut next);
            std::mem::swap(&mut self.next, &mut next.unwrap().next);
        }
    }
}

impl<T> std::fmt::Debug for SinglyLinkedList<T>
where
    T: Display + Default,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut cursor = self;
        write!(f, "(start)")?;
        while let Some(node) = cursor.next() {
            write!(f, "->({})", node.data)?;
            cursor = node;
        }
        Ok(())
    }
}

#[cfg(debug_assertions)]
impl<T> Drop for SinglyLinkedList<T>
where
    T: Display + Default,
{
    fn drop(&mut self) {
        println!("drop {}", self.data);
    }
}
