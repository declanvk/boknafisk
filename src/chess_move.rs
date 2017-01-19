use piece::{Piece, PromotionType};
use square_position::SquarePosition;
use castling::CastleType;

pub struct Move {
    start_position: SquarePosition,
    end_position: SquarePosition,
    active_piece: Piece,
    move_type: MoveType,
}

impl Move {
    #[inline]
    pub fn new(start: SquarePosition,
               end: SquarePosition,
               active: Piece,
               move_type: MoveType)
               -> Move {
        Move {
            start_position: start,
            end_position: end,
            active_piece: active,
            move_type: move_type,
        }
    }

    #[inline]
    pub fn start(&self) -> &SquarePosition {
        &self.start_position
    }

    #[inline]
    pub fn end(&self) -> &SquarePosition {
        &self.end_position
    }

    #[inline]
    pub fn active(&self) -> &Piece {
        &self.active_piece
    }

    #[inline]
    pub fn move_type(&self) -> &MoveType {
        &self.move_type
    }
}

pub enum MoveType {
    Quiet,
    Capture(Piece),
    DoublePawnPush,
    Promotion(PromotionType),
    Castle(CastleType),
    CapturePromotion(PromotionType, Piece),
}
