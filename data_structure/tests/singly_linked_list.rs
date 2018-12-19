extern crate data_structure;

use data_structure::SinglyLinkedList;

#[test]
fn test_next() {
    let mut list = SinglyLinkedList::new();
    let head = &mut list;
    for i in 0..10 {
        head.insert(i);
    }
    let mut cursor = &mut list;
    for _ in 0..10 {
        assert_eq!(true, cursor.next().is_some());
        cursor = cursor.next_mut().unwrap();
    }
    assert_eq!(false, cursor.next().is_some());
    assert_eq!(false, cursor.next_mut().is_some());
}

#[test]
fn test_insert() {
    let mut list = SinglyLinkedList::new();
    let head = &mut list;
    for i in 0..10 {
        head.insert(i);
    }
    assert_eq!(
        "(start)->(9)->(8)->(7)->(6)->(5)->(4)->(3)->(2)->(1)->(0)",
        format!("{:?}", list)
    );
}

#[test]
fn test_delete() {
    let mut list = SinglyLinkedList::new();
    let head = &mut list;
    for i in 0..10 {
        head.insert(i);
    }
    let mut cursor = &mut list;
    cursor.delete();
    cursor = cursor.next_mut().unwrap();
    cursor = cursor.next_mut().unwrap();
    cursor = cursor.next_mut().unwrap();
    cursor.delete();
    cursor = cursor.next_mut().unwrap();
    cursor.delete();
    cursor = cursor.next_mut().unwrap();
    cursor = cursor.next_mut().unwrap();
    cursor = cursor.next_mut().unwrap();
    cursor.delete();
    assert_eq!(
        "(start)->(8)->(7)->(6)->(4)->(2)->(1)->(0)",
        format!("{:?}", list)
    );
}
