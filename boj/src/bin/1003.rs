use std::io;

const MAX: usize = 40;
static mut DP: [[i32; 2]; MAX + 1] = [[0, 0]; MAX + 1];

/// fib(0)과 fib(1)이 호출된 횟수를 리턴
fn solution_recur(n: usize) -> &'static [i32; 2] {
    match n {
        0 => &[1, 0],
        1 => &[0, 1],
        other if other <= MAX => {
            if unsafe { DP }[other] == [0, 0] {
                let twice_before = solution_recur(n - 2);
                let first_before = solution_recur(n - 1);
                unsafe {
                    DP[other][0] = twice_before[0] + first_before[0];
                    DP[other][1] = twice_before[1] + first_before[1];
                }
            }
            unsafe { &DP[other] }
        }
        _ => panic!("bound error"),
    }
}

fn main() {
    let mut lines = io::stdin()
        .lines()
        .flat_map(|res_str| res_str.map(|s| s.parse::<usize>().expect("parse error")));
    let t = lines.next().unwrap();
    for _ in 0..t {
        let n = lines.next().unwrap();
        let submit = solution_recur(n);
        println!("{} {}", submit[0], submit[1]);
    }
}
