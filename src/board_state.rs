use piece_board::PieceBoard;
use castling::CastlingFlags;
use piece::{Color, PieceType};
use square_position::SquarePosition;
use std::ops::Index;
use bit_boards::*;
use std::convert::TryFrom;
use std::fmt;
use std::result::Result;
use std::str::FromStr;
use error_types::FromFenError;

pub struct BoardState {
    bit_board: [[BitBoard; 6]; 2],
    bit_occupancy: [BitBoard; 2],
    en_passant: BitBoard,
    piece_board: PieceBoard,
    castling_rights: CastlingFlags,
    halfmove_clock: u32,
    fullmove_clock: u32,
    active_color: Color,
}

impl FromStr for BoardState {
    type Err = FromFenError;

    fn from_str(fen_string: &str) -> Result<BoardState, FromFenError> {
        let components = fen_string.split_whitespace().collect::<Vec<&str>>();
        if components.len() == 6 {
            let piece_board: PieceBoard = components[0].parse()?;

            let (bit_board, bit_occupancy): ([[BitBoard; 6]; 2], [BitBoard; 2]) =
                From::from(piece_board);

            let active_color: Color = components[1].parse()?;

            let castling_rights: CastlingFlags = components[2].parse()?;

            let en_passant_position: BitBoard = match components[3].parse::<SquarePosition>() {
                Ok(position) => BitBoard::from(position),
                Err(_) => 0,
            };

            let halfmove_clock: u32 = components[4].parse()?;

            let fullmove_clock: u32 = components[5].parse()?;

            Ok(BoardState {
                bit_board: bit_board,
                bit_occupancy: bit_occupancy,
                en_passant: en_passant_position,
                piece_board: piece_board,
                castling_rights: castling_rights,
                halfmove_clock: halfmove_clock,
                fullmove_clock: fullmove_clock,
                active_color: active_color,
            })
        } else {
            Err(FromFenError::IncorrectNumberOfFields(components.len()))
        }
    }
}

impl fmt::Display for BoardState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let en_passant_position =
            bit_scan_forward(self.en_passant).map_or("-".to_owned(), |square_index| {
                SquarePosition::try_from(square_index).unwrap().to_string()
            });

        write!(f,
               "{} {} {} {} {} {}",
               self.piece_board,
               self.active_color,
               self.castling_rights,
               en_passant_position,
               self.halfmove_clock,
               self.fullmove_clock)
    }
}

impl Index<(Color, PieceType)> for BoardState {
    type Output = BitBoard;

    #[inline]
    fn index(&self, index: (Color, PieceType)) -> &Self::Output {
        &self.bit_board[index.0 as usize][index.1 as usize]
    }
}

impl Index<Color> for BoardState {
    type Output = BitBoard;

    #[inline]
    fn index(&self, index: Color) -> &Self::Output {
        &self.bit_occupancy[index as usize]
    }
}

#[cfg(test)]
mod test {
    use bit_boards::BitBoard;
    use board_state::BoardState;
    use piece_board::PieceBoard;
    use std::convert::From;
    use castling::CastlingFlags;
    use piece::Color;

    fn starting_board_state() -> BoardState {
        let starting_piece_board = PieceBoard::starting_board();
        let (bit_board, bit_occupancy) = From::from(starting_piece_board);

        BoardState {
            bit_board: bit_board,
            bit_occupancy: bit_occupancy,
            piece_board: starting_piece_board,
            en_passant: 0 as BitBoard,
            castling_rights: CastlingFlags::all(),
            halfmove_clock: 0,
            fullmove_clock: 1,
            active_color: Color::White,
        }
    }

    #[test]
    fn starting_to_string_test() {
        assert_eq!("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
                   starting_board_state().to_string());
    }

    #[test]
    fn starting_from_string_test() {
        assert_eq!(starting_board_state().to_string(),
                   "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
                       .parse::<BoardState>()
                       .unwrap()
                       .to_string());
    }
}
