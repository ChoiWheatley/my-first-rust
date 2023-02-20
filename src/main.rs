/**
 * boj.kr/1000
 */
use std::io;

fn main() {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");

    let splitted: Vec<&str> = line.split(' ').collect();

    let a: u16 = splitted[0].trim().parse().unwrap();
    let b: u16 = splitted[1].trim().parse().unwrap();

    println!("{}", a + b);
}
