use core::cmp::Ordering;
use core::mem::size_of;
use std::{collections::VecDeque, io};

const MAX_DAY: usize = 1_000_000;
const BIT_CNT: usize = size_of::<usize>() * 8;
const BIN_CNT: usize = MAX_DAY / BIT_CNT;
const fn bin_no(idx: usize) -> usize {
    idx / BIT_CNT
}
const fn bit_no(idx: usize) -> usize {
    idx % BIT_CNT
}

struct Bitset {
    bits: [usize; BIN_CNT + 1],
}

impl Bitset {
    pub fn new() -> Self {
        Bitset {
            bits: [0; BIN_CNT + 1],
        }
    }

    pub fn test(&self, idx: usize) -> bool {
        self.bits[bin_no(idx)] >> bit_no(idx) & 1 == 1
    }

    pub fn set(&mut self, idx: usize) {
        self.bits[bin_no(idx)] |= 1 << bit_no(idx)
    }

    pub fn reset(&mut self, idx: usize) {
        self.bits[bin_no(idx)] &= !(1 << bit_no(idx))
    }
}

fn solution(m: i32, mut schedule: VecDeque<i32>, bitset: Bitset) -> bool {
    let last_day = schedule
        .back()
        .cloned()
        .expect("schedule must have at least one element");

    // coffee를 끓이면 bowl을 하나 소비하는 식임.
    let mut bowl_cnt = 0;
    let mut coffee_cnt = 0;

    for day in 0..=last_day {
        let delta_day = schedule.front().cloned().unwrap_or(1 << 30) - day;

        match (
            bitset.test(day as usize), // Does customer come???
            delta_day.cmp(&m),         // It it okay to brew a coffee?
            bowl_cnt > 0,
            coffee_cnt > 0,
        ) {
            (true, _, _, true) => {
                // 사람이 와 있고 커피가 준비돼 있으면 커피를 제공한다.
                coffee_cnt -= 1;
            }
            (true, _, _, false) => {
                // 사람이 와 있는데 커피가 준비되어있지 않으면 장사를 말아먹는것임.
                return false;
            }
            (false, Ordering::Greater, ..) => {
                // 사람은 아직 안 왔는데 커피를 너무 일찍 끓이면 안되는 경우엔 그릇을 빚는다.
                bowl_cnt += 1;
            }
            (false, Ordering::Less | Ordering::Equal, false, _) => {
                // 사람은 아직 안 왔고 커피를 끓여도 될 타임인데 그릇이 없으면 그릇을 만들어야지
                bowl_cnt += 1;
            }
            (false, Ordering::Less | Ordering::Equal, true, _) => {
                // 사람은 아직 안 왔고 커피를 끓여도 될 타임이고 그릇까지 있다? 커피 안 끓이고 뭐해
                coffee_cnt += 1;
                bowl_cnt -= 1;
                schedule.pop_front();
            }
        }
    }

    true
}

fn main() -> Result<(), io::Error> {
    let mut lines = io::stdin().lines().map(|resstr| {
        resstr.map(|s| {
            s.trim()
                .split(' ')
                .map(|s| s.parse::<i32>().expect("parse error"))
                .collect::<Vec<_>>()
        })
    });

    let [_n, m] = lines.next().unwrap()?[..] else {panic!("two numbers expected")};

    let schedule: VecDeque<i32>;
    let mut bitset = Bitset::new();

    schedule = VecDeque::from(lines.next().unwrap()?);
    for day in schedule.iter().cloned() {
        bitset.set(day as usize);
    }

    println!(
        "{}",
        if solution(m, schedule, bitset) {
            "success"
        } else {
            "fail"
        }
    );

    Ok(())
}
