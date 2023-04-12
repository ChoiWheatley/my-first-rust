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
        // Option::map can make some option type to other option type very easily
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

impl<T> Default for List<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// IntoIter **consumes** data members
pub struct IntoIter<T>(List<T>);
/// Iter only borrows `&Node<T>` but we must ensure lifetime is valid during being used!
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}
/// IterMut borrows with mutation, have same lifetime as `Iter<'a, T>`
pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<T> List<T> {
    /// List contains head, which is type of `Option<Box<Node<T>>>` pew...
    ///
    /// 1. desired result: `Option<&'a Node<T>>`
    /// 2. head.as_ref() becomes `Option<&Box<Node<T>>>`
    /// 3. convert option type with map => node will be `&Box<Node<T>>` if node is Some
    /// 4. node.as_ref() becomes `&Node<T>`
    /// 5. return of map becomes `Option<&Node<T>>`
    ///
    /// However, `as_deref()` automatically dereferences inner type
    /// (from `Option<T>` to `Option<&T>`) so we don't need to do such thing!!! SICK
    ///
    /// Let's move on to the impl of [Iter<'a, T>]
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.head.as_deref(), // same as below
                                        // next: self.head.as_ref().map(|node| node.as_ref()),  // same as below
                                        // next: self.head.as_ref().map(|boxed_node| &**boxed_node),
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            next: self.head.as_deref_mut(),
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

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    /// just call `pop` over and over to ownership of top item
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    /// Finally, Iter gives us reference of elements!
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref(); // same as below
                                              // self.next = node.next.as_ref().map::<&Node<T>, _>(|node| &node);
            &node.elem
        })
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        // We have to *consume* self.next because it's mutable.
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            // `&` can be coppied, `&mut` can not be coppied
            &mut node.elem
        })
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

        if let Some(val) = list.peek_mut() {
            *val = 20230331;
        }
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

    #[test]
    fn iter() {
        let mut list = List::new();
        list.push("hi")
            .push("my")
            .push("name")
            .push("is")
            .push("choi")
            .push("wheatley");
        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&"wheatley"));
        assert_eq!(iter.next(), Some(&"choi"));
        assert_eq!(iter.next(), Some(&"is"));
        assert_eq!(iter.next(), Some(&"name"));
        assert_eq!(iter.next(), Some(&"my"));
        assert_eq!(iter.next(), Some(&"hi"));

        // iter dosen't consume anything!!!
        assert_eq!(list.peek(), Some(&"wheatley"));
    }

    #[test]
    fn iter_mut() {
        let mut list = List::new();
        list.push(0).push(1).push(2).push(3);
        let mut iter = list.iter_mut();
        let mut _top = iter.next().unwrap();
        *_top = 4;

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&4));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&0));
    }

    #[test]
    #[allow(clippy::option_as_ref_deref)]
    fn as_deref() {
        struct Node(i32);
        let original = Node(1);
        let option_box_node: Option<Box<Node>> = Some(Box::new(original));
        let _option_borrowed: Option<&Node> = option_box_node.as_ref().map(|e| &**e); // same as below
        let _option_borrowed: Option<&Node> = option_box_node.as_ref().map::<&Node, _>(|e| e); // same as below
        let _option_borrowed: Option<&Node> = option_box_node.as_deref();
    }
}
