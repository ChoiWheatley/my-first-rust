use std::io::stdin;

const MAX_N: usize = 1000;
const MODULO: u32 = 1_000_000_009;
type ArrT = [[Option<u32>; MAX_N + 1]; MAX_N + 1];

static mut DP: ArrT = [[None; MAX_N + 1]; MAX_N + 1];

fn get(arr: &mut ArrT, n: i32, m: i32) -> u32 {
    match arr[n as usize][m as usize] {
        None => get_recur(arr, n, m),
        Some(x) => x,
    }
}
fn get_recur(arr: &mut ArrT, n: i32, m: i32) -> u32 {
    if n < m || n <= 0 || m <= 0 {
        return 0;
    }
    if let Some(ret) = arr[n as usize][m as usize] {
        return ret;
    }
    let a = get_recur(arr, n - 1, m - 1);
    let b = get_recur(arr, n - 2, m - 1);
    let c = get_recur(arr, n - 3, m - 1);

    let ret = (a + b + c) % MODULO;
    arr[n as usize][m as usize] = Some(ret);
    ret
}

fn main() {
    unsafe {
        DP[1][1] = Some(1);
        DP[2][1] = Some(1);
        DP[3][1] = Some(1);
    }
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();

    stdin().lines().for_each(|each_line| {
        let input: Vec<i32> = each_line
            .unwrap()
            .split(' ')
            .map(|e| e.parse().unwrap())
            .collect();
        let submit = get(unsafe { &mut DP }, input[0], input[1]);
        println!("{}", submit);
    });
}
