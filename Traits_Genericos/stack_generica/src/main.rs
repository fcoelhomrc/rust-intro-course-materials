// Stack Genérica:
// Utilizar as estruturas de dados faladas para implementar uma stack genérica.

// Stack
// -> Last in, first out
// -> Only elements of same type, but this type is generic


struct Stack<T> {
    contents: Vec<T>,
}
impl<T> Stack<T> {
    fn new() -> Self {
        Self { contents: Vec::new() }
    }

    fn pop(&mut self) -> Option<T> {
        self.contents.pop()
    }

    fn push(&mut self, value: T) {
        self.contents.push(value)
    }

    fn peek(&self) -> Option<&T> {
        self.contents.last()
    }

    fn len(&self) -> usize {
        self.contents.len()
    }

}

#[derive(Debug)]
#[derive(PartialEq)]
struct MyCustomObject {
    value: i32,
}

impl MyCustomObject {
    fn new(x: i32) -> Self {
        Self { value: x }
    }
}



fn main() {
    todo!()
}

#[cfg(test)]
mod test {
    use crate::{MyCustomObject, Stack};

    #[test]
    fn test_stack_integer() {
        let mut stack = Stack::<i32>::new();

        assert_eq!(stack.peek(), None);
        assert_eq!(stack.pop(), None);

        stack.push(1);
        assert_eq!(stack.peek(), Some(&1));
        assert_eq!(stack.len(), 1);

        stack.push(2);
        assert_eq!(stack.peek(), Some(&2));
        assert_eq!(stack.len(), 2);

        stack.push(3);
        assert_eq!(stack.peek(), Some(&3));
        assert_eq!(stack.len(), 3);

        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
    }

    #[test]
    fn test_stack_float() {
        let mut stack = Stack::<f64>::new();

        assert_eq!(stack.peek(), None);
        assert_eq!(stack.pop(), None);

        stack.push(1.0);
        assert_eq!(stack.peek(), Some(&1.0));
        assert_eq!(stack.len(), 1);

        stack.push(2.0);
        assert_eq!(stack.peek(), Some(&2.0));
        assert_eq!(stack.len(), 2);

        stack.push(3.0);
        assert_eq!(stack.peek(), Some(&3.0));
        assert_eq!(stack.len(), 3);

        assert_eq!(stack.pop(), Some(3.0));
        assert_eq!(stack.pop(), Some(2.0));
        assert_eq!(stack.pop(), Some(1.0));
    }
    #[test]
    fn test_stack_str() {
        let mut stack = Stack::<&str>::new();

        assert_eq!(stack.peek(), None);
        assert_eq!(stack.pop(), None);

        stack.push("A");
        assert_eq!(stack.peek(), Some(&"A"));
        assert_eq!(stack.len(), 1);

        stack.push("B");
        assert_eq!(stack.peek(), Some(&"B"));
        assert_eq!(stack.len(), 2);

        stack.push("C");
        assert_eq!(stack.peek(), Some(&"C"));
        assert_eq!(stack.len(), 3);

        assert_eq!(stack.pop(), Some("C"));
        assert_eq!(stack.pop(), Some("B"));
        assert_eq!(stack.pop(), Some("A"));
    }
    
    #[test]
    fn test_stack_string() {
        let mut stack = Stack::<String>::new();

        assert_eq!(stack.peek(), None);
        assert_eq!(stack.pop(), None);

        stack.push("A".to_string());
        assert_eq!(stack.peek(), Some(&"A".to_string()));
        assert_eq!(stack.len(), 1);

        stack.push("B".to_string());
        assert_eq!(stack.peek(), Some(&"B".to_string()));
        assert_eq!(stack.len(), 2);

        stack.push("C".to_string());
        assert_eq!(stack.peek(), Some(&"C".to_string()));
        assert_eq!(stack.len(), 3);

        assert_eq!(stack.pop(), Some("C".to_string()));
        assert_eq!(stack.pop(), Some("B".to_string()));
        assert_eq!(stack.pop(), Some("A".to_string()));
    }

    #[test]
    fn test_stack_custom() {
        let mut stack = Stack::<MyCustomObject>::new();

        assert_eq!(stack.peek(), None);
        assert_eq!(stack.pop(), None);

        stack.push(MyCustomObject::new(1));
        assert_eq!(stack.peek(), Some(&MyCustomObject::new(1)));
        assert_eq!(stack.len(), 1);

        stack.push(MyCustomObject::new(2));
        assert_eq!(stack.peek(), Some(&MyCustomObject::new(2)));
        assert_eq!(stack.len(), 2);

        stack.push(MyCustomObject::new(3));
        assert_eq!(stack.peek(), Some(&MyCustomObject::new(3)));
        assert_eq!(stack.len(), 3);

        assert_eq!(stack.pop(), Some(MyCustomObject::new(3)));
        assert_eq!(stack.pop(), Some(MyCustomObject::new(2)));
        assert_eq!(stack.pop(), Some(MyCustomObject::new(1)));
    }
}