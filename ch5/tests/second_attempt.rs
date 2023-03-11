use ch5::Rectangle;

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
