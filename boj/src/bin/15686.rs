use std::{io, ops::Sub};

#[derive(Copy, Clone, Debug)]
struct Point(i32, i32);
#[derive(Copy, Clone, Debug)]
struct Chicken {
    pos: Point,
    is_alive: bool,
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Point {
    pub fn abs(self) -> i32 {
        self.0.abs() + self.1.abs()
    }
}

impl Chicken {
    pub fn new(i: i32, j: i32) -> Self {
        Self {
            pos: Point(i, j),
            is_alive: true,
        }
    }
}

/// sum of the least distances between houses and chickens which is alive
fn chicken_dist(houses: &[Point], chickens: &[Chicken]) -> i32 {
    let mut sum = 0;
    for &house in houses.iter() {
        let mut min = i32::MAX;
        for chicken in chickens.iter().filter(|e| e.is_alive).map(|e| e.pos) {
            min = min.min((house - chicken).abs());
        }
        sum += min;
    }
    sum
}

fn solution_recur(houses: &[Point], chickens: &mut [Chicken], m: i32, start_from: usize) -> i32 {
    if chickens.iter().filter(|e| e.is_alive).count() <= m as usize {
        return chicken_dist(houses, chickens);
    }
    let mut local_best = i32::MAX;

    for i in start_from..chickens.len() {
        chickens[i].is_alive = false;
        local_best = local_best.min(solution_recur(houses, chickens, m, i + 1));
        chickens[i].is_alive = true;
    }

    local_best
}

fn main() -> Result<(), io::Error> {
    let mut lines = io::stdin().lines().map(|res_str| {
        res_str.map(|s| {
            s.split(' ')
                .map(|i| i.parse::<i32>().expect("parse error"))
                .collect::<Vec<_>>()
        })
    });
    let spl = lines.next().unwrap()?;
    let (n, m) = (spl[0], spl[1]);
    let mut houses: Vec<Point> = vec![];
    let mut chickens: Vec<Chicken> = vec![];

    for i in 0..n {
        let spl = lines.next().unwrap()?;
        for (elem, j) in spl.iter().zip(0..) {
            match elem {
                1 => houses.push(Point(i, j)),
                2 => chickens.push(Chicken::new(i, j)),
                _ => continue,
            }
        }
    }

    let submit = solution_recur(&houses, &mut chickens, m, 0);

    println!("{}", submit);

    Ok(())
}
