use std::{vec, slice::{Iter, IterMut}, ops::Index};

#[derive(Debug)]
#[derive(Clone)]
struct Person<'a> {
    first: &'a str,
    last: &'a str,
    age: u32
}

impl Person<'_> {
    fn new<'b>(first: &'b str, last: &'b str, age: u32) -> Person<'b> {
        Person {
            first: first,
            last: last,
            age: age
        }
    }

    fn full_name(&self) -> String {
        format!("{} {}", self.first, self.last)
    }
}


#[derive(Debug)]
struct Stack<T> {
    data: Vec<T>,
    size: usize
}

impl<T: Clone> Stack<T> {
    fn new(arr: &[T]) -> Self {
        Stack {
            data: arr.to_vec(),
            size: arr.len()
        }
    }

    fn push(&mut self, item: T) {
        self.data.push(item);
        self.size += 1;
    }

    fn pop(&mut self) -> T {
        self.size -= 1;
        self.data.pop().unwrap()
    }

    fn peek(&self) -> &T {
        &self.data[self.size - 1]
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn iter(&self) -> Iter<'_, T> {
        self.data.iter()
    }

    fn iter_mut(&mut self) -> IterMut<'_, T> {
        self.data.iter_mut()
    }
}

impl<T, const N: usize> From<[T; N]> for Stack<T> {
    fn from(arr: [T; N]) -> Stack<T> {
        Stack {
            data: vec::Vec::from(arr),
            size: N
        }
    }
}

macro_rules! stack {
    [$($e:expr),*] => {
        Stack::from($($e,)*)
    };
}

fn main() {
    let p1 = Person::new("Will", "Zhang", 18);
    let p2 = Person::new("Bill", "Gates", 100);
    let p3 = Person::new("Tyler", "1", 26);

    let mut stack = stack!([p1, p2, p3]);
    stack.push(Person::new("Venti", "Nnnghhhh", 1000));

    for x in stack.iter_mut() {
        x.age *= 2;
        println!("{:?}", x);
    }

    let mut stack2: Stack<char> = stack!([]);
    let test = "(())[]";


    let open = ['(', '[', '{'];
    let closed = [')', ']', '}'];

    let mut balanced: bool = true;

    for c in test.chars() {
        if open.contains(&c) {
            stack2.push(c);
        } else {
            let idx = closed.iter().position(|&r| r == c).unwrap();
            if stack2.is_empty() || stack2.pop() != open[idx] {
                balanced = false;
                break;
            }
        }
    }

    if !stack2.is_empty() {
        balanced = false;
    }

    println!("{} is {} balanced!", test, if balanced { "" } else { "not" });
}
