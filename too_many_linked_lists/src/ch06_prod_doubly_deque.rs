#![allow(unused)]
/// What we will learn...
/// - Variance and Subtyping
///     - I guess this is about hierarchy of enums? polymorphic traits?
/// - Panic Safety
/// - PhantomData
///     - 0-sized marker... for what?
/// - Cursors
///     - Huh? Is it similar to Iterators?
///     - Seek back and forth with it.
/// - NonNull
///     - What???? Nullable NonNull???
use std::{marker::PhantomData, ptr::NonNull};

pub struct LinkedList<T> {
    front: Link<T>,
    back: Link<T>,
    len: usize,
    // We semantically store values of T by value.
    _boo: PhantomData<T>,
}

type Link<T> = Option<NonNull<Node<T>>>;

struct Node<T> {
    front: Link<T>,
    back: Link<T>,
    elem: T,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            front: None,
            back: None,
            len: 0,
            _boo: PhantomData,
        }
    }

    /// version of later-wrapped-into-NonNull
    pub fn push_front(&mut self, elem: T) {
        // create new node in a heap realm
        let new_node = Box::into_raw(Box::new(Node {
            front: None,
            back: None,
            elem,
        }));
        if self.front.is_none() {
            // this is an empty list, front and back are both empty
            self.back = NonNull::new(new_node);
        } else {
            // front already points real node, we will actually append new head
            let old_front = self.front;
            unsafe { &mut *new_node }.back = old_front;
            unsafe { &mut *old_front.unwrap().as_mut() }.front = NonNull::new(new_node);
        }
        self.front = NonNull::new(new_node);
        self.len += 1;
    }

    /// version of early-wrapped-NonNull, which is same as the author wrote before.
    // pub fn push_front(&mut self, elem: T) {
    //     unsafe {
    //         let new_node = NonNull::new_unchecked(Box::into_raw(Box::new(Node {
    //             elem,
    //             front: None,
    //             back: None,
    //         })));
    //         if let Some(old_front) = self.front {
    //             // front already exists, append it!
    //             (*new_node.as_ptr()).back = Some(old_front);
    //             (*old_front.as_ptr()).front = Some(new_node);
    //         } else {
    //             // empty list
    //             self.back = Some(new_node);
    //         }
    //         self.front = Some(new_node);
    //         self.len += 1;
    //     }
    // }

    pub fn pop_front(&mut self) -> Option<T> {
        unsafe {
            // no need to `take` because every raw pointer is `Copy`
            self.front.map(|old_node| {
                // ownership has changed
                let old_node = Box::from_raw(old_node.as_ptr());

                self.front = old_node.back;
                if let Some(new_front) = self.front {
                    // cleanup this reference to the removed node
                    (*new_front.as_ptr()).front = None;
                } else {
                    // become an empty list
                    self.back = None;
                }

                self.len -= 1;
                old_node.elem
            })
        }
    }
    pub fn len(&self) -> usize {
        self.len
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop_front() {}
    }
}

#[cfg(test)]
mod test {
    use std::ops::Range;

    use super::*;

    #[test]
    fn push_pop_front() {
        const SEQ: Range<i32> = (0..10);
        let mut ls = LinkedList::new();
        SEQ.for_each(|e| ls.push_front(e));
        assert_eq!(10, ls.len());

        SEQ.rev().for_each(|e| assert_eq!(Some(e), ls.pop_front()));
        assert_eq!(0, ls.len());
        assert_eq!(None, ls.pop_front());
    }

    #[test]
    fn test_basic_front() {
        let mut list = LinkedList::new();

        // Try to break an empty list
        assert_eq!(list.len(), 0);
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.len(), 0);

        // Try to break a one item list
        list.push_front(10);
        assert_eq!(list.len(), 1);
        assert_eq!(list.pop_front(), Some(10));
        assert_eq!(list.len(), 0);
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.len(), 0);

        // Mess around
        list.push_front(10);
        assert_eq!(list.len(), 1);
        list.push_front(20);
        assert_eq!(list.len(), 2);
        list.push_front(30);
        assert_eq!(list.len(), 3);
        assert_eq!(list.pop_front(), Some(30));
        assert_eq!(list.len(), 2);
        list.push_front(40);
        assert_eq!(list.len(), 3);
        assert_eq!(list.pop_front(), Some(40));
        assert_eq!(list.len(), 2);
        assert_eq!(list.pop_front(), Some(20));
        assert_eq!(list.len(), 1);
        assert_eq!(list.pop_front(), Some(10));
        assert_eq!(list.len(), 0);
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.len(), 0);
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.len(), 0);
    }
}
