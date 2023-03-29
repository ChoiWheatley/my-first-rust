/// `Pin` 타입은 러스트 기본속성인 movable에 제약을 두기 위해 만들어졌다.
/// Move하면 안되는 상황이 바로 Self-referential struct를 사용할 때가 대표적임.
/// 포인터 타입 `P` (Box, Rc, Arc, RefCell, ...)을 Pin으로 래핑하면 스왑과 같은
/// 복잡한 상황에서 원치 않는 포인터의 move를 막아준다고..
///
/// 다만, Pinned 포인터들이라 할지라도 Unpin이 되어 move될 수 있다. 기본적으로
/// 모든 타입들은 Unpin auto-trait들이 구현되어있기 때문에 Pin<P>의 효과가
/// 무효화 되는 것이다. 열심히 pin한 포인터가 Unpin 되는 사태를 막기 위해
/// [[box_phantompin.rs]] 파일에서 실습을 진행한다.
use std::pin::Pin;

#[derive(Debug)]
#[allow(unused)]
struct Test {
    value: String,
    pointer_to_value: *const String,
}

#[allow(unused)]
impl Test {
    fn new(txt: &str) -> Pin<Box<Self>> {
        let mut this = Box::pin(Test {
            value: String::from(txt),
            pointer_to_value: std::ptr::null(),
        });
        this.as_mut().pointer_to_value = &this.value as *const String;
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

        // set by using `as_mut().get_mut()` is DANGEROUS
        // because `Unpin` auto trait is implemented right now!
        test1.as_mut().get_mut().value = "new test1".to_owned();
        test2.as_mut().get_mut().value = "new test2".to_owned();
        assert_eq!("new test1", test1.as_ref().get_value());
        assert_eq!("new test1", test1.as_ref().get_pointer_to_value());
        assert_eq!("new test2", test2.as_ref().get_value());
        assert_eq!("new test2", test2.as_ref().get_pointer_to_value());

        // swap unpinned pointers cause pointers MOVE
        std::mem::swap(test1.as_mut().get_mut(), test2.as_mut().get_mut());
        assert_eq!("new test2", test1.as_ref().get_value());
        // assert_eq!("new test2", test1.as_ref().get_pointer_to_value()); // ERROR!
        assert_eq!("new test1", test2.as_ref().get_value());
        // assert_eq!("new test1", test2.as_ref().get_pointer_to_value()); // ERROR!
    }
}
