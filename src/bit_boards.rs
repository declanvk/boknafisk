use std::convert::From;

use unicode_segmentation::UnicodeSegmentation;

use square_position::SquarePosition;
use piece_board::PieceBoard;

pub type BitBoard = u64;

lazy_static! {
    pub static ref FILE_BOARDS: [BitBoard; 8] = {
        let mut file_boards = [0x101010101010101 as BitBoard; 8];

        for index in 0..file_boards.len() {
            file_boards[index] = file_boards[index] << index;
        }

        file_boards
    };
    pub static ref RANK_BOARDS: [BitBoard; 8] = {
        let mut rank_boards = [0xff as BitBoard; 8];

        for index in 0..rank_boards.len() {
            rank_boards[index] = rank_boards[index] << (8 * index);
        }

        rank_boards
    };
}

#[cfg_attr(rustfmt, rustfmt_skip)]
const INDEX_64: [usize; 64] = [
    00, 47, 01, 56, 48, 27, 02, 60,
    57, 49, 41, 37, 28, 16, 03, 61,
    54, 58, 35, 52, 50, 42, 21, 44,
    38, 32, 29, 23, 17, 11, 04, 62,
    46, 55, 26, 59, 40, 36, 15, 53,
    34, 51, 20, 43, 31, 22, 10, 45,
    25, 39, 14, 33, 19, 30, 09, 24,
    13, 18, 08, 12, 07, 06, 05, 63
];

const DEBRUIJIN_64: u64 = 0x03f79d71b4cb0a89;

pub fn bit_scan_forward(board: BitBoard) -> Option<usize> {
    if board == 0 {
        None
    } else {
        Some(INDEX_64[((board ^ (board - 1)).wrapping_mul(DEBRUIJIN_64) >> 58) as usize])
    }
}

pub fn bit_scan_reverse(mut board: BitBoard) -> Option<usize> {
    if board == 0 {
        None
    } else {
        board |= board >> 1;
        board |= board >> 2;
        board |= board >> 4;
        board |= board >> 8;
        board |= board >> 16;
        board |= board >> 32;

        Some(INDEX_64[(board.wrapping_mul(DEBRUIJIN_64) >> 58) as usize])
    }
}

pub struct BitBoardIter<'iter> {
    board: &'iter BitBoard,
    mask: BitBoard,
}

impl<'iter> Iterator for BitBoardIter<'iter> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let square_index = bit_scan_forward(*self.board & !self.mask);
        self.mask |= 1 << square_index.unwrap_or(0);
        square_index
    }
}

pub fn bit_iterator<'a>(board: &'a BitBoard) -> BitBoardIter<'a> {
    BitBoardIter {
        board: board,
        mask: 0,
    }
}

pub fn bit_to_str(&board: &BitBoard) -> String {
    let mut output = format!("{:064b}", board);

    let mut index = 8;
    for _ in 1..8 {
        output.insert(index, '\n');
        index += 9;
    }

    output = output.split_whitespace()
        .map(|x| {
            x.graphemes(true)
                .rev()
                .flat_map(|g| g.chars())
                .collect::<String>() + "\n"
        })
        .collect::<String>();

    output = output.replace('0', ".");
    output = output.replace("", " ");

    output
}

#[derive(Debug)]
pub struct BitSubsetIter<'a> {
    board: &'a BitBoard,
    subset: BitBoard,
    count: usize,
}

impl<'a> Iterator for BitSubsetIter<'a> {
    type Item = BitBoard;

    fn next(&mut self) -> Option<Self::Item> {
        if self.subset == 0 && self.count > 0 {
            None
        } else if *self.board == 0 {
            None
        } else {
            let result = self.subset;
            self.subset = (self.subset.wrapping_sub(*self.board)) & *self.board;
            self.count += 1;
            Some(result)
        }
    }
}

pub fn subsets_iterator(set: &BitBoard) -> BitSubsetIter {
    BitSubsetIter {
        board: set,
        subset: 0,
        count: 0,
    }
}

impl From<SquarePosition> for BitBoard {
    fn from(position: SquarePosition) -> BitBoard {
        1 << position.to_square_index()
    }
}

impl From<PieceBoard> for ([[BitBoard; 6]; 2], [BitBoard; 2]) {
    fn from(piece_board: PieceBoard) -> ([[BitBoard; 6]; 2], [BitBoard; 2]) {
        let mut bit_board = [[0 as BitBoard; 6]; 2];
        let mut bit_occupancy = [0 as BitBoard; 2];

        for (position, piece) in piece_board.into_iter() {
            bit_board[piece.color() as usize][piece.piece_type() as usize] |=
                1 << position.to_square_index();
            bit_occupancy[piece.color() as usize] |= 1 << position.to_square_index();
        }

        (bit_board, bit_occupancy)
    }
}

#[cfg(test)]
mod test {

    use bit_boards::{BitBoard, subsets_iterator};

    #[test]
    fn subset_iterator_contains_zero_test() {
        let mask: BitBoard = 0x1010106e101000;

        assert!(subsets_iterator(&mask).any(|x| x == 0));
    }

    #[test]
    fn subset_iterator_correct_len_test() {
        let mask: BitBoard = 0x1010106e101000;
        let count_size: usize = subsets_iterator(&mask).count();
        let expected_size: usize = 2u32.pow(mask.count_ones()) as usize;
        assert_eq!(count_size, expected_size);
    }
}
