use std::io;
use std::io::Write;

const MAX_V: usize = 10_000;
const CAPACITY: usize = 4096000;
type ElemType = u32;

fn main() -> Result<(), io::Error> {
    let mut lines = io::stdin()
        .lines()
        .flat_map(|res_str| res_str.map(|s| s.parse::<usize>().expect("parse error")));
    let _n = lines.next().unwrap();

    let mut num_list = [0 as ElemType; MAX_V + 1];

    while let Some(num) = lines.next() {
        num_list[num] += 1;
    }

    let mut output = io::BufWriter::with_capacity(CAPACITY, io::stdout());

    for (i, el) in num_list.into_iter().enumerate().filter(|(_, el)| *el > 0) {
        for _ in 0..el {
            write!(output, "{}\n", i)?;
        }
    }

    Ok(())
}
