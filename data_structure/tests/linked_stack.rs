extern crate data_structure;

use data_structure::LinkedStack;

#[test]
fn test_push() {
    let mut stack = LinkedStack::new();
    for i in 0..10 {
        stack.push(i);
    }
    assert_eq!(
        "len(10) : (start)->(0)->(1)->(2)->(3)->(4)->(5)->(6)->(7)->(8)->(9)",
        format!("{:?}", stack)
    );
}

#[test]
fn test_pop() {
    let mut stack = LinkedStack::new();
    for i in 0..10 {
        stack.push(i);
    }
    assert_eq!(Some(9), stack.pop());
    stack.pop();
    stack.pop();
    assert_eq!(Some(6), stack.pop());
    stack.pop();
    stack.pop();
    stack.pop();
    assert_eq!(Some(2), stack.pop());
    stack.pop();
    stack.pop();
    assert_eq!(None, stack.pop());
}
