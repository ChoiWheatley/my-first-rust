// #![allow(unused)]
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

/// Move back and forth freely between elements. Cursor also can walk over between lists!
/// for example, when you splice two lists, you can
/// jump between lists's elements!
///
/// For conformance of single mutably-borrow-rule, `Cursor` takes list as mutable reference.
/// acquiring each cursor's element also need to be borrowed by once!
pub struct CursorMut<'a, T> {
    cur: Link<T>,
    list: &'a mut LinkedList<T>,
    index: Option<usize>,
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
        while self.pop_front().is_some() {}
    }
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}
impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
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

impl<T> LinkedList<T> {
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

impl<T> LinkedList<T> {
    pub fn cursor_mut(&mut self) -> CursorMut<T> {
        CursorMut {
            cur: None,
            list: self,
            index: None, // ghost at first place 👻
        }
    }
}

impl<'a, T> CursorMut<'a, T> {
    pub fn index(&self) -> Option<usize> {
        self.index
    }

    /// Interesting four cases
    /// 1. normal case: move on to the next(back) cursor
    /// 2. normal case, hit ghost: clean up index
    /// 3. ghost case, very beginning
    /// 4. ghost case, empty list
    pub fn move_next(&mut self) {
        if let Some(cur) = self.cur {
            // we are placed in somwhere in the list
            unsafe {
                self.cur = (*cur.as_ptr()).back;
                if self.cur.is_some() {
                    // new node is available
                    self.index = self.index.map(|idx| idx + 1);
                } else {
                    // new node is ghost, no more index
                    self.index = None;
                }
            }
        } else if !self.list.is_empty() {
            // we are at the very beginning
            self.cur = self.list.front;
            self.index = Some(0);
        } else { // that was a empty list... do nothing
        }
    }

    pub fn move_prev(&mut self) {
        if let Some(cur) = self.cur {
            // normal case
            unsafe {
                self.cur = (*cur.as_ptr()).front;
                if self.cur.is_some() {
                    // new node is available
                    self.index = self.index.map(|idx| idx - 1);
                } else {
                    // new node hit the ghost, no more index
                    self.index = None;
                }
            }
        } else if !self.list.is_empty() {
            // ghost case, with very beginning (at the back)
            self.cur = self.list.back;
            self.index = Some(self.list.len() - 1);
        } else {
            // ghost case, empty list
        }
    }

    /// get current element pointed to  
    pub fn current(&mut self) -> Option<&mut T> {
        self.cur.map(|node| unsafe { &mut (*node.as_ptr()).elem })
    }

    pub fn peek_next(&mut self) -> Option<&mut T> {
        let next = match self.cur {
            Some(cur) => {
                // normal case
                unsafe { (*cur.as_ptr()).back }
            }
            None => {
                // 👻 case
                self.list.front
            }
        };

        // yield the element if the next node exists
        unsafe { next.map(|node| &mut (*node.as_ptr()).elem) }
    }

    pub fn peek_prev(&mut self) -> Option<&mut T> {
        let next = match self.cur {
            Some(cur) => unsafe { (*cur.as_ptr()).front },
            None => {
                // 👻 case
                self.list.back
            }
        };

        // yield the element if the next node exists
        unsafe { next.map(|node| &mut (*node.as_ptr()).elem) }
    }

    /// Split list in a range of [list.front, cur) and [cur, list.back]
    /// return the first list, original list will be changed into second one.
    ///
    /// before
    /// ```
    /// l.f -> A <-> B <-> C <-> D <- l.b
    ///                    ^
    ///                   cur
    /// ```
    ///
    /// after
    /// ```
    /// return: {l.f -> A <-> B <- l.b}
    ///
    /// {l.f -> C <-> D <- l.b}
    ///         ^
    ///        cur
    /// ```
    ///
    pub fn split_before(&mut self) -> LinkedList<T> {
        if self.cur.is_none() {
            // two possibilities, which have same consequence:
            // 1. we hit the list's back
            // 2. the list itself is empty
            return std::mem::replace(self.list, LinkedList::new());
        }
        // normal case
        unsafe {
            let cur = self.cur.unwrap();
            let prev = (*cur.as_ptr()).front;

            let new_list = LinkedList {
                front: self.list.front,
                back: prev,
                len: self.index.unwrap(),
                _boo: PhantomData,
            };
            let old_front = self.cur;
            let old_back = self.list.back;
            let old_len = self.list.len() - self.index.unwrap();

            // break the links between cur and prev
            if let Some(prev) = prev {
                (*prev.as_ptr()).back = None;
                (*cur.as_ptr()).front = None;
            }

            // produce the result
            self.list.front = old_front;
            self.list.back = old_back;
            self.list.len = old_len;
            self.index = Some(0);

            new_list
        }
    }
    /// Split list in a range of `[list.front, cur]` and `(cur, list.back]`
    /// return the second list, original list will be changed into first one.
    ///
    /// before
    /// ```
    /// l.f -> A <-> B <-> C <-> D <- l.b
    ///              ^
    ///             cur
    /// ```
    ///
    /// after
    /// ```
    /// {l.f -> A <-> B <- l.b}
    ///               ^
    ///              cur
    ///
    /// return: {l.f -> C <-> D <- l.b}
    /// ```
    ///
    pub fn split_after(&mut self) -> LinkedList<T> {
        if self.cur.is_none() {
            // corner case:
            // 1. we hit the list's front
            // 2. the list itself is empty
            return std::mem::replace(self.list, LinkedList::new());
        }
        // normal case
        unsafe {
            let cur = self.cur.unwrap();
            let post = (*cur.as_ptr()).back;

            let old_front = self.list.front;
            let old_back = self.cur;
            let old_len = self.index.unwrap() + 1;

            let new_list = LinkedList {
                front: post,
                back: self.list.back,
                len: self.list.len - old_len,
                _boo: PhantomData,
            };

            // break the links between cur and next
            if let Some(post) = post {
                (*post.as_ptr()).front = None;
                (*cur.as_ptr()).back = None;
            }

            // produce the result
            self.list.front = old_front;
            self.list.back = old_back;
            self.list.len = old_len;
            // self.index = self.index; // index not changed

            new_list
        }
    }

    /// Consume other list, grafts its contents into ours.
    ///
    /// before
    /// ```
    /// {A-B-C-D}.splice_before({1-2-3})
    ///      ^
    ///     cur
    /// ```
    /// after
    /// ```
    /// {A-B-1-2-3-C-D}
    ///            ^
    ///           cur
    /// ```
    /// [test_cursor_mut_insert]
    pub fn splice_before(&mut self, mut other: LinkedList<T>)
    where
        T: Debug,
    {
        if other.is_empty() {
            return; // do nothing
        }

        dbg!(&other);

        let self_len = self.list.len();
        let other_len = other.len();

        unsafe {
            match self.cur {
                Some(_cur) if self.cur == self.list.front => {
                    // append front!
                    self.list.append_front(other);
                    // Also you have to make sure our index has changed!!
                    self.index = self.index.map(|idx| idx + other_len);
                }

                Some(cur) => {
                    // general case! link four pointers
                    let other_front = other.front.take().unwrap();
                    let other_back = other.back.take().unwrap();

                    let prev = (*cur.as_ptr()).front.unwrap();
                    (*prev.as_ptr()).back = Some(other_front);
                    (*other_front.as_ptr()).front = Some(prev);
                    (*cur.as_ptr()).front = Some(other_back);
                    (*other_back.as_ptr()).back = Some(cur);
                    // Also you have to make sure our index has changed
                    self.index = self.index.map(|idx| idx + other_len);
                }

                None if self.list.is_empty() => {
                    // just replace other list into ours
                    std::mem::swap(self.list, &mut other);
                }

                _ => {
                    // self.list is not empty, assume our cursor hit the list.back
                    // append back!
                    self.list.append_back(other);
                }
            }
        } // end of unsafe

        // make sure our list's length changed(or not)!!
        self.list.len = self_len + other_len;
    }

    /// Consume other list, grafts contents into ours
    /// before
    /// ```
    /// {A-B-C-D}.splice_before({1-2-3})
    ///    ^
    ///   cur
    /// ```
    /// after
    /// ```
    /// {A-B-1-2-3-C-D}
    ///    ^
    ///   cur
    /// ```
    pub fn splice_after(&mut self, mut other: LinkedList<T>)
    where
        T: Debug,
    {
        if other.is_empty() {
            // do nothing
            return;
        }
        dbg!(&other);

        let self_len = self.list.len();
        let other_len = other.len();

        unsafe {
            match self.cur {
                Some(_) if self.list.back == self.cur => {
                    // append back!
                    self.list.append_back(other);
                }

                Some(cur) => {
                    // general case! link four pointers
                    let other_front = other.front.take().unwrap();
                    let other_back = other.back.take().unwrap();

                    let post = (*cur.as_ptr()).back.unwrap();
                    (*cur.as_ptr()).back = Some(other_front);
                    (*other_front.as_ptr()).front = Some(cur);
                    (*post.as_ptr()).front = Some(other_back);
                    (*other_back.as_ptr()).back = Some(post);
                }

                None if self.list.is_empty() => {
                    // just replace other list into ours
                    std::mem::swap(self.list, &mut other);
                }

                _ => {
                    // self.list is not empty
                    // append front!
                    self.list.append_front(other);
                    // 3. change index number
                    self.index = self.index.map(|idx| idx + other_len);
                }
            }
        } // end of unsafe

        // finally we can change our list's length
        self.list.len = self_len + other_len;
    }

    pub fn insert_before(&mut self, elem: T) {
        match self.cur {
            Some(_cur) if self.list.front == self.cur => {
                // cur is front
                self.list.push_front(elem);
                self.index = self.index.map(|idx| idx + 1);
            }
            Some(cur) => {
                // normal case, create new elem and graft between
                // prev and cur
                unsafe {
                    let prev = (*cur.as_ptr()).front.unwrap();
                    let new_node = NonNull::new_unchecked(Box::into_raw(Box::new(Node {
                        front: Some(prev),
                        back: Some(cur),
                        elem,
                    })));
                    (*prev.as_ptr()).back = Some(new_node);
                    (*cur.as_ptr()).front = Some(new_node);
                }
                self.index = self.index.map(|idx| idx + 1);
            }
            None => {
                // assume cur hit list.back
                self.list.push_back(elem);
            }
        }
        self.list.len += 1;
    }

    pub fn insert_after(&mut self, elem: T) {
        match self.cur {
            Some(_cur) if self.list.back == self.cur => {
                // cur is back
                self.list.push_back(elem);
            }
            Some(cur) => {
                // normal case, create new elem and graft between
                // cur and post
                unsafe {
                    let post = (*cur.as_ptr()).back.unwrap();
                    let new_node = NonNull::new_unchecked(Box::into_raw(Box::new(Node {
                        front: Some(cur),
                        back: Some(post),
                        elem,
                    })));
                    (*post.as_ptr()).front = Some(new_node);
                    (*cur.as_ptr()).back = Some(new_node);
                }
            }
            None => {
                // assume cur hit list.front
                self.list.push_front(elem);
                self.index = self.index.map(|idx| idx + 1);
            }
        }
        self.list.len += 1;
    }

    /// remove current element(if exists) and return it.
    /// cur will point right next(after) one
    pub fn remove(&mut self) -> Option<T> {
        self.cur.map(|cur| unsafe {
            // convert from raw pointer to owned Box object
            let cur = Box::from_raw(cur.as_ptr());
            // link two nodes between cur
            match (cur.front, cur.back) {
                (None, None) => {
                    // this list will be emptied
                    self.index = None;
                }
                (None, Some(back)) => {
                    // cur is front
                    self.list.front = Some(back);
                }
                (Some(front), None) => {
                    // cur is back
                    self.list.back = Some(front);
                }
                (Some(front), Some(back)) => {
                    // cur is mid
                    (*front.as_ptr()).back = Some(back);
                    (*back.as_ptr()).front = Some(front);
                }
            }
            // move cursor onto the next(after) node
            self.cur = cur.back;
            cur.elem
        })
    }
}

impl<T> LinkedList<T> {
    /// before
    /// ```
    /// {A-B-C}.append_back({1-2-3})
    /// ```
    /// after
    /// ```
    /// {A-B-C-1-2-3}
    /// ```
    pub fn append_back(&mut self, mut other: LinkedList<T>)
    where
        T: Debug,
    {
        if self.is_empty() {
            std::mem::swap(self, &mut other);
            return;
        }
        if other.is_empty() {
            // do nothing
            return;
        }
        dbg!(&other);
        // we must **take** `other`'s front and back so that dropping `other` will not
        // affect null pointer reference
        let self_back = self.back.take().unwrap();
        let other_front = other.front.take().unwrap();
        let other_back = other.back.take().unwrap();

        unsafe {
            (*self_back.as_ptr()).back = Some(other_front);
            (*other_front.as_ptr()).front = Some(self_back);
        }
        self.back = Some(other_back);
        self.len += other.len();
    }

    /// before
    /// ```
    /// {A-B-C}.append_front({1-2-3})
    /// ```
    /// after
    /// ```
    /// {1-2-3-A-B-C}
    /// ```
    pub fn append_front(&mut self, mut other: LinkedList<T>) {
        if self.is_empty() {
            std::mem::swap(self, &mut other);
            return;
        }
        if other.is_empty() {
            return;
        }
        // make sure `other` will be empty
        let other_back = other.back.take().unwrap();
        let other_front = other.front.take().unwrap();
        let self_front = self.front.unwrap();

        unsafe {
            (*self_front.as_ptr()).front = Some(other_back);
            (*other_back.as_ptr()).back = Some(self_front);
        }
        self.front = Some(other_front);
        self.len += other.len();
    }
}

#[cfg(test)]
mod test {
    use std::ops::Range;

    use super::*;

    #[test]
    fn push_pop_front() {
        const SEQ: Range<i32> = 0..10;
        let mut ls = LinkedList::new();
        SEQ.for_each(|e| ls.push_front(e));
        assert_eq!(10, ls.len());

        SEQ.rev().for_each(|e| assert_eq!(Some(e), ls.pop_front()));
        assert_eq!(0, ls.len());
        assert_eq!(None, ls.pop_front());
    }

    #[test]
    fn push_pop_back() {
        const SEQ: Range<i32> = 0..10;
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

        let iter = ls.iter();
        let answer = "hello, work!";
        assert!(iter.zip(answer.chars()).all(|(&lhs, rhs)| lhs == rhs));
    }

    #[test]
    fn into_iter_of_reference() {
        let seq: Vec<_> = "hello, world".chars().collect();
        let mut ls = LinkedList::new();
        seq.iter().for_each(|&e| ls.push_back(e));

        let mut cur = (&ls).into_iter();
        while cur.next().is_some() {} // drain?

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
    #[allow(clippy::zero_divided_by_zero, clippy::neg_cmp_op_on_partial_ord)]
    fn test_ord_nan() {
        let nan = 0.0f64 / 0.0;
        let n = list_from(&[nan]);
        let m = list_from(&[nan]);
        assert!((n.partial_cmp(&m)).is_none()); // clippy says that `partial_cmp` can detect uncomparable pair
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
    #[test]
    fn test_cursor_move_peek() {
        let mut m: LinkedList<u32> = LinkedList::new();
        m.extend([1, 2, 3, 4, 5, 6]);
        let mut cursor = m.cursor_mut();
        cursor.move_next();
        assert_eq!(cursor.current(), Some(&mut 1));
        assert_eq!(cursor.peek_next(), Some(&mut 2));
        assert_eq!(cursor.peek_prev(), None);
        assert_eq!(cursor.index(), Some(0));
        cursor.move_prev(); // 👻
        assert_eq!(cursor.current(), None);
        assert_eq!(cursor.peek_next(), Some(&mut 1));
        assert_eq!(cursor.peek_prev(), Some(&mut 6));
        assert_eq!(cursor.index(), None);
        cursor.move_next();
        cursor.move_next();
        assert_eq!(cursor.current(), Some(&mut 2));
        assert_eq!(cursor.peek_next(), Some(&mut 3));
        assert_eq!(cursor.peek_prev(), Some(&mut 1));
        assert_eq!(cursor.index(), Some(1));

        let mut cursor = m.cursor_mut();
        cursor.move_prev();
        assert_eq!(cursor.current(), Some(&mut 6));
        assert_eq!(cursor.peek_next(), None);
        assert_eq!(cursor.peek_prev(), Some(&mut 5));
        assert_eq!(cursor.index(), Some(5));
        cursor.move_next();
        assert_eq!(cursor.current(), None);
        assert_eq!(cursor.peek_next(), Some(&mut 1));
        assert_eq!(cursor.peek_prev(), Some(&mut 6));
        assert_eq!(cursor.index(), None);
        cursor.move_prev();
        cursor.move_prev();
        assert_eq!(cursor.current(), Some(&mut 5));
        assert_eq!(cursor.peek_next(), Some(&mut 6));
        assert_eq!(cursor.peek_prev(), Some(&mut 4));
        assert_eq!(cursor.index(), Some(4));
    }

    #[test]
    fn test_cursor_mut_insert() {
        let mut m: LinkedList<u32> = LinkedList::new();
        m.extend([1, 2, 3, 4, 5, 6]);
        let mut cursor = m.cursor_mut();
        cursor.move_next();
        cursor.splice_before(Some(7).into_iter().collect());
        cursor.splice_after(Some(8).into_iter().collect());
        // check_links(&m);
        assert_eq!(
            m.iter().cloned().collect::<Vec<_>>(),
            &[7, 1, 8, 2, 3, 4, 5, 6]
        );
        let mut cursor = m.cursor_mut();
        cursor.move_next();
        cursor.move_prev();
        cursor.splice_before(Some(9).into_iter().collect());
        cursor.splice_after(Some(10).into_iter().collect());
        check_links(&m);
        assert_eq!(
            m.iter().cloned().collect::<Vec<_>>(),
            &[10, 7, 1, 8, 2, 3, 4, 5, 6, 9]
        );

        /* remove_current not impl'd
        let mut cursor = m.cursor_mut();
        cursor.move_next();
        cursor.move_prev();
        assert_eq!(cursor.remove_current(), None);
        cursor.move_next();
        cursor.move_next();
        assert_eq!(cursor.remove_current(), Some(7));
        cursor.move_prev();
        cursor.move_prev();
        cursor.move_prev();
        assert_eq!(cursor.remove_current(), Some(9));
        cursor.move_next();
        assert_eq!(cursor.remove_current(), Some(10));
        check_links(&m);
        assert_eq!(m.iter().cloned().collect::<Vec<_>>(), &[1, 8, 2, 3, 4, 5, 6]);
        */

        let mut m: LinkedList<u32> = LinkedList::new();
        m.extend([1, 8, 2, 3, 4, 5, 6]);
        let mut cursor = m.cursor_mut();
        cursor.move_next();
        let mut p: LinkedList<u32> = LinkedList::new();
        p.extend([100, 101, 102, 103]);
        let mut q: LinkedList<u32> = LinkedList::new();
        q.extend([200, 201, 202, 203]);
        cursor.splice_after(p);
        cursor.splice_before(q);
        check_links(&m);
        assert_eq!(
            m.iter().cloned().collect::<Vec<_>>(),
            &[200, 201, 202, 203, 1, 100, 101, 102, 103, 8, 2, 3, 4, 5, 6]
        );
        let mut cursor = m.cursor_mut();
        cursor.move_next();
        cursor.move_prev();
        let tmp = cursor.split_before();
        assert_eq!(m.into_iter().collect::<Vec<_>>(), &[]);
        m = tmp;
        let mut cursor = m.cursor_mut();
        cursor.move_next();
        cursor.move_next();
        cursor.move_next();
        cursor.move_next();
        cursor.move_next();
        cursor.move_next();
        cursor.move_next();
        let tmp = cursor.split_after();
        assert_eq!(
            tmp.into_iter().collect::<Vec<_>>(),
            &[102, 103, 8, 2, 3, 4, 5, 6]
        );
        check_links(&m);
        assert_eq!(
            m.iter().cloned().collect::<Vec<_>>(),
            &[200, 201, 202, 203, 1, 100, 101]
        );
    }

    fn check_links<T>(_list: &LinkedList<T>) {
        // would be good to do this!
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
#[allow(unused)]
fn iter_mut_invariant() {}
