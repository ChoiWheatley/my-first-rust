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
                // Re-wrap into Box
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
}
