/// src: https://rust-unofficial.github.io/too-many-lists/first.html
///
///
use std::mem;

struct Node {
    elem: i32,
    next: Link,
}

enum Link {
    Empty,
    Next(Box<Node>),
}

pub struct List {
    head: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem,
            next: mem::replace(&mut self.head, Link::Empty), // head becomes Empty, next becomes head
        });
        // now finally head links to older head now
        self.head = Link::Next(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        match &mut self.head {
            Link::Empty => None,
            Link::Next(next) => {
                let ret = next.elem;
                self.head = mem::replace(&mut next.next, Link::Empty);
                Some(ret)
            }
        }
    }
}

/// Box type doesn't care about recursive drop, which may leak memory
/// So we have to manually iterate through all links, via pointer
impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);
        while let Link::Next(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        list.push(4);
        list.push(5);

        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn drops() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);
        list.push(4);
        list.push(5);
        list.push(6);

        drop(list);
    }
}
