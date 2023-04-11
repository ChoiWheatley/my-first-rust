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
use std::{fmt::Debug, hash::Hash, io::Cursor, marker::PhantomData, ptr::NonNull};

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
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
    pub fn clear(&mut self) {
        while let Some(_) = self.pop_front() {}
    }
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}
impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop_front() {}
    }
}
impl<T> Clone for LinkedList<T>
where
    T: Clone,
{
    /// It is different from Default derivable `Clone` behavior
    fn clone(&self) -> Self {
        let mut new_list = Self::new();
        for item in self {
            new_list.push_back(item.clone());
        }
        new_list
    }
}

impl<T> Extend<T> for LinkedList<T> {
    /// `extend`given collection which can be converted to iterator
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for item in iter {
            self.push_back(item);
        }
    }
}

impl<T> FromIterator<T> for LinkedList<T> {
    /// create a new `LinkedList` with given collection which can be converted into iterator
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut ret = Self::new();
        for item in iter {
            ret.push_back(item);
        }
        ret
    }
}

impl<T: Debug> Debug for LinkedList<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self).finish()
    }
}

impl<T: PartialEq> PartialEq for LinkedList<T> {
    fn eq(&self, other: &Self) -> bool {
        self.len() == other.len() && self.iter().eq(other)
    }

    fn ne(&self, other: &Self) -> bool {
        self.len() != other.len() || self.iter().ne(other)
    }
}

impl<T: Eq> Eq for LinkedList<T> {}

impl<T: PartialOrd> PartialOrd for LinkedList<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.iter().partial_cmp(other)
    }
}

impl<T: Ord> Ord for LinkedList<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.iter().cmp(other)
    }
}

impl<T: Hash> Hash for LinkedList<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.len().hash(state);
        for item in self {
            item.hash(state);
        }
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

    fn list_from<T: Clone>(v: &[T]) -> LinkedList<T> {
        v.iter().map(|x| (*x).clone()).collect()
    }

    fn generate_test() -> LinkedList<i32> {
        list_from(&[0, 1, 2, 3, 4, 5, 6])
    }
    #[test]
    fn test_basic() {
        let mut m = LinkedList::new();
        assert_eq!(m.pop_front(), None);
        assert_eq!(m.pop_back(), None);
        assert_eq!(m.pop_front(), None);
        m.push_front(1);
        assert_eq!(m.pop_front(), Some(1));
        m.push_back(2);
        m.push_back(3);
        assert_eq!(m.len(), 2);
        assert_eq!(m.pop_front(), Some(2));
        assert_eq!(m.pop_front(), Some(3));
        assert_eq!(m.len(), 0);
        assert_eq!(m.pop_front(), None);
        m.push_back(1);
        m.push_back(3);
        m.push_back(5);
        m.push_back(7);
        assert_eq!(m.pop_front(), Some(1));

        let mut n = LinkedList::new();
        n.push_front(2);
        n.push_front(3);
        {
            assert_eq!(n.front().unwrap(), &3);
            let x = n.front_mut().unwrap();
            assert_eq!(*x, 3);
            *x = 0;
        }
        {
            assert_eq!(n.back().unwrap(), &2);
            let y = n.back_mut().unwrap();
            assert_eq!(*y, 2);
            *y = 1;
        }
        assert_eq!(n.pop_front(), Some(0));
        assert_eq!(n.pop_front(), Some(1));
    }
    #[test]
    fn test_iterator() {
        let m = generate_test();
        for (i, elt) in m.iter().enumerate() {
            assert_eq!(i as i32, *elt);
        }
        let mut n = LinkedList::new();
        assert_eq!(n.iter().next(), None);
        n.push_front(4);
        let mut it = n.iter();
        assert_eq!(it.size_hint(), (1, Some(1)));
        assert_eq!(it.next().unwrap(), &4);
        assert_eq!(it.size_hint(), (0, Some(0)));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn test_iterator_double_end() {
        let mut n = LinkedList::new();
        assert_eq!(n.iter().next(), None);
        n.push_front(4);
        n.push_front(5);
        n.push_front(6);
        let mut it = n.iter();
        assert_eq!(it.size_hint(), (3, Some(3)));
        assert_eq!(it.next().unwrap(), &6);
        assert_eq!(it.size_hint(), (2, Some(2)));
        assert_eq!(it.next_back().unwrap(), &4);
        assert_eq!(it.size_hint(), (1, Some(1)));
        assert_eq!(it.next_back().unwrap(), &5);
        assert_eq!(it.next_back(), None);
        assert_eq!(it.next(), None);
    }

    #[test]
    fn test_rev_iter() {
        let m = generate_test();
        for (i, elt) in m.iter().rev().enumerate() {
            assert_eq!(6 - i as i32, *elt);
        }
        let mut n = LinkedList::new();
        assert_eq!(n.iter().rev().next(), None);
        n.push_front(4);
        let mut it = n.iter().rev();
        assert_eq!(it.size_hint(), (1, Some(1)));
        assert_eq!(it.next().unwrap(), &4);
        assert_eq!(it.size_hint(), (0, Some(0)));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn test_mut_iter() {
        let mut m = generate_test();
        let mut len = m.len();
        for (i, elt) in m.iter_mut().enumerate() {
            assert_eq!(i as i32, *elt);
            len -= 1;
        }
        assert_eq!(len, 0);
        let mut n = LinkedList::new();
        assert!(n.iter_mut().next().is_none());
        n.push_front(4);
        n.push_back(5);
        let mut it = n.iter_mut();
        assert_eq!(it.size_hint(), (2, Some(2)));
        assert!(it.next().is_some());
        assert!(it.next().is_some());
        assert_eq!(it.size_hint(), (0, Some(0)));
        assert!(it.next().is_none());
    }

    #[test]
    fn test_iterator_mut_double_end() {
        let mut n = LinkedList::new();
        assert!(n.iter_mut().next_back().is_none());
        n.push_front(4);
        n.push_front(5);
        n.push_front(6);
        let mut it = n.iter_mut();
        assert_eq!(it.size_hint(), (3, Some(3)));
        assert_eq!(*it.next().unwrap(), 6);
        assert_eq!(it.size_hint(), (2, Some(2)));
        assert_eq!(*it.next_back().unwrap(), 4);
        assert_eq!(it.size_hint(), (1, Some(1)));
        assert_eq!(*it.next_back().unwrap(), 5);
        assert!(it.next_back().is_none());
        assert!(it.next().is_none());
    }

    #[test]
    fn test_eq() {
        let mut n: LinkedList<u8> = list_from(&[]);
        let mut m = list_from(&[]);
        assert!(n == m);
        n.push_front(1);
        assert!(n != m);
        m.push_back(1);
        assert!(n == m);

        let n = list_from(&[2, 3, 4]);
        let m = list_from(&[1, 2, 3]);
        assert!(n != m);
    }

    #[test]
    fn test_ord() {
        let n = list_from(&[]);
        let m = list_from(&[1, 2, 3]);
        assert!(n < m);
        assert!(m > n);
        assert!(n <= n);
        assert!(n >= n);
    }

    #[test]
    fn test_ord_nan() {
        let nan = 0.0f64 / 0.0;
        let n = list_from(&[nan]);
        let m = list_from(&[nan]);
        assert!(!(n < m));
        assert!(!(n > m));
        assert!(!(n <= m));
        assert!(!(n >= m));

        let n = list_from(&[nan]);
        let one = list_from(&[1.0f64]);
        assert!(!(n < one));
        assert!(!(n > one));
        assert!(!(n <= one));
        assert!(!(n >= one));

        let u = list_from(&[1.0f64, 2.0, nan]);
        let v = list_from(&[1.0f64, 2.0, 3.0]);
        assert!(!(u < v));
        assert!(!(u > v));
        assert!(!(u <= v));
        assert!(!(u >= v));

        let s = list_from(&[1.0f64, 2.0, 4.0, 2.0]);
        let t = list_from(&[1.0f64, 2.0, 3.0, 2.0]);
        assert!(!(s < t));
        assert!(s > one);
        assert!(!(s <= one));
        assert!(s >= one);
    }

    #[test]
    fn test_debug() {
        let list: LinkedList<i32> = (0..10).collect();
        assert_eq!(format!("{:?}", list), "[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]");

        let list: LinkedList<&str> = vec!["just", "one", "test", "more"]
            .iter()
            .copied()
            .collect();
        assert_eq!(format!("{:?}", list), r#"["just", "one", "test", "more"]"#);
    }

    #[test]
    fn test_hashmap() {
        // Check that HashMap works with this as a key

        let list1: LinkedList<i32> = (0..10).collect();
        let list2: LinkedList<i32> = (1..11).collect();
        let mut map = std::collections::HashMap::new();

        assert_eq!(map.insert(list1.clone(), "list1"), None);
        assert_eq!(map.insert(list2.clone(), "list2"), None);

        assert_eq!(map.len(), 2);

        assert_eq!(map.get(&list1), Some(&"list1"));
        assert_eq!(map.get(&list2), Some(&"list2"));

        assert_eq!(map.remove(&list1), Some("list1"));
        assert_eq!(map.remove(&list2), Some("list2"));

        assert!(map.is_empty());
    }
}

/// Compile-time assertions checking what is Send and Sync
#[allow(dead_code)]
fn assert_properties() {
    fn is_send<T: Send>() {}
    fn is_sync<T: Sync>() {}

    is_send::<LinkedList<i32>>();
    is_sync::<LinkedList<i32>>();

    is_send::<IntoIter<i32>>();
    is_send::<IntoIter<i32>>();

    is_send::<Iter<i32>>();
    is_sync::<Iter<i32>>();

    // is_send::<IterMut<i32>>();
    // is_sync::<IterMut<i32>>();

    is_send::<Cursor<i32>>();
    is_sync::<Cursor<i32>>();

    fn linked_list_covariant<'a, T>(x: LinkedList<&'static T>) -> LinkedList<&'a T> {
        x
    }
    fn iter_covariant<'i, 'a, T>(x: Iter<'i, &'static T>) -> Iter<'i, &'a T> {
        x
    }
    fn into_iter_covariant<'a, T>(x: IntoIter<&'static T>) -> IntoIter<&'a T> {
        x
    }
}

/// Let's opt-back-in Send and Sync to our `LinkedList` which was
/// firstly opt-out because of using raw pointers
unsafe impl<T: Send> Send for LinkedList<T> {}
unsafe impl<T: Sync> Sync for LinkedList<T> {}
unsafe impl<'a, T: Send> Send for Iter<'a, T> {}
unsafe impl<'a, T: Sync> Sync for Iter<'a, T> {}
unsafe impl<T: Send> Send for IntoIter<T> {}
unsafe impl<T: Sync> Sync for IntoIter<T> {}
// IterMut DEFINITELY shouldn't be covariant, because it is like `&mut T`
// unsafe impl<'a, T: Send> Send for IterMut<'a, T> {}
// unsafe impl<'a, T: Sync> Sync for IterMut<'a, T> {}

/// This Doccomment can be compiled independently
/// ```
/// fn iter_mut_covariant<'i, 'a, T>(x: IterMut<'i, &'static T>) -> IterMut<'i, &'a T> { x }
/// ```
fn iter_mut_invariant() {}
