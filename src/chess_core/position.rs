use std::ops::{Mul, Add};
use std::fmt;

#[derive(Debug, Copy, Hash, Clone)]
pub struct Position {
    pub rank: usize,
    pub file: usize
}

impl Position {
    pub fn new(rank:usize, file:usize) -> Position {
        Position {
            rank: rank,
            file: file
        }
    }

    pub fn from_square_index(square_index:usize) -> Option<Position> {
        if 0 as usize <= square_index && square_index <= 63 as usize {
            let file = square_index % 8;
            let rank = square_index / 8;

            Some(Position {
                rank: rank,
                file: file
            })
        } else {
            None
        }
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let display_file = match self.file {
            0 => "a",
            1 => "b",
            2 => "c",
            3 => "d",
            4 => "e",
            5 => "f",
            6 => "g",
            7 => "h",
            _ => "?"
        };
        
        write!(f, "{}{}", display_file, self.rank + 1) 
    }
}

impl Add<Direction> for Position {
    type Output = Option<Position>;

    fn add(self, other:Direction) -> Option<Position> {
        let irank = self.rank as i32;
        let ifile = self.file as i32;

        let square_mapping = (irank + other.0) * 8 + (ifile + other.1);

        if square_mapping >= 0 && square_mapping <= 63 {
            Some(Position {
                rank: (irank + other.0) as usize,
                file: (ifile + other.1) as usize
            })
        } else {
            None
        }
    }
}

#[derive(Debug, Copy, Hash, Clone)]
pub struct Direction(i32, i32);

pub const NORTH:Direction = Direction(1, 0);
pub const SOUTH:Direction = Direction(-1, 0);
pub const EAST:Direction = Direction(0, 1);
pub const WEST:Direction = Direction(0, -1);

pub const ROSE:[Direction; 4] = [NORTH, SOUTH, EAST, WEST];

impl Direction {
    pub fn new(rank:i32, file:i32) -> Direction {
        Direction(rank, file)
    }

    pub fn from_cardinal(north:i32, south:i32, east:i32, west:i32) -> Direction {
        NORTH * north + SOUTH * south + EAST * east + WEST * west
    }
}

impl Add for Direction {
    type Output = Direction;

    fn add(self, other:Direction) -> Direction {
        Direction(self.0 + other.0, self.1 + other.1)
    }
}

impl Mul for Direction {
    type Output = Direction;

    fn mul(self, other:Direction) -> Direction {
        Direction(self.0 * other.0, self.1 * other.1)
    }
}

impl Mul<i32> for Direction {
    type Output = Direction;

    fn mul(self, other:i32) -> Direction {
        Direction(self.0 * other, self.1 * other)
    }
}
