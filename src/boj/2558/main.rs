use std::io;

fn main() {
    // let line = String::new();
    let mut sum = 0;
    let mut itr = io::stdin().lines();
    for _ in 0..2 {
        let num = itr.next().unwrap().unwrap().trim().parse::<i32>().unwrap();
        sum += num;
    }
    println!("{}", sum);
}
