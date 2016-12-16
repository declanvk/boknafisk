use chess_core::piece::*;
use chess_core::position::*;
use std::ops::{Index, IndexMut};

#[derive(Copy)]
pub struct Board {
    board: [Option<Piece>; 64]
}

impl Board {
    pub fn empty_board() -> Board {
        Board {
            board: [None; 64]
        }
    }

    pub fn starting_board() -> Board {
        let mut starting = Board::empty_board();

        starting.board[0] = Some(Piece::new(PieceType::Rook, Color::White));;
        starting.board[1] = Some(Piece::new(PieceType::Knight, Color::White));;
        starting.board[2] = Some(Piece::new(PieceType::Bishop, Color::White));;
        starting.board[3] = Some(Piece::new(PieceType::Queen, Color::White));;
        starting.board[4] = Some(Piece::new(PieceType::King, Color::White));;
        starting.board[5] = Some(Piece::new(PieceType::Bishop, Color::White));;
        starting.board[6] = Some(Piece::new(PieceType::Knight, Color::White));;
        starting.board[7] = Some(Piece::new(PieceType::Rook, Color::White));;

        for white_pawn_index in 8..15 {
            starting.board[white_pawn_index] =  
                Some(
                    Piece::new(PieceType::Pawn, Color::White)
                );
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
            starting.board[black_pawn_index] =
                Some(
                    Piece::new(PieceType::Pawn, Color::Black)
                );
        }

        starting
    }

    pub fn to_board_iter(&self) -> BoardIterator {
        BoardIterator {
            board: &self,
            square_index: 0
        }
    }
}

impl Clone for Board {
    fn clone(&self) -> Board { *self }
}

impl Index<Position> for Board {
    type Output = Option<Piece>;

    fn index<'a>(&'a self, position: Position) -> &'a Option<Piece> {
        &self.board[8 * position.rank + position.file]
    }
}

impl IndexMut<Position> for Board {
    fn index_mut(&mut self, position: Position) -> &mut Option<Piece> {
        &mut self.board[8 * position.rank + position.file]
    }
}

pub struct BoardIterator<'a> {
    board: &'a Board,
    square_index: usize
}

impl<'a> Iterator for BoardIterator<'a> {
    type Item = (Position, Piece);

    fn next(&mut self) -> Option<(Position, Piece)> {
        let mut search_index = self.square_index;
        loop {
            if let Some(position) = Position::from_square_index(search_index) {
                if let Some(piece) = self.board[position] {
                    self.square_index = search_index + 1;
                    return Some((position, piece))
                }
            } else {
                self.square_index = search_index; 
                return None
            }

            search_index += 1;
        }
    }
}
