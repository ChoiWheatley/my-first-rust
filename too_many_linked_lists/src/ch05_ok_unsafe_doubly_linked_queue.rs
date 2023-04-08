#![allow(unused)]
use std::ptr::null_mut;

/// # hello unsafe
///
/// What I can learn with this chapter?
///
/// - The mistakes in a previous chapter
/// - Guarantee that pointing same node at same time would be safe
/// - Also I want a Node to own its element

pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
}

pub struct IntoIter<T>(List<T>);

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

type Link<T> = *mut Node<T>;
struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {
            head: null_mut(),
            tail: null_mut(),
        }
    }

    /// push new node into the tail
    pub fn push(&mut self, elem: T) {
        /// allocate new node to the heap
        let new_node = Box::into_raw(Box::new(Node {
            elem,
            next: null_mut(),
        }));
        if self.head.is_null() {
            self.head = new_node;
        } else {
            unsafe {
                (*self.tail).next = new_node;
            }
        }
        self.tail = new_node;
    }

    /// pop from head and make other things tidy
    pub fn pop(&mut self) -> Option<T> {
        if self.head.is_null() {
            None
        } else {
            let old_head = unsafe { Box::from_raw(self.head) };
            self.head = old_head.next;

            // tidy
            if self.head.is_null() {
                // empty tail again!
                self.tail = null_mut();
            }

            Some(old_head.elem)
        }
    }

    pub fn peek(&self) -> Option<&T> {
        unsafe { self.head.as_ref().map(|node| &node.elem) }
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        unsafe { self.head.as_mut().map(|node| &mut node.elem) }
    }
}

impl<'a, T> List<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
    pub fn iter(&self) -> Iter<'a, T> {
        Iter {
            next: unsafe { self.head.as_ref() },
        }
    }
    pub fn iter_mut(&mut self) -> IterMut<'a, T> {
        IterMut {
            next: unsafe { self.head.as_mut() },
        }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = unsafe { (node.next.as_ref()) };
            &node.elem
        })
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = unsafe { node.next.as_mut() };
            &mut node.elem
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while let Some(cur) = self.pop() {}
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basics() {
        let ls = [0, 1, 2, 3];
        let mut list = List::new();

        ls.iter().for_each(|e| list.push(e));

        ls.iter().for_each(|e| assert_eq!(Some(e), list.pop()));
        assert_eq!(None, list.pop());
    }

    #[test]
    fn peek() {
        let ls = [0, 1, 2, 3, 4, 5];
        let mut list = List::new();

        ls.iter().cloned().for_each(|e| list.push(e));

        assert_eq!(Some(&0), list.peek());
        assert_eq!(
            Some(&mut 100),
            list.peek_mut().map(|elem| {
                *elem += 100;
                elem
            })
        );
    }

    #[test]
    fn into_iter() {
        let ls = [0, 1, 2, 3, 4, 5];
        let mut list = List::new();

        ls.iter().cloned().for_each(|e| list.push(e));

        let mut consumed_iterator = list.into_iter();
        let mut answer_iterator = ls.iter();

        while let Some(answer) = answer_iterator.next() {
            assert_eq!(*answer, consumed_iterator.next().unwrap());
        }

        assert_eq!(None, consumed_iterator.next());
    }

    #[test]
    fn iter() {
        let ls = [0, 1, 2, 3, 4, 5];
        let mut list = List::new();

        ls.iter().cloned().for_each(|e| list.push(e));

        let mut iter = list.iter();
        assert_eq!(Some(&0), iter.next());
        assert_eq!(Some(&1), iter.next());
        assert_eq!(Some(&2), iter.next());
        assert_eq!(Some(&3), iter.next());
        assert_eq!(Some(&4), iter.next());
        assert_eq!(Some(&5), iter.next());
        assert_eq!(None, iter.next());

        /// check all of elements are still alive
        let mut iter = list.iter();
        assert_eq!(Some(&0), iter.next());
        assert_eq!(Some(&1), iter.next());
        assert_eq!(Some(&2), iter.next());
        assert_eq!(Some(&3), iter.next());
        assert_eq!(Some(&4), iter.next());
        assert_eq!(Some(&5), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn iter_mut() {
        let ls = [0, 1, 2, 3, 4, 5];
        let mut list = List::new();

        ls.iter().cloned().for_each(|e| list.push(e));

        list.iter_mut().for_each(|elem| *elem *= 2);

        assert!(ls
            .iter()
            .map(|e| e * 2)
            .zip(list.iter().cloned())
            .all(|(l, r)| l == r));
    }
}
