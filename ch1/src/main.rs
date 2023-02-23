/**
 * boj.kr/1000
 */
use std::io;

fn main() {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");

    let mut sum: u16 = 0;
    for i in line.split(' ') {
        sum += i.trim().parse::<u16>().expect("Failed to parse");
    }
    println!("{}", sum);
}
