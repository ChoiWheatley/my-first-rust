use std::io::{self, stdin};

fn main() -> io::Result<()> {
    for line in stdin().lines() {
        let submit = line?
            .split(' ')
            .map(|e| e.parse::<i32>().expect("cannot parse!"))
            .reduce(|acc, e| acc + e)
            .expect("reduce failed");
        println!("{}", submit);
    }

    Ok(())
}
