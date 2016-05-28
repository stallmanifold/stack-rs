use super::stack::Stack;


#[test]
fn test_stack_empty() {
    let mut stack: Stack<usize> = Stack::new();

    assert!(stack.is_empty());
    assert!(stack.pop().is_none());
    assert!(stack.pop().is_none());
    assert!(stack.pop().is_none());
}

#[test]
fn test_stack_operations() {
    let vec1     = vec![1,2,3,4,5,6,7,8,9,10];
    let vec2     = vec![10,9,8,7,6,5,4,3,2,1];
    let mut vec3 = vec![];

    let mut stack: Stack<usize> = Stack::new();
    for item in vec1.into_iter() {
        stack.push(item);
    }

    assert!(stack.peek().is_some());
    assert!(!stack.is_empty());

    loop {
        let item = stack.pop();

        match item {
            None      => break,
            Some(val) => vec3.push(val), 
        }
    }

    assert_eq!(vec2, vec3);
    assert!(stack.is_empty());
}