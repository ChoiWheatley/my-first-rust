#[test]
pub fn sample_test() {
    assert_eq!(1, 1);
}

/**
* 변수에 대한 aliasing과 mutation은 공존할 수 없다는 것을 잘 알려준다.
> Mutable references allow mutation but prevent aliasing. The borrowed
path `vec` becomes temporarily unusable, so effectively not an alias
*/
#[test]
fn mutable_references() {
    let mut vec: Vec<i32> = vec![1, 2, 3];

    let num: &mut i32 = &mut vec[2];
    /*
     * When `num` was an immutable reference, `vec` still had read permissions.
     * Now that `num` is a mutable reference, `vec` has lost *all* permissions
     * while `num` is in use.
     */
    *num += 1;
    /*
     * When `num` was an immutable reference, the path `*num` only had read/own
     * permissions. Now that `num` is a mutable reference, `*num` has also
     * gained write permissions
     */

    println!("Third elemet is {}", *num);

    println!("Vector is now {:?}", vec);
}

#[test]
fn temporarily_downgraded_to_readonly_reference() {
    let mut vec: Vec<i32> = vec![1, 2, 3];
    /*
     * vec: +R +W +O
     */

    let num: &mut i32 = &mut vec[2];
    /*
     * vec: -R -W -O
     * num: +R -W +O
     * *num: +R +W +O
     */

    let num2: &i32 = &*num;
    /*
     * num: +R -W -O // `*&num` removes the write permission from `*num` but not the read permission
     * *num: +R -W +O
     * num2: +R -W +O
     * *num2: +R -W +O
     */

    println!("{} {}", *num, *num2);

    assert_eq!(num, num2);
}

#[test]
fn lifetime() {
    let mut x = 1;

    let y = &x;
    /*
     * lifteime of y starts
     */

    let z = *y;
    /*
     * lifetime of y ends, the write permission on x are retured to x
     */

    x += z;

    assert_eq!(x, 2);
}

#[test]
fn lifetime2() {
    let mut v: Vec<char> = vec!['a', 's', 'c', 'i', 'i'];

    let c = &mut v[0];

    if c.is_ascii_lowercase() {
        // `c` lives here, v lose permission R, W, O
        // println!("Before capitalized: {:?}", v); // Compile error
        let up = c.to_ascii_uppercase();

        *c = up;
        println!("After capitalized: {:?}", v);
    } else {
        // `c` is not used, v regain permission R, W, O
        println!("Already capitalized : {:?}", v);
    }
}

#[test]
fn quiz() {
    let mut s: String = String::from("Hello");

    let t: &mut String = &mut s;

    /* here, s loses its permission R, W, O */
    t.push_str(" World");

    println!("{}", s);
}

#[test]
fn data_must_outlive_any_reference_to_it() {
    /*
     * this function's return type contains a borrowed value,
    but there is no value for it to be borrowed from.
     * 이거 마치 cpp에서 레퍼런스를 리턴할때 발생하는 에러같은데?
    함수의 스택프레임이 끝나면 해당 레퍼런스가 제거되는 문제.
    */
    // fn return_a_string() -> &String {
    //     let s = String::from("Hello, World");
    //     let s_ref = &s;
    //     s_ref
    // }
    /*
     * `n` does not live long enough
     */
    // fn add_ref(v: &mut Vec<&i32>, n: i32) {
    //     let r = &n;
    //     v.push(r);
    // }
}
