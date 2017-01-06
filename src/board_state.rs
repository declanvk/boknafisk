use piece_board::PieceBoard;
use castling::{CastlingFlags, WHITE_QUEENSIDE, WHITE_KINGSIDE, BLACK_QUEENSIDE, BLACK_KINGSIDE};
use piece::{PieceType, Color};
use square_position::{SquarePosition,Direction, EAST};
use std::ops::Index;
use bit_boards::*;
use std::convert::{TryFrom};
use std::fmt;
use std::result::Result;

pub struct BoardState {
    bit_board: [[BitBoard; 6]; 2],
    bit_occupancy: [BitBoard; 2],
    en_passant: BitBoard,
    piece_board: PieceBoard,
    castling_rights: CastlingFlags,
    halfmove_clock: u32,
    fullmove_clock: u32,
    active_color: Color
}

#[derive(Debug)]
pub enum FromFenError {
    FakeError
}

impl<'a> TryFrom<&'a str> for BoardState {
    type Err = FromFenError;

    fn try_from(fen_string: &'a str) -> Result<BoardState, FromFenError> {
        Err(FromFenError::FakeError)
    }
}

impl fmt::Display for BoardState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result: String = String::new();

        //Piece placement
        for rank_index in (0..8).rev() {
            let mut position_index = SquarePosition {rank: rank_index, file: 0};

            let mut empty_count = 0;
            match self.piece_board[position_index] {
                Some(piece) => {
                    if empty_count > 0 {
                        result += &empty_count.to_string();
                    }

                    result += match piece.piece_type() {
                        PieceType::King => "K",
                        PieceType::Queen => "Q",
                        PieceType::Bishop => "B",
                        PieceType::Knight => "K",
                        PieceType::Rook => "R",
                        PieceType::Pawn => "P",
                    };

                    empty_count = 0;
                },
                None => empty_count += 1,
            }

            while let Some(new_position) = position_index + EAST {
                match self.piece_board[new_position] {
                    Some(piece) => {
                        if empty_count > 0 {
                            result += &empty_count.to_string();
                        }

                        result += match piece.piece_type() {
                            PieceType::King => "K",
                            PieceType::Queen => "Q",
                            PieceType::Bishop => "B",
                            PieceType::Knight => "K",
                            PieceType::Rook => "R",
                            PieceType::Pawn => "P",
                        };

                        empty_count = 0;
                    },
                    None => empty_count += 1,
                }

                position_index = new_position;
            }

            if empty_count > 0 {
                result += &empty_count.to_string();
            }

            if rank_index > 0 {
                result += "/"
            }
        }

        result += " ";

        //Active color
        match self.active_color {
            Color::White => {
                result += "w";
            },
            Color::Black => {
                result += "b";
            }
        }

        result += " ";

        //Castling availability
        if self.castling_rights.is_empty() {
            result += "-";
        } else {
            if self.castling_rights.intersects(WHITE_KINGSIDE) {
                result += "K";
            }

            if self.castling_rights.intersects(WHITE_QUEENSIDE) {
                result += "Q";
            }

            if self.castling_rights.intersects(BLACK_KINGSIDE) {
                result += "k";
            }

            if self.castling_rights.intersects(BLACK_QUEENSIDE) {
                result += "q";
            }
        }

        result += " ";

        //En passant position
        let en_passant_position =
            bit_scan_forward(self.en_passant)
                .map(|square_index| SquarePosition::try_from(square_index));
        if let Some(Ok(new_pos)) = en_passant_position {
            result += &new_pos.to_string();
        } else {
            result += "-"
        }

        result += " ";

        result += &self.halfmove_clock.to_string();

        result += " ";

        result += &self.fullmove_clock.to_string();

        write!(f, "{}", result)
    }
}

impl Index<(Color, PieceType)> for BoardState {
    type Output = BitBoard;

    fn index(&self, index: (Color, PieceType)) -> &Self::Output {
        &self.bit_board[index.0 as usize][index.1 as usize]
    }
}

impl Index<Color> for BoardState {
    type Output = BitBoard;

    fn index(&self, index: Color) -> &Self::Output {
        &self.bit_occupancy[index as usize]
    }
}