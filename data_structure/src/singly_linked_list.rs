pub struct SinglyLinkedList {
    pub data: i64,
    pub next: Option<Box<SinglyLinkedList>>,
}

impl SinglyLinkedList {
    pub fn new() -> SinglyLinkedList {
        SinglyLinkedList {
            data: -1,
            next: None,
        }
    }

    pub fn next(&self) -> Option<&SinglyLinkedList> {
        if let Some(ref next) = self.next {
            Some(next)
        } else {
            None
        }
    }

    pub fn next_mut(&mut self) -> Option<&mut SinglyLinkedList> {
        if let Some(ref mut next) = self.next {
            Some(next)
        } else {
            None
        }
    }

    /// insert a new node after cursor
    pub fn insert(&mut self, data: i64) {
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
        std::mem::swap(&mut self.next, &mut next);
        std::mem::swap(&mut self.next, &mut next.unwrap().next);
    }
}

impl std::fmt::Debug for SinglyLinkedList {
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
impl Drop for SinglyLinkedList {
    fn drop(&mut self) {
        println!("drop {}", self.data);
    }
}
