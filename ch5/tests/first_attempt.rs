#[test]
fn listing_5_8() {
    let w = 30;
    let h = 20;
    assert_eq!(w * h, area(w, h));
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
