#![feature(try_from)]
#![feature(box_syntax)]


#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate lazy_static;
extern crate unicode_segmentation;

pub mod bit_boards;
pub mod piece_board;
pub mod board_state;
pub mod castling;
pub mod chess_move;
pub mod misc;
pub mod piece;
pub mod square_position;
pub mod error_types;
pub mod rkiss;
pub mod move_gen;
