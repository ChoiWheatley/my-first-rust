use std::error::Error;
use std::io::{stdin, stdout, BufRead, Write};

const INF: isize = isize::MAX;
const MAX_N: usize = 100;

static mut ARR: Arr = [[INF; MAX_N + 1]; MAX_N + 1];
type Arr = [[isize; MAX_N + 1]; MAX_N + 1];

fn init_arr(arr: &'static mut Arr) {
    arr.iter_mut().for_each(|line| {
        line.fill(INF);
    });
    for i in 1..=MAX_N {
        arr[i][i] = 0;
    }
}

fn solution(arr: &'static mut Arr, n: usize) {
    for k in 1..=n {
        for i in 1..=n {
            for j in 1..=n {
                arr[i][j] = arr[i][j].min(arr[i][k].checked_add(arr[k][j]).unwrap_or(INF));
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    init_arr(unsafe { &mut ARR });
    let mut lines = stdin().lock().lines().map(|result_line| {
        result_line.map(|line| {
            line.split(' ')
                .map(|word| word.parse::<isize>().expect("parse error!"))
                .collect::<Vec<_>>()
        })
    });

    let n = lines.next().unwrap()?[0] as usize;
    let m = lines.next().unwrap()?[0];

    // input

    for _ in 0..m {
        let sp = lines.next().unwrap()?;
        let (i, j, weight) = (sp[0] as usize, sp[1] as usize, sp[2]);
        unsafe {
            ARR[i][j] = ARR[i][j].min(weight);
        }
    }

    // solve

    solution(unsafe { &mut ARR }, n);

    for i in 1..=n {
        for j in 1..=n {
            stdout().write_fmt(format_args!("{} ", unsafe {
                if ARR[i][j] == INF {
                    0
                } else {
                    ARR[i][j]
                }
            }))?;
        }
        println!("");
    }

    Ok(())
}
