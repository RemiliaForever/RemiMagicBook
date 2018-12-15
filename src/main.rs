use data_structure::SinglyLinkedList;

fn main() {
    let mut list = SinglyLinkedList::new();
    let cursor = &mut list;
    for i in 0..10 {
        cursor.insert(i);
        println!("insert {}", i);
    }
    let mut cursor = &list;
    while let Some(node) = cursor.next() {
        println!("{}", node.data);
        cursor = node;
    }
    let cursor = &mut list;
    while let Some(node) = cursor.next() {
        println!("delete {}", node.data);
        cursor.delete();
    }
}
