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
