/// src: https://rust-unofficial.github.io/too-many-lists/second.html
/// objectives:
///     - make it generic
///     - advanced `Option` use
///     - lifetimes
///     - custom iterators

pub struct List {
    head: Link,
}

struct Node {
    elem: i32,
    next: Link,
}

type Link = Option<Box<Node>>;

impl List {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: i32) -> &mut Self {
        let new_node = Box::new(Node {
            elem,
            next: self.head.take(), // We no more have to depend on `mem::replace`
        });
        self.head = Some(new_node);
        self
    }

    pub fn pop(&mut self) -> Option<i32> {
        // Option::take can make some option type to other option type very easily
        // and elegantly
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
        // match &mut self.head {
        //     Some(node) => {
        //         let ret = node.elem;
        //         self.head = node.next.take();
        //         Some(ret)
        //     }
        //     None => None,
        // }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        list.push(4);
        list.push(5);

        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn drops() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);
        list.push(4);
        list.push(5);
        list.push(6);

        drop(list);
    }
}
