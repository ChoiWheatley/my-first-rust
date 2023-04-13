use std::fmt::Debug;
use std::io;

type Id = i32;
type Index = i32;
struct Block {
    id: Id,
    row: Index,
    col: Index,
}

pub struct Database {
    entries: Vec<Block>,
    n: i32,
}

pub struct DatabaseBuilder {
    entries: Vec<Block>,
    n: i32,
}

trait Rotate {
    /// move `pos` block into `dst`
    /// return: number of rotations happened
    fn rotate(&mut self, pos: (Index, Index), dst: (Index, Index)) -> i32;
}

trait Query {
    /// query block from database and return `(row, col)`
    fn get_pos(&self, id: Id) -> Option<(Index, Index)>;
}

enum RowOrCol {
    Row(Index),
    Col(Index),
}

impl Database {
    /// rotate given line amount times,
    /// if line is `Row`, rotate this row right,
    /// if line is `Col`, rotate this col down.
    fn do_rotate(&mut self, line: RowOrCol, amount: i32) {
        match line {
            RowOrCol::Row(row) => {
                // rotate this row to the right
                self.entries
                    .iter_mut()
                    .filter(|e| e.row == row)
                    .for_each(|e| e.col = (e.col + amount) % self.n);
            }
            RowOrCol::Col(col) => {
                self.entries
                    .iter_mut()
                    .filter(|e| e.col == col)
                    .for_each(|e| e.row = (e.row + amount) % self.n);
            }
        }
    }
}

impl Debug for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:?}, {:?}, {:?})", self.id, self.row, self.col)
    }
}

impl Debug for Database {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Database")
            .field("entries", &self.entries)
            .finish()
    }
}

impl Rotate for Database {
    /// as problem says, rotate row to the right, then rotate col to the down
    /// so when dst < pos, you have to rotate one direction and jump back to destination
    fn rotate(&mut self, pos: (Index, Index), dst: (Index, Index)) -> i32 {
        // watch out deltas are actually difference between rows and cols
        let delta_y = (dst.0 + self.n - pos.0) % self.n;
        let delta_x = (dst.1 + self.n - pos.1) % self.n;
        // row first
        self.do_rotate(RowOrCol::Row(pos.0), delta_x);
        // col then
        self.do_rotate(RowOrCol::Col((pos.1 + delta_x) % self.n), delta_y);

        delta_y + delta_x
    }
}
impl Query for Database {
    fn get_pos(&self, id: Id) -> Option<(Index, Index)> {
        self.entries
            .iter()
            .find(|&block| block.id == id)
            .map(|block| (block.row, block.col))
    }
}

impl DatabaseBuilder {
    pub fn new(n: i32) -> Self {
        DatabaseBuilder { entries: vec![], n }
    }
    pub fn add_entry(&mut self, id: Id) {
        let block = Block {
            id,
            row: id / self.n,
            col: id % self.n,
        };
        self.entries.push(block);
    }
    pub fn build(self) -> Database {
        Database {
            entries: self.entries,
            n: self.n,
        }
    }
}

fn main() -> Result<(), io::Error> {
    let mut lines = io::stdin().lines().map(|resstr| {
        resstr.map(|s| {
            s.split(' ')
                .map(|s| s.parse::<i32>().expect("parse error!"))
                .collect::<Vec<_>>()
        })
    });

    let [n, k] = lines.next().unwrap()?[..] else {panic!("two numbers expected")};

    let mut builder = DatabaseBuilder::new(n);
    let mut pending = vec![];

    for _ in 0..k {
        let [id, row, col] = lines.next().unwrap()?[..] else {panic!("three numbers expected")};
        builder.add_entry(id - 1);
        pending.push((id as Id, row - 1 as Index, col - 1 as Index)); // because the problem starts index with 1
    }

    let mut db = builder.build();

    for (id, row, col) in pending {
        let submit = db.rotate(db.get_pos(id - 1).expect("id not found"), (row, col));
        println!("{submit}");
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn index() {
        const N: i32 = 4;
        let mut builder = DatabaseBuilder::new(N);
        for i in 0..N * N {
            builder.add_entry(i);
        }
        let db = builder.build();
        for i in 0..N * N {
            assert_eq!((i / N, i % N), db.get_pos(i).unwrap());
        }
    }
}
