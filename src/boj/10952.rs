use std::io::{self, stdin};
fn main() -> io::Result<()> {
    for line in stdin().lines() {
        let case = line?
            .split(' ')
            .map(|e| e.parse::<i32>().expect("parse error"))
            .reduce(|acc, e| acc + e)
            .unwrap_or_default();
        if case == 0 {
            break;
        } else {
            println!("{}", case);
        }
    }

    Ok(())
}
