/// src: https://rust-unofficial.github.io/too-many-lists/second.html
/// objectives:
///     - make it generic
///     - advanced `Option` use
///     - lifetimes
///     - custom iterators

pub struct List<T> {
    head: Link<T>,
}

struct Node<T> {
    elem: T,
    next: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: T) -> &mut Self {
        let new_node = Box::new(Node {
            elem,
            next: self.head.take(), // We no more have to depend on `mem::replace`
        });
        self.head = Some(new_node);
        self
    }

    pub fn pop(&mut self) -> Option<T> {
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

    /// get a borrowed data from top
    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|boxed_node| &boxed_node.elem)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|boxed_node| &mut boxed_node.elem)
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}

/// IntoIterator only exposes method `into_iter`.
/// Concrete implementations are in custom iterator `IntoIter<T>`
impl<T> IntoIterator for List<T> {
    type Item = T;

    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self)
    }
}

/// IntoIter **consumes** data members
pub struct IntoIter<T>(List<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    /// just call `pop` over and over to ownership of top item
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
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

    #[test]
    fn peek() {
        let mut list = List::new();
        assert_eq!(list.pop(), None);
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);

        list.push(1).push(2).push(3);

        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.peek_mut(), Some(&mut 3));

        list.peek_mut().map(|val| *val = 20230331);
        assert_eq!(list.peek(), Some(&20230331));
        assert_eq!(list.pop(), Some(20230331));
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push(1).push(2).push(3);

        let mut iter = list.into_iter();
        // now `iter` loses its ownership UwU
        assert_eq!(Some(3), iter.next());
        assert_eq!(Some(2), iter.next());
        assert_eq!(Some(1), iter.next());

        assert_eq!(None, iter.next());
    }
}
