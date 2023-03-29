use std::{cell::RefCell, collections::BTreeSet, marker::PhantomPinned, pin::Pin, rc::Rc};
type RcCell<T> = Rc<RefCell<T>>;

#[derive(Debug)]
#[allow(unused)]
pub struct Me {
    name: String,
    mutate_by_holder: i32,
    my_holder: RcCell<Holder>,
    _pinned: PhantomPinned,
}

#[derive(Debug)]
#[allow(unused)]
pub struct Holder {
    set_of_me: BTreeSet<*mut Me>,
}

#[allow(unused)]
impl Me {
    pub fn new(my_holder: RcCell<Holder>, name: impl Into<String>) -> Pin<Box<Self>> {
        let mut this = Box::pin(Self {
            name: name.into(),
            mutate_by_holder: 0,
            my_holder,
            _pinned: PhantomPinned,
        });
        let this_ptr: *mut _ = unsafe { this.as_mut().get_unchecked_mut() };
        this.my_holder.borrow_mut().set_of_me.insert(this_ptr);
        this
    }

    /// Allows you to mutate a value within Me.
    /// Run this from `Holder` to see what happens.
    fn mutate_me(self: Pin<&mut Self>, val: i32) {
        let this = unsafe { self.get_unchecked_mut() };
        this.mutate_by_holder += val;
    }
}

#[allow(unused)]
impl Holder {
    pub fn new() -> RcCell<Self> {
        Rc::new(RefCell::new(Self {
            set_of_me: Default::default(),
        }))
    }

    /// Mutate every value of `Me`
    /// Note how a pinned value is reconstructed.
    pub fn mutate_value_of_me(&self, val: i32) {
        self.set_of_me.iter().for_each(|a| {
            let a = unsafe { Pin::new_unchecked(&mut **a) };
            a.mutate_me(val);
        });
    }
}

/// Me 객체를 제거할 때 Holder의 원소도 제거하도록 구현
impl Drop for Me {
    fn drop(&mut self) {
        let this = &(self as *mut _);
        self.my_holder.borrow_mut().set_of_me.remove(this);
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
        assert_eq!(1, c.mutate_by_holder);

        // make ref of holder
        let holder_ref = Rc::clone(&holder);
        assert_eq!(5, Rc::strong_count(&holder));

        dbg!(a, b, holder_ref);
    }
}
