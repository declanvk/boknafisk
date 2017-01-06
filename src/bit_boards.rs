use square_position::{SquarePosition, Direction, NORTH, SOUTH, EAST, WEST, INTERMEDIATE, CARDINAL};
use unicode_segmentation::UnicodeSegmentation;
use std::convert::TryFrom;

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
    pub static ref KING_DIRECTIONS: [Direction; 8] = {
        [
            NORTH,
            NORTH + EAST,
            EAST,
            SOUTH + EAST,
            SOUTH,
            SOUTH + WEST,
            WEST,
            NORTH + WEST
        ]
    };
    pub static ref KNIGHT_DIRECTIONS: [Direction; 8] = {
        [
            NORTH + NORTH + EAST,
            NORTH + EAST + EAST,
            SOUTH + EAST + EAST,
            SOUTH + SOUTH + EAST,
            SOUTH + SOUTH + WEST,
            SOUTH + WEST + WEST,
            NORTH + WEST + WEST,
            NORTH + NORTH + WEST
        ]
    };
    pub static ref KNIGHT_ATTACK_BOARDS: [BitBoard; 64] = generate_attacks(&*KNIGHT_DIRECTIONS);
    pub static ref KING_ATTACK_BOARDS: [BitBoard; 64] = generate_attacks(&*KING_DIRECTIONS);
    pub static ref BISHOP_PRE_MASKS: [BitBoard; 64] = generate_mask(&INTERMEDIATE, false);
    pub static ref BISHOP_POST_MASKS: [BitBoard; 64] = generate_mask(&INTERMEDIATE, true);
    pub static ref ROOK_PRE_MASKS: [BitBoard; 64] = generate_mask(&CARDINAL, false);
    pub static ref ROOK_POST_MASKS: [BitBoard; 64] = generate_mask(&CARDINAL, true);
}

fn generate_mask(directions: &[Direction], edges: bool) -> [BitBoard; 64] {
    let mut masks = [0 as BitBoard; 64]; 
    for square_index in 0..64 {
        if let Ok(position) = TryFrom::try_from(square_index) {
            for direction in directions {
                let mut rook_position: SquarePosition = position;
                while let Some(new_pos) = rook_position + *direction {
                    rook_position = new_pos;

                    masks[square_index as usize] |= 1 << rook_position.to_square_index();
                }

                if !edges {
                    masks[square_index as usize] &= !(1 << rook_position.to_square_index());
                }
            }
        }
    }

    masks
}

fn generate_attacks(directions: &[Direction]) -> [BitBoard; 64] {
        let mut attacks = [0 as BitBoard; 64];

        for square_index in 0..64 {
            let position = SquarePosition::try_from(square_index);
            if let Ok(pos) = position {
                for direction in directions {
                    if let Some(new_pos) = pos + *direction {
                        attacks[square_index as usize] |= 1 << new_pos.to_square_index();
                    }
                }
            }
        }

        attacks
    }

const INDEX_64: [usize; 64] = [
    0, 47,  1, 56, 48, 27,  2, 60,
   57, 49, 41, 37, 28, 16,  3, 61,
   54, 58, 35, 52, 50, 42, 21, 44,
   38, 32, 29, 23, 17, 11,  4, 62,
   46, 55, 26, 59, 40, 36, 15, 53,
   34, 51, 20, 43, 31, 22, 10, 45,
   25, 39, 14, 33, 19, 30,  9, 24,
   13, 18,  8, 12,  7,  6,  5, 63
];

const DEBRUIJIN_64: u64 =  0x03f79d71b4cb0a89;

pub fn bit_scan_forward(board: BitBoard) -> Option<usize> {
    if board == 0 {
        None
    } else {
        Some(INDEX_64[((board ^ (board-1)).wrapping_mul(DEBRUIJIN_64) >> 58) as  usize])
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
    mask: BitBoard
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
        mask: 0
    }
}

pub fn bit_to_str(&board: &BitBoard) -> String {
    let mut output = format!("{:064b}", board);

    let mut index = 8;
    for _ in 1..8 {
        output.insert(index, '\n');
        index += 9;
    }

    output = output
                .split_whitespace()
                .map(|x| x.graphemes(true)
                          .rev()
                          .flat_map(|g| g.chars())
                          .collect::<String>() + "\n")
                .collect::<String>();

    output = output.replace('0', ".");
    output = output.replace("", " ");

    output
}

pub struct BitSubsetIter<'a> {
    board: &'a BitBoard,
    subset: BitBoard
}

impl<'a> Iterator for BitSubsetIter<'a> {
    type Item = BitBoard;

    fn next(&mut self) -> Option<Self::Item> {
        if self.subset == 0 {
            None
        } else {
            let result = self.subset;
            self.subset = (self.subset.wrapping_sub(*self.board)) & *self.board;
            Some(result)
        }
    }
}

pub fn subsets_iterator(set: &BitBoard) -> BitSubsetIter {
    BitSubsetIter {
        board: set,
        subset: ((0 as BitBoard).wrapping_sub(*set)) & *set
    }
}