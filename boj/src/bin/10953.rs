use std::io::{self, stdin};
fn main() -> io::Result<()> {
    let t: i32;
    let mut line = String::new();
    stdin().read_line(&mut line)?;
    t = line.trim().parse::<i32>().expect("parse error!!");

    let mut itr = stdin().lines();

    for _ in 0..t {
        let case = itr
            .next()
            .unwrap()?
            .split(',')
            .map(|e| e.parse::<i32>().expect("parse error!"))
            .reduce(|acc, e| acc + e)
            .expect("reduce error!");
        println!("{}", case);
    }

    Ok(())
}
