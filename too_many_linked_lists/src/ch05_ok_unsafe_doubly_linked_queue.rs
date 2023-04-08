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

type Link<T> = *mut Node<T>;
struct Node<T> {
    elem: T,
    next: Link<T>,
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}
pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}
pub struct IntoIter<T>(List<T>);

impl<T> List<T> {
    pub fn new() -> Self {
        List {
            head: null_mut(),
            tail: null_mut(),
        }
    }

    /// This is queue, so push operation can be done in its tail
    pub fn push(&mut self, elem: T) {
        unsafe {
            /// create a new node in a heap area, turn it exactly into raw pointer
            let new_tail = Box::into_raw(Box::new(Node {
                elem,
                next: null_mut(),
            }));
            if self.head.is_null() {
                self.head = new_tail;
            } else {
                (*self.tail).next = new_tail;
            }
            self.tail = new_tail;
        }
    }

    /// This is queue, so pop operation can be done in its head
    pub fn pop(&mut self) -> Option<T> {
        if self.head.is_null() {
            None
        } else {
            unsafe {
                // Re-wrap into Box, this object will be dropped when this block ends
                let ret = Box::from_raw(self.head);

                self.head = ret.next;

                if self.head.is_null() {
                    // empty tail again!
                    self.tail = null_mut();
                }

                Some(ret.elem)
            }
        }
    }

    pub fn peek(&self) -> Option<&T> {
        unsafe { self.head.as_ref().map(|node| &node.elem) }
    }
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        unsafe { self.head.as_mut().map(|node| &mut node.elem) }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while let Some(cur) = self.pop() {}
    }
}

impl<'a, T> List<T> {
    /// consumes list and iterate elements
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: unsafe { self.head.as_ref() },
        }
    }
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
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
        unsafe {
            self.next.take().map(|node| {
                self.next = node.next.as_ref();
                &node.elem
            })
        }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            self.next.take().map(|node| {
                self.next = node.next.as_mut();
                &mut node.elem
            })
        }
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
