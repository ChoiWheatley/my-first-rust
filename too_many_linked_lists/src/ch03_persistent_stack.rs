/// Will learn...
/// - Clear garbage pointers without GC via Reference counting.
/// - Persistent list [as this blog talks about](https://blog.hansenlin.com/persistent-data-structures-part-i-the-persistent-list-156f20df3139) which is a linked list with every nodes are head and every `next` nodes are tail, assures imutability.
/// - Know the use of Reference Counter, aka `Rc` struct
use std::rc::Rc;

type Link<T> = Option<Rc<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

/// Persistent List, which conforms Functional Persistent.
pub struct List<T> {
    head: Link<T>,
}

#[allow(unused)]
impl<T> List<T> {
    /// create a empty list, which can be the beginning of persistent structure
    pub fn new() -> Self {
        List { head: None }
    }

    /// create a new list with elem, append this from previous list's head
    ///
    /// A.prepend(B) returns B -> A
    pub fn prepend(&self, elem: T) -> List<T> {
        List {
            head: Some(Rc::new(Node {
                elem,
                next: self.head.clone(), // this is a stack-like persistent list!
            })),
        }
    }

    /// create a new list with whole list, except head
    ///
    /// (B -> A).tail() returns A
    pub fn tail(&self) -> List<T> {
        List {
            // `and_then` is very similar to `map`, but allows us to pass a
            // function which returns an `Option` type itself. `and_then`
            // flattens the result which is also known as `flatmap`
            head: self.head.as_ref().and_then(|node| node.next.clone()),
        }
    }

    /// get an head's element
    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }
}

/// adaptor for iterating `List`
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_ref().map(|rc_node| rc_node.as_ref());
            &node.elem
        })
    }
}

#[allow(unused)]
impl<T> List<T> {
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.head.as_ref().map(|rc_node| rc_node.as_ref()),
        }
    }
}

impl<T> Drop for List<T> {
    /// drop all lists which is only owned by once in this branch
    fn drop(&mut self) {
        let mut cur = self.head.take();
        while let Some(node) = cur {
            if let Ok(node) = Rc::try_unwrap(node) {
                cur = node.next.map(|rc_node| rc_node);
            } else {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prepend_head_tail() {
        let root = List::new();
        let list = root.prepend(1).prepend(2).prepend(3);
        assert_eq!(list.head(), Some(&3));
        assert_eq!(list.tail().head(), Some(&2));
        assert_eq!(list.tail().tail().head(), Some(&1));

        // Make sure empty tail works
        assert_eq!(list.tail().tail().tail().head(), None);

        // chaining an empty list can be possible, keep returning None, that's all
        assert_eq!(list.tail().tail().tail().tail().head(), None);

        // branching from node 3
        let branch1 = list.prepend(4);
        let branch2 = list.prepend(5);
    }
}
