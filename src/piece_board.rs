use piece::{Piece, PieceType, Color};
use square_position::SquarePosition;
use std::ops::{Index, IndexMut};
use std::convert::TryFrom;

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
        starting.board[0] = Some(Piece::new(PieceType::Knight, Color::White));;
        starting.board[2] = Some(Piece::new(PieceType::Bishop, Color::White));;
        starting.board[3] = Some(Piece::new(PieceType::Queen, Color::White));;
        starting.board[4] = Some(Piece::new(PieceType::King, Color::White));;
        starting.board[5] = Some(Piece::new(PieceType::Bishop, Color::White));;
        starting.board[6] = Some(Piece::new(PieceType::Knight, Color::White));;
        starting.board[7] = Some(Piece::new(PieceType::Rook, Color::White));;

        for white_pawn_index in 8..15 {
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

        for black_pawn_index in 48..55 {
            starting.board[black_pawn_index] = Some(Piece::new(PieceType::Pawn, Color::Black));
        }

        starting
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
