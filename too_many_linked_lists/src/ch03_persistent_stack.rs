/// Will learn...
/// - Clear garbage pointers without GC via Reference counting.
/// - Persistent list [as this blog talks about](https://blog.hansenlin.com/persistent-data-structures-part-i-the-persistent-list-156f20df3139) which is a linked list with every nodes are head and every `next` nodes are tail, assures imutability.
/// - Know the use of Reference Counter, aka `Arc` struct
use std::sync::Arc;

type Link<T> = Option<Arc<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

pub struct List<T> {
    head: Link<T>,
}

#[allow(unused)]
impl<T> List<T> {
    /// create a new empty list, which can be a root
    pub fn new() -> Self {
        List { head: None }
    }

    /// create a new list containing an element, which also connects to previous list
    pub fn prepend(&self, elem: T) -> List<T> {
        List {
            head: Some(Arc::new(Node {
                elem,
                next: self.head.as_ref().map(|rc_node| rc_node.clone()),
            })),
        }
    }

    /// create a new list of this branch, except of head node
    pub fn tail(&self) -> List<T> {
        List {
            head: self.head.as_ref().and_then(|rc_node| rc_node.next.clone()),
        }
    }

    /// get a element inside head of this branch
    pub fn head(&self) -> Option<&'_ T> {
        self.head.as_ref().map(|rc_node| &rc_node.elem)
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

#[allow(unused)]
impl<T> List<T> {
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.head.as_deref(),
        }
    }
}

impl<T> Drop for List<T> {
    /// drop nodes which has only one link, this branch
    fn drop(&mut self) {
        let mut cur = self.head.take(); // take head's ownership
        while let Some(rc_node) = cur {
            // `try_unwrap` literally tries to unwrap Arc value if it has only one strong reference.
            if let Ok(mut node) = Arc::try_unwrap(rc_node) {
                cur = node.next.take();
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

        assert_eq!(branch1.head(), Some(&4));
        assert_eq!(branch1.tail().head(), Some(&3));
        assert_eq!(branch1.tail().tail().head(), Some(&2));
        assert_eq!(branch1.tail().tail().tail().head(), Some(&1));

        assert_eq!(branch2.head(), Some(&5));
        assert_eq!(branch2.tail().head(), Some(&3));
        assert_eq!(branch2.tail().tail().head(), Some(&2));
        assert_eq!(branch2.tail().tail().tail().head(), Some(&1));
    }

    #[test]
    fn iter() {
        let queue = "Hello, world!".chars().collect::<Vec<_>>();
        let mut list = List::new();

        queue.iter().for_each(|c| {
            list = list.prepend(c);
        });
        let answer_iter = queue.iter().rev();
        let list_iter = list.iter();
        assert!(list_iter
            .zip(answer_iter)
            .all(|(&list, answer)| list == answer));
    }
}
