use std::io::{self, stdin};
fn main() -> io::Result<()> {
    let t: i32;
    let mut line = String::new();
    stdin().read_line(&mut line)?;
    t = line.trim().parse::<i32>().unwrap();

    let mut itr = stdin().lines();
    for _ in 0..t {
        let result = itr
            .next()
            .unwrap()?
            .split(' ')
            .map(|item| item.parse::<i32>().unwrap())
            .reduce(|acc, e| acc + e)
            .unwrap();
        println!("{}", result);
    }

    Ok(())
}
