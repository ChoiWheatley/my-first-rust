#![allow(unused)]
// be aware of accidently using borrow::BorrowMut!
// It is implemented via "blanket implementations" both
// `Rc` and `RefCell`, which can cause infinite `borrow_mut()` loop!
// use std::borrow::BorrowMut;
use std::cell::{Ref, RefCell};
use std::rc::Rc;
/// Let's make doubly-linked deque!!!
///

pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
    prev: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {
            head: None,
            tail: None,
        }
    }

    pub fn push_front(&mut self, elem: T) {
        let new_head = Node::new(elem);
        match self.head.take() {
            Some(old_head) => {
                // non-empty list, need to connect the old_head
                old_head.borrow_mut().prev = Some(new_head.clone());
                new_head.borrow_mut().next = Some(old_head);
                self.head = Some(new_head);
            }
            None => {
                // empty list, need to set the tail
                self.tail = Some(new_head.clone());
                self.head = Some(new_head);
            }
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    // not emptying list
                    new_head.borrow_mut().prev.take();
                    self.head = Some(new_head); // replace head to a new one
                }
                None => {
                    // emptying list
                    self.tail.take();
                }
            }
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().elem
        })
    }

    /// We cannot hand over `Option<&T>`, because `RefCell::borrow()` will be destroied
    /// when this function returns. So we have no choice to return the return type of itself: `Ref`
    ///
    /// In order to hide `Node` to users, we have to convert `Option<Ref<Node<T>>>` to `Option<Ref<T>>`,
    /// which can be done by `Ref::map`, which makes new Ref for borrowed data.
    pub fn peek_front(&self) -> Option<Ref<T>> {
        self.head
            .as_ref()
            .map(|rccell_node| Ref::map(rccell_node.borrow(), |node| &node.elem))
    }
}

impl<T> Node<T> {
    fn new(elem: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            elem,
            next: None,
            prev: None,
        }))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop_front(), None);

        // Populate list
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        // Check normal removal
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push_front(4);
        list.push_front(5);

        // Check normal removal
        assert_eq!(list.pop_front(), Some(5));
        assert_eq!(list.pop_front(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn peek() {
        let mut list = List::new();
        assert!(list.peek_front().is_none());
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        assert_eq!(&*list.peek_front().unwrap(), &3);
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(&*list.peek_front().unwrap(), &2);
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(&*list.peek_front().unwrap(), &1);
        assert_eq!(list.pop_front(), Some(1));
    }
}
