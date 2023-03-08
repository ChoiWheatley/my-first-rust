#[test]
fn integer_is_copyable() {
    let v: Vec<i32> = vec![0, 1, 2];

    let n_ref: &i32 = &v[0];

    let n: i32 = *n_ref; // Ok, &i32 is Copyable

    println!("n is {}", n); // Ok
}

#[test]
fn string_is_not_copyable() {
    let v: Vec<String> = vec![String::from("Hello"), String::from("World")];

    let s_ref: &String = &v[0]; // v의 첫번째 원소인 "Hello"에 대한 소유권을 잠깐 빌린다.

    /* cannot move out of `*s_ref` which is behind a shared reference */
    // let s: String = *s_ref; // `String` does own heap data, so it cannot be copied without a move

    let s: &String = &*s_ref; // s는 힙영역의 문자열을 가리킬 뿐, v에 소유권을 빌려가지 않는다.

    println!("v is {:?}", v);

    println!("s is {}", s);

    println!("s_ref is {}", *s_ref);

    println!("s is {}", s);

    println!("drop(s)");
    drop(s);

    println!("v is {:?}", v);
    drop(v);
}

#[test]
fn mutable_string_ref() {
    // 우리도 cpp에서 vector<string &> 이렇게 적지는 않잖아
    let mut v: Vec<String> = vec![String::from("Hello"), String::from("World")];

    let mut s_ref = &mut v[0];
    *s_ref = "AA".to_owned();

    println!("v: {:?}", v);

    s_ref = &mut v[1];
    *s_ref = "BB".to_owned();

    println!("v: {:?}", v);
    /*
    cannot borrow `v` as mutable more than once at a time.
    만약 정 v[1]의 레퍼런스를 가져와야겠다면 `split_at_mut` 메서드를
    활용하는 방법을 생각해 볼 수 있다.
    */
    // let mut s_ref1 = &mut v[1];
    let (slice1, slice2) = v.split_at_mut(1);
    slice1[0] = "CC".to_owned();
    slice2[0] = "DD".to_owned();

    println!("v: {:?}", v);

    /*
    또는 `iter_mut` 메서드를 사용할 수도 있다.
     */
    let mut itr = v.iter_mut();
    *itr.next().unwrap() = "EE".to_owned();
    *itr.next().unwrap() = "FF".to_owned();

    println!("v: {:?}", v);
}

#[test]
fn mutable_references1() {
    let mut n = 0;
    {
        let a = &mut n; //move occurs because `a` has type `&mut i32`, which does not implement the `Copy` trait
        let b = a; // ownership of a is moved to b
        assert_eq!(0, *b);
        // assert_eq!(0, *a); // Err, a already handed ownership to b
    }

    {
        let a = &mut n;
        let b: &mut i32 = a; // ownership of &mut *a(implicitly) is moved to b
        assert_eq!(0, *b);
        // lifetime of b ended, a regains permission
        assert_eq!(0, *a); // Ok, a borrows ownership of n now

        *a = 2;
    }
    assert_eq!(2, n);
}

#[test]
fn mutable_references2() {
    fn test(_: &mut i32) {}

    let mut i = 1;
    let ref_i = &mut i;
    // test(&mut i); // Err! i is currently read-locked

    test(ref_i); // same as `&mut *ref_i`

    let another_ref_i = &mut i;
    test(another_ref_i); // same as `&mut *another_ref_i`
}

#[test]
fn immutable_reference() {
    fn test(_: &i32) {}

    let i = 1;
    let ref_i = &i;
    test(&i); // Ok, i is currently not read-locked
    test(ref_i); // immutable ref COPIED into parameter

    let another_ref_i = &i;
    test(another_ref_i); // immutable ref COPIED into parameter
}

#[test]
fn outlive() {
    let mut i = 1;

    // mutable reference cannot outlive
    // let j = {
    //     let x = &mut i;
    //     let y = &x;
    //     &**y
    // }; // x is dropped. y become dangling pointer

    // immutable reference COPIED out
    let j = {
        let x = &i;
        let y = &x;
        &**y
    }; // x dropped, but y has

    println!("{}", j);
}

#[test]
fn move_string_out_of_the_vec() {
    let mut v: Vec<String> = vec![String::from("Hello world")];
    let mut s: String = v.remove(0);
    s.push('!');
    println!("{s}");
    assert!(v.len() == 0);
}
