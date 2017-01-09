use std::ops::Not;
use std::fmt;
use std::str::FromStr;
use error_types::FromStrError;

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

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str_repr = match *self {
            Color::White => "w",
            Color::Black => "b"
        };

        write!(f, "{}", str_repr)
    }
}

impl FromStr for Color {
    type Err = FromStrError;

    fn from_str(color_str: &str) -> Result<Color, Self::Err> {
        if color_str.len() > 1 || color_str.len() == 0 {
            Err(FromStrError::InvalidInputLength("color", 1, color_str.len()))
        } else {
            match color_str {
                "w" => Ok(Color::White),
                "b" => Ok(Color::Black),
                _ => Err(FromStrError::MalformedInput("color"))
            }
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

impl fmt::Display for PieceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string_repr = match self {
            &PieceType::King => "K",
            &PieceType::Queen => "Q",
            &PieceType::Bishop => "B",
            &PieceType::Knight => "N",
            &PieceType::Rook => "R",
            &PieceType::Pawn => "P",             
        };

        write!(f, "{}", string_repr)
    }
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

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.color {
            Color::White => write!(f, "{}", self.piece_type.to_string().as_str().to_uppercase()),
            Color::Black => write!(f, "{}", self.piece_type.to_string().as_str().to_lowercase())
        }
        
    }
}

impl FromStr for Piece {
    type Err = FromStrError;

    fn from_str(piece_string: &str) -> Result<Piece, Self::Err> {
        match piece_string {
            "K" => Ok(Piece::new(PieceType::King, Color::White)),
            "Q" => Ok(Piece::new(PieceType::Queen, Color::White)),
            "B" => Ok(Piece::new(PieceType::Bishop, Color::White)),
            "N" => Ok(Piece::new(PieceType::Knight, Color::White)),
            "R" => Ok(Piece::new(PieceType::Rook, Color::White)),
            "P" => Ok(Piece::new(PieceType::Pawn, Color::White)),
            "k" => Ok(Piece::new(PieceType::King, Color::Black)),
            "q" => Ok(Piece::new(PieceType::Queen, Color::Black)),
            "b" => Ok(Piece::new(PieceType::Bishop, Color::Black)),
            "n" => Ok(Piece::new(PieceType::Knight, Color::Black)),
            "r" => Ok(Piece::new(PieceType::Rook, Color::Black)),
            "p" => Ok(Piece::new(PieceType::Pawn, Color::Black)),
            _ => Err(FromStrError::MalformedInput("piece"))
        }
    }
}