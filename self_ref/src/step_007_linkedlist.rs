/// Let's make self-referential linked list by my own knowledge!! 💪
use std::{
    borrow::BorrowMut,
    cell::RefCell,
    default,
    marker::PhantomPinned,
    ops::{Deref, DerefMut},
    pin::Pin,
    rc::Rc,
};

type RcCell<T> = Rc<RefCell<T>>;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Node<T: Sized> {
    next: Option<RcCell<NodePtr<T>>>, // can be null...
    prev: Option<RcCell<NodePtr<T>>>, // can be null...
    val: T,
    _marker: PhantomPinned,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct NodePtr<T>(*mut Node<T>);

/// holds two dummy nodes which holds each `head` and `tail` position of entire list
#[derive(Debug)]
struct MyLinkedList<T: Sized> {
    head: RcCell<NodePtr<T>>,
    tail: RcCell<NodePtr<T>>,
}

impl<T> From<*mut Node<T>> for NodePtr<T> {
    fn from(value: *mut Node<T>) -> Self {
        NodePtr(value)
    }
}

impl<T> Deref for NodePtr<T> {
    type Target = Node<T>;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}

impl<T> DerefMut for NodePtr<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.0 }
    }
}

impl<T> Drop for Node<T> {
    /// before drop solely, we must take care of both prev and next elements to
    /// connect correctly.
    fn drop(&mut self) {
        let this: NodePtr<T> = (self as *mut Self).into();

        if let Some(prev) = &self.prev {
            prev.as_ref().borrow_mut().next = this.next.clone();
        }
        if let Some(next) = &self.next {
            next.as_ref().borrow_mut().prev = this.prev.clone();
        }

        drop(&this);
    }
}

impl<T: Sized + Default> Node<T> {
    /// create dummy node
    fn new() -> Self {
        Self {
            next: None,
            prev: None,
            val: Default::default(),
            _marker: PhantomPinned,
        }
    }
    pub fn from_link(val: T, next: RcCell<NodePtr<T>>, prev: RcCell<NodePtr<T>>) -> Pin<Box<Self>> {
        Box::pin(Self {
            next: Some(next),
            prev: Some(prev),
            val: val,
            _marker: PhantomPinned,
        })
    }
}

impl<T: Sized + Default> MyLinkedList<T> {
    pub fn new() -> Self {
        let dummy_head = &mut Node::<T>::new() as *mut Node<T>;
        let dummy_tail = &mut Node::<T>::new() as *mut Node<T>;

        unsafe {
            (*dummy_head).next = Some(Rc::new(RefCell::new(dummy_tail.into())));
            (*dummy_tail).prev = Some(Rc::new(RefCell::new(dummy_head.into())));
        }

        let dummy_head: NodePtr<T> = dummy_head.into();
        let dummy_tail: NodePtr<T> = dummy_tail.into();

        Self {
            head: Rc::new(RefCell::new(dummy_head)),
            tail: Rc::new(RefCell::new(dummy_tail)),
        }
    }
}

impl<T> MyLinkedList<T> {
    pub fn get_head(&self) -> RcCell<NodePtr<T>> {
        self.head.clone()
    }
    pub fn get_tail(&self) -> RcCell<NodePtr<T>> {
        self.tail.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// This test causes memory invalid error at the end...
    #[test]
    fn testdrive() {
        let list = MyLinkedList::<i32>::new();
        let node = Node::<i32>::from_link(0, list.get_tail(), list.get_head());
        dbg!(node);
    }
}
