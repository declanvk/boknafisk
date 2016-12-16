mod chess_core;

use chess_core::board::*;
use chess_core::piece::*;
use chess_core::position::*;

fn main() {
    let board = Board::starting_board();
    
    println!("All pieces!");
    for (position, piece) in board.to_board_iter() {
        println!("{} contains {:?}", position, piece);
    }

    let white_pieces = board.to_board_iter()
        .filter(|&(_, piece)| piece.color() == Color::White)
        .collect::<Vec<(Position, Piece)>>();

    for (position, piece) in white_pieces {
        println!("{} contains {:?}", position, piece);
    }
}
