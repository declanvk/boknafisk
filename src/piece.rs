use std::ops::Not;

#[derive(Debug, Copy, Hash, Clone, PartialEq, Eq)]
pub enum Color {
    White = 0,
    Black = 1,
}

impl Not for Color {
    type Output = Color;

    fn not(self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White
        }
    }
}


#[derive(Debug, Copy, Hash, Clone, PartialEq, Eq)]
pub enum PieceType {
    King = 0,
    Queen = 1,
    Bishop = 2,
    Knight = 3,
    Rook = 4,
    Pawn = 5,
}

#[derive(Debug, Copy, Hash, Clone, PartialEq, Eq)]
pub enum PromotionType {
    Queen,
    Bishop,
    Knight,
    Rook,
}

impl PieceType {
    pub fn value(&self) -> u32 {
        match *self {
            PieceType::King => 40000,
            PieceType::Queen => 1050,
            PieceType::Rook => 500,
            PieceType::Bishop => 325,
            PieceType::Knight => 325,
            PieceType::Pawn => 100,
        }
    }
}

#[derive(Debug, Copy, Hash, Clone)]
pub struct Piece {
    piece_type: PieceType,
    color: Color,
}

impl Piece {
    pub fn new(piece_type: PieceType, color: Color) -> Piece {
        Piece {
            piece_type: piece_type,
            color: color,
        }
    }

    pub fn value(&self) -> u32 {
        self.piece_type.value()
    }

    pub fn piece_type(&self) -> PieceType {
        self.piece_type
    }

    pub fn color(&self) -> Color {
        self.color
    }
}
