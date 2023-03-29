#[derive(Debug)]
#[allow(unused)]
pub struct Test {
    value: String,
    pointer_to_value: *const String,
}

#[allow(unused)]
impl Test {
    pub fn new(txt: &str) -> Self {
        let mut this = Test {
            value: String::from(txt),
            pointer_to_value: std::ptr::null(),
        };
        this.pointer_to_value = &this.value;
        this
    }

    pub fn get_value(&self) -> &str {
        &self.value
    }

    pub fn get_pointer_to_value(&self) -> &String {
        unsafe { &*(self.pointer_to_value) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn init_test() {
        let test1 = Test::new("test1");
        let test2 = Test::new("test2");

        assert_eq!("test1", test1.get_value());
        assert_eq!("test1", test1.get_pointer_to_value());
        assert_eq!("test2", test2.get_value());
        assert_eq!("test2", test2.get_pointer_to_value());
    }

    #[test]
    /// 내용물은 스왑 되지만 정작 포인터의 값은 그대로 기존에 가리키던 것을 가리킨다.
    /// 근데 이건 C/C++에서도 해당하는거 아닌가? 아니네 ㅎㅅㅎ
    /// ```c++
    /// #include <cassert>
    /// #include <string>
    /// using namespace std;
    /// struct Test {
    ///   string s;
    ///   string *s_p;
    ///   explicit Test(string s) : s{s}, s_p{&this->s} {}
    /// };
    /// int main(void) {
    ///   Test test1("test1");
    ///   Test test2("test2");
    ///   std::swap(test1, test2);
    ///   assert("test2" == *test2.s_p);
    ///   assert("test1" == *test1.s_p);
    /// }
    /// ```
    fn swap_test() {
        let mut test1 = Test::new("test1");
        let mut test2 = Test::new("test2");

        std::mem::swap(&mut test1, &mut test2);

        assert_eq!("test2", test1.get_value());
        // assert_eq!("test2", test1.get_pointer_to_value()); // ERROR
        assert_eq!("test1", test2.get_value());
        // assert_eq!("test1", test2.get_pointer_to_value()); // ERROR
    }
}
