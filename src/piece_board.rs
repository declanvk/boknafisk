use piece::{Piece, PieceType, Color};
use square_position::SquarePosition;
use std::ops::{Index, IndexMut};
use std::convert::TryFrom;
use std::str::FromStr;
use std::fmt;
use error_types::FromStrError;

#[derive(Copy)]
pub struct PieceBoard {
    board: [Option<Piece>; 64],
}

impl PieceBoard {
    pub fn empty_board() -> PieceBoard {
        PieceBoard { board: [None; 64] }
    }

    pub fn starting_board() -> PieceBoard {
        let mut starting = PieceBoard::empty_board();

        starting.board[0] = Some(Piece::new(PieceType::Rook, Color::White));;
        starting.board[1] = Some(Piece::new(PieceType::Knight, Color::White));;
        starting.board[2] = Some(Piece::new(PieceType::Bishop, Color::White));;
        starting.board[3] = Some(Piece::new(PieceType::Queen, Color::White));;
        starting.board[4] = Some(Piece::new(PieceType::King, Color::White));;
        starting.board[5] = Some(Piece::new(PieceType::Bishop, Color::White));;
        starting.board[6] = Some(Piece::new(PieceType::Knight, Color::White));;
        starting.board[7] = Some(Piece::new(PieceType::Rook, Color::White));;

        for white_pawn_index in 8..16 {
            starting.board[white_pawn_index] = Some(Piece::new(PieceType::Pawn, Color::White));
        }

        starting.board[56] = Some(Piece::new(PieceType::Rook, Color::Black));;
        starting.board[57] = Some(Piece::new(PieceType::Knight, Color::Black));;
        starting.board[58] = Some(Piece::new(PieceType::Bishop, Color::Black));;
        starting.board[59] = Some(Piece::new(PieceType::Queen, Color::Black));;
        starting.board[60] = Some(Piece::new(PieceType::King, Color::Black));;
        starting.board[61] = Some(Piece::new(PieceType::Bishop, Color::Black));;
        starting.board[62] = Some(Piece::new(PieceType::Knight, Color::Black));;
        starting.board[63] = Some(Piece::new(PieceType::Rook, Color::Black));;

        for black_pawn_index in 48..56 {
            starting.board[black_pawn_index] = Some(Piece::new(PieceType::Pawn, Color::Black));
        }

        starting
    }
}

impl FromStr for PieceBoard {
    type Err = FromStrError;
    fn from_str(piece_placement_string: &str) -> Result<PieceBoard, FromStrError> {
        let mut piece_board = PieceBoard::empty_board();
        let piece_placement_components = piece_placement_string.split("/").collect::<Vec<&str>>();

        if piece_placement_components.len() != 8 {
            return Err(FromStrError::InvalidInputLength("piece board", 8, piece_placement_components.len()))
        } else {
            for (rank_index, rank_string) in (0..8).rev().zip(piece_placement_components.into_iter()) {
                let mut file_index: usize = 0;
                for rank_char in rank_string.chars() {
                    match rank_char {
                        x if x.is_digit(10) => {
                            file_index += x.to_digit(10).unwrap() as usize;
                        },
                        x => {
                            let piece_to_insert: Piece = (&x.to_string()).parse()?;
                            let board_index = file_index + rank_index * 8;
                            piece_board.board[board_index] = Some(piece_to_insert);

                            println!("Inserted: {} at {} based on {}", piece_to_insert, board_index, x);

                            file_index += 1;
                        }
                    }
                }
            }

            Ok(piece_board)
        }
    }
}

impl fmt::Display for PieceBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();

        for rank_index in (0..8).rev() {
            let mut empty_count = 0;

            for file_index in 0..8 {
                let board_index = rank_index * 8 + file_index;

                match self.board[board_index] {
                    Some(piece) => {
                        if empty_count > 0 {
                            result += &empty_count.to_string();
                        }

                        result += &piece.to_string();

                        empty_count = 0;
                    },
                    None => empty_count += 1,
                }
            }

            if empty_count > 0 {
                result += &empty_count.to_string();
            }

            if rank_index > 0 {
                result += "/"
            }
        }

        write!(f, "{}", result)
    }
}

impl<'a> IntoIterator for &'a PieceBoard {
    type Item = (SquarePosition, Piece);
    type IntoIter = PieceBoardIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        PieceBoardIterator {
            board: self,
            square_index: 0,
        }
    }
}

impl Clone for PieceBoard {
    fn clone(&self) -> PieceBoard {
        *self
    }
}

impl Index<SquarePosition> for PieceBoard {
    type Output = Option<Piece>;

    fn index<'a>(&'a self, position: SquarePosition) -> &'a Option<Piece> {
        &self.board[8 * position.rank + position.file]
    }
}

impl IndexMut<SquarePosition> for PieceBoard {
    fn index_mut(&mut self, position: SquarePosition) -> &mut Option<Piece> {
        &mut self.board[8 * position.rank + position.file]
    }
}

pub struct PieceBoardIterator<'a> {
    board: &'a PieceBoard,
    square_index: usize,
}

impl<'a> Iterator for PieceBoardIterator<'a> {
    type Item = (SquarePosition, Piece);

    fn next(&mut self) -> Option<(SquarePosition, Piece)> {
        let mut search_index = self.square_index;
        loop {
            if let Ok(position) = SquarePosition::try_from(search_index) {
                if let Some(piece) = self.board[position] {
                    self.square_index = search_index + 1;
                    return Some((position, piece));
                }
            } else {
                self.square_index = search_index;
                return None;
            }

            search_index += 1;
        }
    }
}

#[cfg(test)]
mod test {
    use piece_board::PieceBoard;

    #[test]
    fn starting_board_to_str_test() {
        let starting_board = PieceBoard::starting_board();

        assert_eq!(starting_board.to_string(), "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR")
    }

    #[test]
    fn starting_board_from_str_test() {
        let starting_board_str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";
        let starting_board_from_str: PieceBoard = starting_board_str.parse().unwrap();

        assert_eq!(starting_board_from_str.to_string(), PieceBoard::starting_board().to_string());
    }
}