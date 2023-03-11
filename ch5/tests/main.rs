pub mod first_attempt {
    #[test]
    fn listing_5_8() {
        let w = 30;
        let h = 20;
        assert_eq!(w * h, area(w, h));
    }

    fn area(width: u32, height: u32) -> u32 {
        width * height
    }
}

pub mod second_attempt {

    use ch5::rectangle::Rectangle;

    #[test]
    fn listing_5_10() {
        let rect = Rectangle {
            width: 30,
            height: 50,
        };
        assert_eq!(1500, area(&rect));
        println!("rect = {:?}", rect);
        println!("rect = {:#?}", rect);
        dbg!(&rect);
    }

    fn area(rect: &Rectangle) -> u32 {
        rect.width * rect.height
    }
}

pub mod third_attempt {
    use ch5::rectangle::Rectangle;

    /**
     * 모듈이 서로 다른 곳에 위치해서 바로 impl Rectangle 할 수 없었다.
     * error[E0116]: cannot define inherent `impl` for a type outside of the crate where the type is defined
     *
     * 친절하게도, trait를 정의하라고 설명해줘서 추가했더니 정상적으로 컴파일이 된다.
     */
    trait Area {
        fn area(&self) -> u32;
    }
    trait Hold {
        fn can_hold(&self, other: &Self) -> bool;
    }

    impl Area for Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }
    }

    impl Hold for Rectangle {
        fn can_hold(&self, other: &Self) -> bool {
            self.width > other.width && self.height > other.height
        }
    }

    #[test]
    fn listing_5_13() {
        let mut rect = Rectangle {
            width: 30,
            height: 50,
        };
        assert_eq!(1500, Area::area(&rect));
        assert_eq!(1500, rect.area());
        assert_eq!(1500, dbg!(&rect).area());
        rect.width = 300;
        assert_eq!(15000, dbg!(&rect).area());
    }

    #[test]
    fn listing_5_14() {
        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };
        let rect2 = Rectangle {
            width: 10,
            height: 40,
        };
        let rect3 = Rectangle {
            width: 60,
            height: 45,
        };

        assert!(rect1.can_hold(&rect2));
        println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
        assert!(!rect1.can_hold(&rect3));
        println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    }
}

pub mod using_associated_function {
    use ch5::rectangle::Rectangle;

    #[test]
    fn constructor() {
        dbg!(Rectangle::new(30, 50));
        dbg!(Rectangle::square(72));
    }
}
