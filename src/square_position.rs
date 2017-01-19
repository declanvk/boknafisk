use std::ops::{Add, Mul};
use std::fmt;
use std::str::FromStr;
use std::convert::TryFrom;
use error_types::FromStrError;
use bit_boards::BitBoard;

#[derive(Debug, Copy, Hash, Clone, PartialEq)]
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

    pub fn to_bit_board(&self) -> BitBoard {
        1 << self.to_square_index()
    }
}

#[derive(Debug)]
pub enum SquarePositionFromError {
    OutOfBounds,
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

impl FromStr for SquarePosition {
    type Err = FromStrError;

    fn from_str(s: &str) -> Result<SquarePosition, Self::Err> {
        let tokens = s.chars().collect::<Vec<char>>();
        if tokens.len() != 2 {
            Err(FromStrError::InvalidInputLength("square position", 2, tokens.len()))
        } else {
            let file_char = tokens[0];
            let rank_char = tokens[1];

            let file = ((file_char as u8) - ('a' as u8)) as usize;
            if file < (0 as usize) || (7 as usize) < file {
                return Err(FromStrError::MalformedInput("square position"));
            }

            let rank = ((rank_char as u8) - ('1' as u8)) as usize;
            if rank < (0 as usize) || (7 as usize) < rank {
                return Err(FromStrError::MalformedInput("square position"));
            }

            Ok(SquarePosition::new(rank, file))
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
pub const INTERMEDIATE: [Direction; 4] =
    [Direction(1, 1), Direction(-1, 1), Direction(1, -1), Direction(-1, -1)];

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

#[cfg(test)]
mod test {
    use square_position::SquarePosition;
    use std::convert::TryFrom;

    fn all_squares_str() -> Vec<&'static str> {
        vec!["a1", "b1", "c1", "d1", "e1", "f1", "g1", "h1", "a2", "b2", "c2", "d2", "e2", "f2",
             "g2", "h2", "a3", "b3", "c3", "d3", "e3", "f3", "g3", "h3", "a4", "b4", "c4", "d4",
             "e4", "f4", "g4", "h4", "a5", "b5", "c5", "d5", "e5", "f5", "g5", "h5", "a6", "b6",
             "c6", "d6", "e6", "f6", "g6", "h6", "a7", "b7", "c7", "d7", "e7", "f7", "g7", "h7",
             "a8", "b8", "c8", "d8", "e8", "f8", "g8", "h8"]
    }

    fn all_squares_indices() -> Vec<usize> {
        vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
             24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44,
             45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63]
    }

    fn all_squares_positions() -> Vec<SquarePosition> {
        let mut positions = vec![];
        for rank in 0..8 {
            for file in 0..8 {
                positions.push(SquarePosition::new(rank, file));
            }
        }

        positions
    }

    #[test]
    fn positions_indices_equivalence_test() {
        for (&position, &index) in all_squares_positions()
            .iter()
            .zip(all_squares_indices().iter()) {
            assert_eq!(position.to_square_index(), index);
            assert_eq!(position, SquarePosition::try_from(index).unwrap());
        }
    }

    #[test]
    fn positions_str_equivalence_test() {
        for (&position, &position_str) in all_squares_positions()
            .iter()
            .zip(all_squares_str().iter()) {
            assert_eq!(position.to_string(), position_str);
            assert_eq!(position, position_str.parse::<SquarePosition>().unwrap());
        }
    }
}
