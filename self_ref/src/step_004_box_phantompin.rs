use std::{marker::PhantomPinned, pin::Pin};

#[derive(Debug)]
#[allow(unused)]
struct Test {
    value: String,
    pointer_to_value: *const String,
    _pinned: PhantomPinned,
}

#[allow(unused)]
impl Test {
    fn new(txt: &str) -> Pin<Box<Self>> {
        let mut this = Box::pin(Test {
            value: String::from(txt),
            pointer_to_value: std::ptr::null(),
            _pinned: PhantomPinned,
        });
        unsafe {
            this.as_mut().get_unchecked_mut().pointer_to_value = &this.value;
        }
        this
    }

    fn get_value(self: Pin<&Self>) -> &str {
        &self.get_ref().value
    }

    fn get_pointer_to_value(self: Pin<&Self>) -> &String {
        unsafe { &*(self.pointer_to_value) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn testdrive() {
        let mut test1 = Test::new("test1");
        let mut test2 = Test::new("test2");

        assert_eq!("test1", test1.as_ref().get_value());
        assert_eq!("test1", *test1.as_ref().get_pointer_to_value());
        assert_eq!("test2", test2.as_ref().get_value());
        assert_eq!("test2", *test2.as_ref().get_pointer_to_value());

        // swap
        std::mem::swap(&mut test1, &mut test2);

        // raw_pointers.rs 에서와는 다르게 포인터가 멤버를 제대로 가리키고 있음을 알 수 있다.
        assert_eq!("test2", test1.as_ref().get_value());
        assert_eq!("test2", *test1.as_ref().get_pointer_to_value()); // Ok
        assert_eq!("test1", test2.as_ref().get_value());
        assert_eq!("test1", *test2.as_ref().get_pointer_to_value()); // Ok

        // PhantomPin을 사용한 이래로 우리는 deref를 통한 데이터 변경 권한을 잃었다.
        // test1.as_mut().get_mut().value = "new test1".to_owned(); // ERROR
        // test2.as_mut().get_mut().value = "new test2".to_owned(); // ERROR

        // 만약 꼭 바꿔야 한다면... unsafe를 쓰는 수밖에 없지
        unsafe {
            test1.as_mut().get_unchecked_mut().value = "new test1".to_owned();
        }
        assert_eq!("new test1", test1.as_ref().get_value());
        assert_eq!("new test1", test1.as_ref().get_pointer_to_value());
    }
}
