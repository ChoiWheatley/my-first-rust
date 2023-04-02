# Learning Rust With Entirely Too Many Linked Lists by choiwheatley

- src 
    - https://rust-unofficial.github.io/too-many-lists/index.html
- objective 
    - Learn rust with rust's well-known hardcore subject: "Why linked lists in rust hard?"
    - Learn my own data structures which supports self-referencing linked lists
    - Self instructing with high difficultiness
    - Learn about rust's strong type system and borrowing mechanism with lifetime indicator
    - Most important thing as learning is BE HONEST. freaking out because of compiler error doesn't make us productive.

# ch01_bad_stack

bad, which means that this module uses custom optional type `Link`. 

```rust
enum Link {
    Empty,
    Next(Box<Node>),
}
```

This is bad because we cannot use `Option`'s proprietary useful methods such as `as_deref` or `map`. Only left for us is match closure, which is bad.

Anyway, I learned something useful such as [replace](https://doc.rust-lang.org/nightly/core/mem/fn.replace.html) and [take](https://doc.rust-lang.org/nightly/core/mem/fn.take.html) and [swap](https://doc.rust-lang.org/nightly/core/mem/fn.swap.html). Those guys make memory manipulation with safe manner. under the hood, it uses `unsafe` block, but it guarentee that no other artifacts would happen.

# ch02_ok_linked_stack

Ok, because we finally make our `Link` type as `Option`

```rust
type Link<T> = Option<Box<Node<T>>>;
```

The intension on this `List` is "we hide details such as `Link` and `Node` behind and give users permissions of **iterate** through".

I learned a lot with this module. 

- generic type itself isn't such a fuss

- We can replace `mem::replace` with `Option`'s prominent method [replace](https://doc.rust-lang.org/std/option/enum.Option.html#method.replace) and [take](https://doc.rust-lang.org/std/option/enum.Option.html#method.take)
- [Option::map](https://doc.rust-lang.org/std/option/enum.Option.html#method.map) can make some option type from original option type very easily without `match` block.

- `as_ref` from [AsRef](https://doc.rust-lang.org/std/convert/trait.AsRef.html#tymethod.as_ref) trait can convert unmovable *field* to reference, same concept as [AsMut](https://doc.rust-lang.org/std/convert/trait.AsMut.html)

- Iterator has several brothers, which needs own struct aka **adaptor** 
    - `IntoIter` let users **consume** the content of list, we implemented this with our `pop` method. `IntoIter` must have to implement [IntoIterator](https://doc.rust-lang.org/std/iter/trait.IntoIterator.html) trait.

    - `Iter` is most common iterator that we have ever met. It doesn't give us ownership, so it only **borrows** references of each element. One thing promising was of its lifetime indicator `'a`, without that, the compiler cannot infer how the referenced variable live long enough. So we had to explicitly set "This lives as long as `&Node<T>` lives.    
    `Iter` must have to implement [Iterator](https://doc.rust-lang.org/std/iter/trait.Iterator.html) trait.
    ```rust
    pub struct Iter<'a, T> {
        next: Option<&'a Node<T>>,
    }
    ```
    
    - `IterMut` let users **mutate** through each element. Nothing special comparing to `Iter` adaptor, except "mutated reference only exist solely". So when we iterate through, we can make mistake very easily as we implicitly `Copy` references. `&` can be coppied, but `&mut` cannot. 
    ```rust
    let mut owner = 4;
    let mut borrower1 = &mut owner;
    let mut borrower2 = &mut owner; 
    *borrower1 = 1; // cannot borrow `owner` as mutable more than once at a time
    *borrower2 = 2;
    ```
    
- [Deref](https://doc.rust-lang.org/std/ops/trait.Deref.html#more-on-deref-coercion) is fantastic tool for eliminating dereference clutters. Found out that multiple deref can be done only one `deref` method call...
```rust
struct Node(i32);
let original = Node(1);
// dirty, but we have see this for entire life
let option_box_node: Option<Box<Node>> = Some(Box::new(original)); 
// *e => Box, **e => Node , &**e => &Node
let option_borrowed: Option<&Node> = option_box_node.as_ref().map(|e: &Box<Node>| &**e); 
// turbo fish makes compiler infer easily, but still too verbose
let option_borrowed: Option<&Node> = option_box_node.as_ref().map::<&Node, _>(|e| e); 
// Box, Node automatically dereferenced. `as_deref` converts `Option<T>` to `Option<&T::Target>` where Target is our desired type 
let option_borrowed: Option<&Node> = option_box_node.as_deref(); 
```

# ch03_persistent_stack

I have never experience of persistent stack, so I searched for it. [Hans Enline](https://blog.hansenlin.com/persistent-data-structures-part-i-the-persistent-list-156f20df3139) thankfully elaborates concepts of 'em.

Persistent list is like a linked-list, but every node is a list. What does that mean? A list can contain sub lists which can be from 0-sized to infinite-sized of list. Only important thing is connections. Each list doesn't care about other lists. They don't care their neighbor is empty, has thousands of branches, whatever.

Branches!! Yes! Git brances are persistent list! every git commit is HEAD of its own list. You can manually branch from existing node that you want. You can iterate through git logs which only contains the differences of previous commit. The final code in your repository's HEAD is just a summation of whole diffs which cannot be changed forever. This is a persistent list and you and me are using nowadays.

So, there was the use of persistent list, then how to impliment it? What is some common protocols?

In this example, we use `prepend`, `tail` and `head` for interfaces, repectively similar to `push`, `pop`, `get`.

TODO: right a doc about things I have learned in this chapter