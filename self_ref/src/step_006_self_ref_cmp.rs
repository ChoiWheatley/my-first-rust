/// [[step_005_self_ref_holder]] 의 단점은 BTreeSet의 Key가 포인터 타입이라는 데에 있다.
///
/// 따라서 우리가 어떤 멤버를 키값으로 BTreeSet을 구축하고 싶어 별도의 Ord 트레이트를
/// 구현할지라도 `BTreeSet<*mut Me>` 에 의해 결국은 주소값을 비교하게 될 것이고,
/// 의도하지 않은 결과를 얻을 것이다. 따라서 `*mut Me`를 감싸는 래퍼 구조체를 만들어
/// 래퍼 자체에 `Ord` 트레이트를 구현하게 만들고 자동 형변환을 구현할 수 있는 `From`
/// 트레이트를 구현하여 사용에 편의를 제공할 수 있다.
///
/// source: https://dev.to/arunanshub/self-referential-structs-in-rust-part-2-1lc2
use std::{
    cell::RefCell, collections::BTreeSet, marker::PhantomPinned, ops::Deref, pin::Pin, rc::Rc,
};
type RcCell<T> = Rc<RefCell<T>>;

/// 다음 모듈은 `Ord` 트레이트의 속성에 대하여 실험을 진행하는 코드이다.
/// 복합 구조체 `Node`에 대하여 노드의 어떤 속성을 기준으로 비교를 진행하는지
/// 확인한 결과, 모든 멤버들에 대한 비교를 진행하는 것으로 확인됐다.
///
/// [Ord](https://doc.rust-lang.org/std/cmp/trait.Ord.html)의 첫번째 줄에서
/// 이미 다음 글귀를 확인할 수 있었다. (좀 일찍 볼걸..)
///
/// > Trait for types that form a **total order**
/// > 전순서 집합을 형성하는 트레이트
mod tutorial {

    #[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
    struct Node {
        name: String,
        id: u32,
    }
    #[cfg(test)]
    mod tests {
        use std::collections::BTreeSet;

        use super::*;
        #[test]
        fn run() {
            let mut holder: BTreeSet<&Node> = BTreeSet::new();
            let nodes = [
                Node {
                    name: "n1".to_owned(),
                    id: 1,
                },
                Node {
                    name: "n2".to_owned(),
                    id: 2,
                },
                Node {
                    name: "n3".to_owned(),
                    id: 3,
                },
            ];
            nodes.iter().for_each(|each| {
                holder.insert(each);
            });

            assert_eq!(3, holder.len());

            // what if we insert same `name` node in holder?
            let dup1 = Node {
                name: "n1".to_owned(),
                id: 123124,
            };
            holder.insert(&dup1);

            assert_eq!(4, holder.len()); // Ok

            // what if we insert same `id` node in holder?
            let dup2 = Node {
                name: "dup2".to_owned(),
                id: 2,
            };
            holder.insert(&dup2);

            assert_eq!(5, holder.len());

            // what if we insert same both `name` and `id` in holder?
            let dup3 = Node {
                name: "n3".to_owned(),
                id: 3,
            };
            holder.insert(&dup3);

            assert_eq!(5, holder.len()); // didn't inserted!!
        }
    }
}

#[derive(Debug, Derivative)]
#[derivative(PartialEq, Eq, PartialOrd, Ord)]
#[allow(unused)]
pub struct Me {
    name: String,
    mutate_by_holder: i32,
    #[derivative(Ord = "ignore")]
    my_holder: RcCell<Holder>,
    _pinned: PhantomPinned,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
#[allow(unused)]
pub struct Holder {
    set_of_me: BTreeSet<MeWrapper>,
}

#[repr(transparent)] // layout of ABI is guaranteed to be the same as that one field
#[derive(Debug)]
struct MeWrapper(*mut Me);

///
/// BOILERPLATE START
///

impl PartialEq for MeWrapper {
    fn eq(&self, other: &Self) -> bool {
        let this = &**self;
        let other = &**other;
        this == other
    }
}
impl PartialOrd for MeWrapper {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let this = &**self;
        let other = &**other;
        this.partial_cmp(&other)
    }
}
impl Eq for MeWrapper {} // 얘는 왜 아무것도 구현 안해도 되냐?
impl Ord for MeWrapper {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let this = &**self;
        let other = &**other;
        this.cmp(&other)
    }
}
impl Deref for MeWrapper {
    type Target = Me;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
impl From<*mut Me> for MeWrapper {
    fn from(value: *mut Me) -> Self {
        MeWrapper(value)
    }
}
/// [[step_005_self_ref_holder.rs]] 와는 다른 드롭 코드
/// 이젠 `*mut Me`를 remove하지 않고 `MeWrapper`를 remove한다.
impl Drop for Me {
    fn drop(&mut self) {
        let this: MeWrapper = (self as *mut Self).into();
        self.my_holder.borrow_mut().set_of_me.remove(&this);
    }
}

///
/// BOILERPLATE END
///

#[allow(unused)]
impl Holder {
    pub fn new() -> RcCell<Self> {
        Rc::new(RefCell::new(Self {
            set_of_me: BTreeSet::new(),
        }))
    }

    pub fn mutate_value_of_me(&self, val: i32) {
        self.set_of_me.iter().for_each(|each| {
            let each = unsafe { Pin::new_unchecked(&mut *each.0) };
            each.mutate_me(val);
        });
    }
}

#[allow(unused)]
impl Me {
    pub fn new<S>(my_holder: RcCell<Holder>, name: S) -> Pin<Box<Self>>
    where
        S: Into<String>,
    {
        let mut this = Box::pin(Self {
            name: name.into(),
            mutate_by_holder: 0,
            my_holder,
            _pinned: PhantomPinned,
        });
        let this_ptr: *mut _ = unsafe { this.as_mut().get_unchecked_mut() };
        this.my_holder
            .borrow_mut() // borrow holder mutably once
            .set_of_me
            .insert(this_ptr.into()); // borrow holder mutably twice, this can cause panic!
                                      // https://doc.rust-lang.org/nightly/core/cell/struct.RefCell.html#method.borrow
                                      // To solve this problem, we can *exclude* me.my_holder from cmp
                                      // 1. use [derivative](https://crates.io/crates/derivative) crate for ignore it
                                      // 2. manually ignore it in Ord::cmp for Me

        this
    }

    fn mutate_me(self: Pin<&mut Self>, val: i32) {
        let this = unsafe { self.get_unchecked_mut() };
        this.mutate_by_holder += val;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn run() {
        let holder = Holder::new();

        let a = Me::new(Rc::clone(&holder), "a");
        let b = Me::new(Rc::clone(&holder), "b");
        let c = Me::new(Rc::clone(&holder), "b"); // What about duplicated object?

        // set all `Me`s with value 1
        holder.borrow().mutate_value_of_me(1);
        assert_eq!(1, a.mutate_by_holder);
        assert_eq!(1, b.mutate_by_holder);
        assert_eq!(0, c.mutate_by_holder); // now c is not a memeber of holder

        // make ref of holder
        let holder_ref = Rc::clone(&holder);
        assert_eq!(5, Rc::strong_count(&holder)); // c is still pointing holder right now

        dbg!(a.as_ref().get_ref() as *const Me);
        dbg!(b.as_ref().get_ref() as *const Me);
        dbg!(c.as_ref().get_ref() as *const Me);
        dbg!(holder_ref);
    }
}
