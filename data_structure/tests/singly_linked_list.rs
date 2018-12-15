extern crate data_structure;

use data_structure::SinglyLinkedList;

#[test]
fn test_insert() {
    let mut list = SinglyLinkedList::new();
    let cursor = &mut list;
    for i in 0..10 {
        cursor.insert(i);
    }
    assert_eq!(
        "(start)->(9)->(8)->(7)->(6)->(5)->(4)->(3)->(2)->(1)->(0)",
        format!("{:?}", list)
    );
}

#[test]
fn test_delete() {
    let mut list = SinglyLinkedList::new();
    let cursor = &mut list;
    for i in 0..10 {
        cursor.insert(i);
    }
    let mut cursor = &mut list;
    cursor.delete();
    cursor = cursor.next_mut().unwrap();
    cursor = cursor.next_mut().unwrap();
    cursor = cursor.next_mut().unwrap();
    cursor.delete();
    cursor = cursor.next_mut().unwrap();
    cursor.delete();
    assert_eq!(
        "(start)->(8)->(7)->(6)->(4)->(2)->(1)->(0)",
        format!("{:?}", list)
    );
}
