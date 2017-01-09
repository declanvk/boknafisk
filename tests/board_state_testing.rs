#![feature(test)]
extern crate test;

extern crate boknafisk;

use self::boknafisk::board_state::*;

#[test]
fn board_state_fen_conversions() {
    let fen_strings = vec![
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
        "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1",
        "rnbqkbnr/pp1ppppp/8/2p5/4P3/8/PPPP1PPP/RNBQKBNR w KQkq c6 0 2",
        "rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2"
    ];

    for fen_string in fen_strings {
        let board_state: BoardState = fen_string.parse().unwrap();
        assert_eq!(board_state.to_string(), fen_string);
    }
}