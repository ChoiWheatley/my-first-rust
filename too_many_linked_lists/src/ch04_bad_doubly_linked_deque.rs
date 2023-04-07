#![allow(unused)]
// be aware of accidently using borrow::BorrowMut!
// It is implemented via "blanket implementations" both
// `Rc` and `RefCell`, which can cause infinite `borrow_mut()` loop!
// use std::borrow::BorrowMut;
use std::cell::{Ref, RefCell, RefMut};
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

pub struct IntoIter<T>(List<T>);

pub struct Iter<'a, T>(Option<Ref<'a, Node<T>>>);

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

    pub fn push_back(&mut self, elem: T) {
        let new_tail = Node::new(elem);
        match self.tail.take() {
            Some(old_tail) => {
                // non-empty list, need to connect the old_tail
                old_tail.borrow_mut().next = Some(new_tail.clone());
                new_tail.borrow_mut().prev = Some(old_tail);
                self.tail = Some(new_tail);
            }
            None => {
                // empty list, need to set the head
                self.head = Some(new_tail.clone());
                self.tail = Some(new_tail);
            }
        }
    }

    /// poping must be guarrenteed that only has one strong reference.
    ///
    /// Unfortunately, every node in doubly linked list has exactly two
    /// strong refs, which block us from implementing it.
    ///
    /// So, we have to `take` ownership from adjacent node so that we can
    /// guarantee only `head` or `tail` have the reference `front` or `back`
    /// element solely!
    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            match old_head.borrow_mut().next.take() // take strong reference of the newcomer
        {
                Some(new_head) => {
                    // not emptying list
                    new_head.borrow_mut().prev.take(); // emptying old one
                    self.head = Some(new_head); // replace head to a new one
                }
                None => {
                    // emptying list because when only one element left, both 
                    // head and tail have a reference of it.
                    self.tail.take();
                }
            }
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().elem
        })
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|old_tail| {
            match old_tail.borrow_mut().prev.take() {
                Some(new_tail) => {
                    // not emptying list
                    new_tail.borrow_mut().next.take(); // emptying old one
                    self.tail = Some(new_tail); // replace head to a new one
                }
                None => {
                    // emptying list because when only one element left, both
                    // head and tail have a reference of it.
                    self.head.take();
                }
            }
            Rc::try_unwrap(old_tail).ok().unwrap().into_inner().elem
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

    pub fn peek_back(&self) -> Option<Ref<T>> {
        self.tail
            .as_ref()
            .map(|refcell_node| Ref::map(refcell_node.borrow(), |node| &node.elem))
    }

    pub fn peek_front_mut(&mut self) -> Option<RefMut<T>> {
        self.head
            .as_mut()
            .map(|rccell_node| RefMut::map(rccell_node.borrow_mut(), |node| &mut node.elem))
    }

    pub fn peek_back_mut(&mut self) -> Option<RefMut<T>> {
        self.tail
            .as_mut()
            .map(|rccell_node| RefMut::map(rccell_node.borrow_mut(), |node| &mut node.elem))
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_front()
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.pop_back()
    }
}

impl<T> List<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
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

        // ---- back -----

        // Check empty list behaves right
        assert_eq!(list.pop_back(), None);

        // Populate list
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        // Check normal removal
        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_back(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push_back(4);
        list.push_back(5);

        // Check normal removal
        assert_eq!(list.pop_back(), Some(5));
        assert_eq!(list.pop_back(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_back(), None);
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

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        list.push_back(4);
        list.push_back(5);

        let mut iter = list.into_iter();

        assert_eq!(Some(1), iter.next());
        assert_eq!(Some(5), iter.next_back());
        assert_eq!(Some(2), iter.next());
        assert_eq!(Some(4), iter.next_back());
        assert_eq!(Some(3), iter.next());
        assert_eq!(None, iter.next_back());
        assert_eq!(None, iter.next());
    }
}
