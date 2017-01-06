use std::ops::{Mul, Add};
use std::fmt;
use std::str::FromStr;
use std::convert::TryFrom;

#[derive(Debug, Copy, Hash, Clone)]
pub struct SquarePosition {
    pub rank: usize,
    pub file: usize,
}

impl SquarePosition {
    pub fn new(rank: usize, file: usize) -> SquarePosition {
        SquarePosition {
            rank: rank,
            file: file,
        }
    }

    pub fn to_square_index(&self) -> usize {
        self.rank * 8 + self.file
    }
}

#[derive(Debug)]
pub enum SquarePositionFromError {
    OutOfBounds
}

impl TryFrom<i32> for SquarePosition {
    type Err = SquarePositionFromError;
    fn try_from(square_index: i32) -> Result<SquarePosition, SquarePositionFromError> {
        if 0 <= square_index && square_index <= 63 {
            let file = square_index % 8;
            let rank = square_index / 8;

            Ok(SquarePosition {
                rank: rank as usize,
                file: file as usize,
            })
        } else {
            Err(SquarePositionFromError::OutOfBounds)
        }
    }
}

impl TryFrom<usize> for SquarePosition {
    type Err = SquarePositionFromError;
    fn try_from(square_index: usize) -> Result<SquarePosition, SquarePositionFromError> {
        if square_index <= 63 {
            let file = square_index % 8;
            let rank = square_index / 8;

            Ok(SquarePosition {
                rank: rank,
                file: file,
            })
        } else {
            Err(SquarePositionFromError::OutOfBounds)
        }
    }
} 

#[derive(Debug)]
pub enum SquarePositionParseError {
    WrongInputLength,
    InvalidFileChar,
    InvalidRankChar,
}

impl FromStr for SquarePosition {
    type Err = SquarePositionParseError;

    fn from_str(s: &str) -> Result<SquarePosition, SquarePositionParseError> {
        let tokens = s.chars().collect::<Vec<char>>();
        if tokens.len() != 2 {
            return Err(SquarePositionParseError::WrongInputLength);
        } else {
            let file_char = tokens[0];
            let rank_char = tokens[1];

            let file = ((file_char as u8) - ('a' as u8)) as usize;
            if file < (0 as usize) || (7 as usize) < file {
                return Err(SquarePositionParseError::InvalidFileChar);
            }

            let rank = ((rank_char as u8) - ('1' as u8)) as usize;
            if rank < (0 as usize) || (7 as usize) < rank {
                return Err(SquarePositionParseError::InvalidRankChar);
            }

            Ok(SquarePosition::new(file, rank))
        }
    }
}

impl fmt::Display for SquarePosition {
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
            _ => "?",
        };

        write!(f, "{}{}", display_file, self.rank + 1)
    }
}

impl Add<Direction> for SquarePosition {
    type Output = Option<SquarePosition>;

    fn add(self, other: Direction) -> Option<SquarePosition> {
        let irank = self.rank as i32;
        let ifile = self.file as i32;

        let x88_mapping = 16 * (irank + other.0) + (ifile + other.1);

        if x88_mapping & 0x88 == 0 {
            let file = x88_mapping & 7;
            let rank = x88_mapping >> 4;

            Some(SquarePosition {
                rank: rank as usize,
                file: file as usize,
            })
        } else {
            None
        }
    }
}

#[derive(Debug, Copy, Hash, Clone)]
pub struct Direction(pub i32, pub i32);

pub const NORTH: Direction = Direction(1, 0);
pub const SOUTH: Direction = Direction(-1, 0);
pub const EAST: Direction = Direction(0, 1);
pub const WEST: Direction = Direction(0, -1);

pub const CARDINAL: [Direction; 4] = [NORTH, SOUTH, EAST, WEST];
pub const INTERMEDIATE: [Direction; 4] = [Direction(1,1), Direction(-1, 1), Direction(1, -1), Direction(-1, -1)];

impl Direction {
    pub fn new(rank: i32, file: i32) -> Direction {
        Direction(rank, file)
    }

    pub fn from_cardinal(north: i32, south: i32, east: i32, west: i32) -> Direction {
        NORTH * north + SOUTH * south + EAST * east + WEST * west
    }
}

impl Add for Direction {
    type Output = Direction;

    fn add(self, other: Direction) -> Direction {
        Direction(self.0 + other.0, self.1 + other.1)
    }
}

impl Mul for Direction {
    type Output = Direction;

    fn mul(self, other: Direction) -> Direction {
        Direction(self.0 * other.0, self.1 * other.1)
    }
}

impl Mul<i32> for Direction {
    type Output = Direction;

    fn mul(self, other: i32) -> Direction {
        Direction(self.0 * other, self.1 * other)
    }
}
