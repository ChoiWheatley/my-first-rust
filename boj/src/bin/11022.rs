use std::io::{self, stdin};
fn main() -> io::Result<()> {
    let t: i32;
    let mut line = String::new();
    stdin().read_line(&mut line)?;
    t = line.trim().parse::<i32>().expect("parse error!");

    let mut itr = stdin().lines();
    for tc in 1..=t {
        let mapped: Vec<i32> = itr
            .next()
            .unwrap()?
            .split(' ')
            .map(|e| e.parse::<i32>().expect("parse error!"))
            .collect();
        println!(
            "Case #{}: {} + {} = {}",
            tc,
            mapped[0],
            mapped[1],
            mapped[0] + mapped[1]
        );
    }

    Ok(())
}
