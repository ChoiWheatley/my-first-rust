use std::{
    cmp::{
        self,
        Ordering::{Equal, Greater, Less},
    },
    io::{self, BufRead},
};

fn main() -> std::io::Result<()> {
    let mut line = String::new();
    let a: i32;
    let b: i32;
    let n: i32;

    io::stdin().read_line(&mut line)?;

    let mut iter = line.split_whitespace();
    a = iter.next().unwrap().parse::<i32>().unwrap();
    b = iter.next().unwrap().parse::<i32>().unwrap();

    line.clear();
    io::stdin().read_line(&mut line)?;
    // println!("line: {:?}", line);
    n = line.trim().parse::<i32>().unwrap();

    let submit = solution(Box::new(io::stdin().lock()), a, b, n);

    println!("{}", submit);

    Ok(())
}

fn dist(lhs: i32, rhs: i32) -> i32 {
    if lhs < rhs {
        rhs - lhs
    } else {
        lhs - rhs
    }
}

fn solution<'a>(mut reader: Box<dyn BufRead>, a: i32, b: i32, n: i32) -> i32 {
    let mut line = String::new();
    let mut min_dist = i32::MAX;
    for _ in 0..n {
        line.clear();
        reader.read_line(&mut line).expect("read err");
        let each_button = line.trim().parse::<i32>().unwrap();
        min_dist = cmp::min(min_dist, dist(each_button, b));
    }
    match dist(a, b).cmp(&min_dist) {
        Less | Equal => dist(a, b),
        Greater => min_dist + 1,
    }
}
