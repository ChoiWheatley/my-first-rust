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

/// non-consuming iterator
pub struct Iter<'a, T> {
    front: Link<T>,
    back: Link<T>,
    len: usize, // remain element between front and back
    _boo: PhantomData<&'a T>,
}

pub struct IterMut<'a, T> {
    front: Link<T>,
    back: Link<T>,
    len: usize,
    _boo: PhantomData<&'a T>,
}

/// consuming iterator
pub struct IntoIter<T>(LinkedList<T>);

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

    pub fn push_back(&mut self, elem: T) {
        let new_node = unsafe {
            NonNull::new_unchecked(Box::into_raw(Box::new(Node {
                front: None,
                back: None,
                elem,
            })))
        };
        if let Some(old_back) = self.back {
            unsafe {
                (*old_back.as_ptr()).back = Some(new_node);
                (*new_node.as_ptr()).front = Some(old_back);
            }
        } else {
            // empty list
            self.front = Some(new_node);
        }
        self.back = Some(new_node);
        self.len += 1;
    }

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

    pub fn pop_back(&mut self) -> Option<T> {
        self.back.map(|old_node| unsafe {
            // ownership has changed
            let old_node = Box::from_raw(old_node.as_ptr());

            self.back = old_node.front;
            if let Some(new_back) = self.back {
                // clean-up new_back
                (*new_back.as_ptr()).back = None;
            } else {
                // become an empty list
                self.front = None;
            }

            self.len -= 1;
            old_node.elem
        })
    }
}

/// getter and setters
impl<T> LinkedList<T> {
    pub fn len(&self) -> usize {
        self.len
    }
    pub fn front(&self) -> Option<&T> {
        unsafe { self.front.map(|node| &(*node.as_ptr()).elem) }
    }
    pub fn back(&self) -> Option<&T> {
        unsafe { self.back.map(|node| &(*node.as_ptr()).elem) }
    }
    pub fn front_mut(&mut self) -> Option<&mut T> {
        unsafe { self.front.map(|node| &mut (*node.as_ptr()).elem) }
    }
    pub fn back_mut(&mut self) -> Option<&mut T> {
        unsafe { self.back.map(|node| &mut (*node.as_ptr()).elem) }
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop_front() {}
    }
}

impl<'a, T> LinkedList<T> {
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            front: self.front,
            back: self.back,
            len: self.len,
            _boo: PhantomData,
        }
    }
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            front: self.front,
            back: self.back,
            len: self.len,
            _boo: PhantomData,
        }
    }
}

/// Why `IntoIterator` didn't consume `LinkedList`???
impl<'a, T> IntoIterator for &'a LinkedList<T> {
    type Item = &'a T;

    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut LinkedList<T> {
    type Item = &'a mut T;

    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        IterMut {
            front: self.front,
            back: self.back,
            len: self.len,
            _boo: PhantomData,
        }
    }
}

/// this `IntoIterator` consumes `LinkedList`
impl<T> IntoIterator for LinkedList<T> {
    type Item = T;

    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self)
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    /// go afterward and change len state
    /// This method doesn't lose link even [front, back] range gets shrink
    fn next(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            return None;
        }
        self.front.map(|node| unsafe {
            self.front = (*node.as_ptr()).back;
            self.len -= 1;
            &(*node.as_ptr()).elem
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_front()
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.0.len, Some(self.0.len))
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            return None;
        }
        self.front.map(|node| unsafe {
            self.front = (*node.as_ptr()).back;
            self.len -= 1;
            &mut (*node.as_ptr()).elem
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
    /// Go backward and change len state
    /// This method doesn't lose link even [front, back] range gets shrink
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            return None;
        }
        self.back.map(|node| unsafe {
            self.back = (*node.as_ptr()).front;
            self.len -= 1;
            &(*node.as_ptr()).elem
        })
    }
}

impl<'a, T> DoubleEndedIterator for IterMut<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            return None;
        }
        self.back.map(|node| unsafe {
            self.back = (*node.as_ptr()).front;
            self.len -= 1;
            &mut (*node.as_ptr()).elem
        })
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.pop_back()
    }
}

impl<'a, T> ExactSizeIterator for Iter<'a, T> {
    fn len(&self) -> usize {
        self.len
    }
}
impl<T> ExactSizeIterator for IntoIter<T> {
    fn len(&self) -> usize {
        self.0.len()
    }
}
impl<'a, T> ExactSizeIterator for IterMut<'a, T> {
    fn len(&self) -> usize {
        self.len
    }
}

#[cfg(test)]
mod test {
    use std::{ops::Range, str::Chars};

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
    fn push_pop_back() {
        const SEQ: Range<i32> = (0..10);
        let mut ls = LinkedList::new();
        SEQ.for_each(|e| ls.push_back(e));
        assert_eq!(10, ls.len());

        SEQ.rev().for_each(|e| assert_eq!(Some(e), ls.pop_back()));
        assert_eq!(0, ls.len());
        assert_eq!(None, ls.pop_back());
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

    #[test]
    fn iter() {
        let seq: Vec<_> = "hello, world!".chars().collect();
        let mut ls = LinkedList::new();
        seq.iter().cloned().for_each(|c| ls.push_back(c));
        assert_eq!(seq.len(), ls.len());

        // same contents test
        assert!(seq
            .iter()
            .cloned()
            .zip(ls.iter().cloned())
            .all(|(lhs, rhs)| lhs == rhs));

        // each element test
        let mut ls_iter = ls.iter();
        let mut seq_iter = seq.iter();
        assert_eq!(seq.len(), ls_iter.len());
        for _ in 0..ls.len() {
            assert_eq!(ls_iter.next(), seq_iter.next());
        }
        assert_eq!(None, ls_iter.next());
        assert_eq!(0, ls_iter.len());

        // iterator cannot regrow backward! I think `Cursor` can go back and forth freely.
        assert_eq!(None, ls_iter.next_back());

        // backward
        let mut ls_iter = ls.iter();
        let mut seq_iter = seq.iter();
        for _ in 0..ls.len() {
            assert_eq!(ls_iter.next_back(), seq_iter.next_back());
        }
        assert_eq!(None, ls_iter.next());
        assert_eq!(None, ls_iter.next_back());
        assert_eq!(0, ls_iter.len());
    }

    #[test]
    fn into_iter() {
        let seq: Vec<_> = "hello, world".chars().collect();
        let mut ls = LinkedList::new();
        seq.iter().for_each(|&e| ls.push_back(e));

        let mut ls_iter = ls.into_iter();
        let mut seq_iter = seq.iter().cloned();

        assert_eq!(seq.len(), ls_iter.len());

        assert_eq!(seq_iter.next(), ls_iter.next());
        assert_eq!(seq_iter.next_back(), ls_iter.next_back());
        assert_eq!(seq_iter.next(), ls_iter.next());
        assert_eq!(seq_iter.next_back(), ls_iter.next_back());
        assert_eq!(seq_iter.next(), ls_iter.next());
        assert_eq!(seq_iter.next_back(), ls_iter.next_back());
        assert_eq!(seq_iter.next(), ls_iter.next());
        assert_eq!(seq_iter.next(), ls_iter.next());
        assert_eq!(seq_iter.next_back(), ls_iter.next_back());
        assert_eq!(seq_iter.next(), ls_iter.next());
        assert_eq!(seq_iter.next_back(), ls_iter.next_back());
        assert_eq!(seq_iter.next(), ls_iter.next());
        assert_eq!(seq_iter.next_back(), ls_iter.next_back());
        assert_eq!(seq_iter.next(), ls_iter.next());

        assert_eq!(None, ls_iter.next());
        assert_eq!(None, ls_iter.next_back());
    }

    #[test]
    fn iter_mut() {
        let seq: Vec<_> = "hello, world".chars().collect();
        let mut ls = LinkedList::new();
        seq.iter().for_each(|&e| ls.push_back(e));

        let mut cur = ls.iter_mut();
        *cur.next_back().unwrap() = '!';
        *cur.next_back().unwrap() = 'k';

        let mut iter = ls.iter();
        let answer = "hello, work!";
        assert!(iter.zip(answer.chars()).all(|(&lhs, rhs)| lhs == rhs));
    }

    #[test]
    fn into_iter_of_reference() {
        let seq: Vec<_> = "hello, world".chars().collect();
        let mut ls = LinkedList::new();
        seq.iter().for_each(|&e| ls.push_back(e));

        let mut cur = (&ls).into_iter();
        while let Some(_) = cur.next() {} // drain?

        // unchanged?
        assert_eq!(ls.len(), seq.len());
    }
}
