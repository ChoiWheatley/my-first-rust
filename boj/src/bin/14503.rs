/// 시뮬구현 만큼은 러스트로 풀어보고 싶어!
///
use std::{
    io::stdin,
    ops::{Add, Neg},
};

const MAX_SIDE: usize = 50;
const DIRS: isize = 4;

#[derive(Copy, Clone, Debug)]
pub struct Position(isize, isize);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    North = 0,
    East = 1,
    South = 2,
    West = 3,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum RoomState {
    Wall,
    Dirty,
    Clean,
}

struct Robot {
    pos: Position,
    dir: Direction,
}

pub struct Room {
    robot: Robot,
    map: [[RoomState; MAX_SIDE]; MAX_SIDE],
}

#[derive(Debug)]
pub struct RoomBuilder {
    robot_pos: Position,
    robot_dir: Direction,
    walls: Vec<Position>,
    dirts: Vec<Position>,
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

impl Add for Position {
    type Output = Position;

    fn add(self, rhs: Self) -> Self::Output {
        Position(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Add<Direction> for Position {
    type Output = Position;

    fn add(self, rhs: Direction) -> Self::Output {
        match rhs {
            Direction::North => Position(self.0 - 1, self.1),
            Direction::East => Position(self.0, self.1 + 1),
            Direction::South => Position(self.0 + 1, self.1),
            Direction::West => Position(self.0, self.1 - 1),
        }
    }
}

impl From<isize> for Direction {
    fn from(value: isize) -> Self {
        match value {
            0 => Direction::North,
            1 => Direction::East,
            2 => Direction::South,
            3 => Direction::West,
            other => {
                let tmp = other % DIRS;
                Self::from(if tmp < 0 { tmp + DIRS } else { tmp })
            }
        }
    }
}

impl Neg for Direction {
    type Output = Direction;

    fn neg(self) -> Self::Output {
        ((self as isize + 2) % DIRS).into()
    }
}

impl Direction {
    pub fn of_ccw(self) -> Self {
        let tmp = (self as isize) - 1;
        (if tmp < 0 { tmp + DIRS } else { tmp }).into()
    }
    pub fn of_backward(self) -> Self {
        -self
    }
}

impl RoomBuilder {
    pub fn new() -> Self {
        Self {
            robot_pos: Position(0, 0),
            robot_dir: 0.into(),
            walls: vec![],
            dirts: vec![],
        }
    }
    pub fn set_robot_pos(&mut self, pos: Position) -> &mut Self {
        self.robot_pos = pos;
        self
    }
    pub fn set_robot_dir(&mut self, dir: Direction) -> &mut Self {
        self.robot_dir = dir;
        self
    }
    pub fn add_wall(&mut self, pos: Position) -> &mut Self {
        self.walls.push(pos);
        self
    }
    pub fn add_dirt(&mut self, pos: Position) -> &mut Self {
        self.dirts.push(pos);
        self
    }
    pub fn build(self) -> Room {
        let mut map = [[RoomState::Wall; MAX_SIDE]; MAX_SIDE];
        for e in self.walls {
            map[e.0 as usize][e.1 as usize] = RoomState::Wall;
        }
        for e in self.dirts {
            map[e.0 as usize][e.1 as usize] = RoomState::Dirty;
        }
        Room {
            robot: Robot {
                pos: self.robot_pos,
                dir: self.robot_dir,
            },
            map,
        }
    }
}

impl Robot {
    pub fn upfront(&self) -> Position {
        self.pos + self.dir
    }
    pub fn of_behind(&self) -> Position {
        self.pos + (-self.dir)
    }
    pub fn do_move_forward(&mut self) {
        self.pos = self.upfront();
    }
    pub fn do_move_backward(&mut self) {
        self.pos = self.of_behind();
    }
    pub fn do_ccw(&mut self) {
        self.dir = self.dir.of_ccw();
    }
}

impl Room {
    pub fn run_robot<T>(&mut self, mut on_clean: T)
    where
        T: FnMut(Position),
    {
        'MAIN: loop {
            match self.try_get_map(self.robot.pos) {
                Ok(dirty_cell @ RoomState::Dirty) => {
                    // DO clean this floor
                    *dirty_cell = RoomState::Clean;
                    on_clean(self.robot.pos);
                }
                Ok(RoomState::Clean) => {} // do nothing
                _ => break,                // wall or out of bounds
            }
            // find dirty floors adjacent to it.
            for _ in 0..DIRS {
                self.robot.do_ccw();
                if let Ok(RoomState::Dirty) = self.try_get_map(self.robot.upfront()) {
                    // Wow, we find a new place to clean!!! Let's DO move on to the next phase!
                    self.robot.do_move_forward();
                    continue 'MAIN;
                }
            }
            // there were no dirty floors near to robot!!!
            // the robot DO tries to move backward
            self.robot.do_move_backward();
        }
    }

    fn is_inside(pos: Position) -> bool {
        pos.0 < 0 || pos.1 < 0 || MAX_SIDE <= pos.0 as usize || MAX_SIDE <= pos.1 as usize
    }

    fn try_get_map(&mut self, pos: Position) -> Result<&mut RoomState, ()> {
        if Self::is_inside(pos) {
            return Err(());
        }
        Ok(&mut self.map[pos.0 as usize][pos.1 as usize])
    }
}

fn main() {
    let mut builder = RoomBuilder::new();
    let mut lines = stdin().lines();
    let splitted: Vec<isize> = lines
        .next()
        .unwrap()
        .unwrap()
        .split(' ')
        .map(|each| each.parse().expect("parse error"))
        .collect();
    let (n, _m) = (splitted[0], splitted[1]);
    let splitted: Vec<isize> = lines
        .next()
        .unwrap()
        .unwrap()
        .split(' ')
        .map(|each| each.parse().expect("parse error"))
        .collect();

    let (robot_y, robot_x, dir) = (splitted[0], splitted[1], splitted[2]);
    builder.set_robot_pos(Position(robot_y, robot_x));
    builder.set_robot_dir(dir.into());

    for i in 0..n {
        let splitted: Vec<usize> = lines
            .next()
            .unwrap()
            .unwrap()
            .split(' ')
            .map(|each| each.parse().expect("parse error"))
            .collect();
        for (j, &elem) in splitted.iter().enumerate() {
            match elem {
                0 => builder.add_dirt(Position(i, j as isize)),
                _ => builder.add_wall(Position(i, j as isize)),
            };
        }
    }

    let mut room = builder.build();

    let mut count = 0;
    room.run_robot(|_pos| {
        count += 1;
    });

    println!("{}", count);
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn ccw() {
        let d: Direction = 0.into();
        assert_eq!(d, Direction::North);
        assert_eq!(d.of_ccw(), Direction::West);
        assert_eq!(d.of_ccw().of_ccw(), Direction::South);
        assert_eq!(d.of_ccw().of_ccw().of_ccw(), Direction::East);
    }

    #[test]
    fn backward() {
        assert_eq!(-Direction::North, Direction::South);
        assert_eq!(-Direction::South, Direction::North);
        assert_eq!(-Direction::East, Direction::West);
        assert_eq!(-Direction::West, Direction::East);
    }

    #[test]
    fn into_direction() {
        assert_eq!(Direction::from(-4), Direction::North);
        assert_eq!(Direction::from(-3), Direction::East);
        assert_eq!(Direction::from(-2), Direction::South);
        assert_eq!(Direction::from(-1), Direction::West);
        assert_eq!(Direction::from(0), Direction::North);
        assert_eq!(Direction::from(1), Direction::East);
        assert_eq!(Direction::from(2), Direction::South);
        assert_eq!(Direction::from(3), Direction::West);
        assert_eq!(Direction::from(4), Direction::North);
    }
}
