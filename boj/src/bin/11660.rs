use std::io::{self, *};

use sol::sum_between;

const MAX_N: usize = 1024;
type Arr = [[Value; MAX_N]; MAX_N];
type Value = i32;

static mut ARR: Arr = [[0; MAX_N]; MAX_N];

fn main() -> io::Result<()> {
    let mut lines = stdin().lines();
    let nm: Vec<isize> = lines
        .next()
        .unwrap()
        .unwrap()
        .split(' ')
        .map(|e| e.trim().parse().unwrap())
        .collect();

    let n = nm[0];
    let m = nm[1];

    for i in 0..n {
        let splitted: Vec<Value> = lines
            .next()
            .unwrap()
            .unwrap()
            .split(' ')
            .map(|e| e.trim().parse().unwrap())
            .collect();
        for (j, elem) in splitted.iter().enumerate() {
            sol::partial_sum(unsafe { &mut ARR }, i, j as isize, *elem);
        }
    }

    for _ in 0..m {
        let xy: Vec<_> = lines
            .next()
            .unwrap()
            .unwrap()
            .split(' ')
            .map(|e| e.trim().parse::<isize>().unwrap())
            .map(|e| e - 1) // problem counts from 1
            .collect();
        let submit = sum_between(unsafe { &ARR }, xy[0], xy[1], xy[2], xy[3]);

        println!("{}", submit);
    }

    Ok(())
}

mod sol {
    use super::{Arr, Value};

    /// ```
    /// sum[i][j] = sum[i-1][j] + sum[i][j-1] - sum[i-1][j-1]
    /// ```
    pub fn partial_sum(sum: &mut Arr, i: isize, j: isize, value: Value) {
        sum[i as usize][j as usize] = value;
        sum[i as usize][j as usize] += *sum.get2d(i - 1, j).unwrap_or(&0);
        sum[i as usize][j as usize] += *sum.get2d(i, j - 1).unwrap_or(&0);
        sum[i as usize][j as usize] -= *sum.get2d(i - 1, j - 1).unwrap_or(&0);
    }

    ///```
    ///sum[i2][j2] - sum[i2][j1-1] - sum[i1 - 1][j2] + sum[i1 - 1][j1 - 1]
    /// where i1 < i2 and j1 < j2
    ///```
    pub fn sum_between(sum: &Arr, i1: isize, j1: isize, i2: isize, j2: isize) -> Value {
        let (i1, i2) = if i1 > i2 { (i2, i1) } else { (i1, i2) };
        let (j1, j2) = if j1 > j2 { (j2, j1) } else { (j1, j2) };

        let mut ret = sum[i2 as usize][j2 as usize];
        ret -= sum.get2d(i2, j1 - 1).unwrap_or(&0);
        ret -= sum.get2d(i1 - 1, j2).unwrap_or(&0);
        ret += sum.get2d(i1 - 1, j1 - 1).unwrap_or(&0);

        ret
    }

    trait Get2D {
        type Target;
        fn get2d(&self, i: isize, j: isize) -> Option<&Self::Target>;
    }

    impl Get2D for Arr {
        type Target = Value;

        fn get2d(&self, i: isize, j: isize) -> Option<&Self::Target> {
            if i < 0 || j < 0 {
                return None;
            }
            match self.get(i as usize) {
                Some(row) => row.get(j as usize),
                None => None,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::sol::partial_sum;

    const MAX_M: isize = 100_000;
    const MAX_RNG: i32 = 1000;

    use super::*;
    use rand::{self, Rng};
    #[test]
    fn timeout() {
        let mut r = rand::thread_rng();
        for i in 0..MAX_N as isize {
            for j in 0..MAX_N as isize {
                partial_sum(unsafe { &mut ARR }, i, j, r.gen_range(1..=MAX_RNG));
            }
        }

        for _ in 0..MAX_M {
            let mut xy: [isize; 4] = [0; 4];
            for e in xy.iter_mut() {
                *e = r.gen_range(1..=MAX_RNG) as isize;
            }
            sum_between(unsafe { &ARR }, xy[0], xy[1], xy[2], xy[3]);
        }
    }
}
