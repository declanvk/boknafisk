use std::convert::TryFrom;

use bit_boards::{BitBoard, subsets_iterator};
use square_position::{CARDINAL, Direction, EAST, INTERMEDIATE, NORTH, SOUTH, SquarePosition, WEST};
use piece::PieceType;
use rkiss::{MAGIC_BOOSTERS, RKISS};

const ROOK_ATTACKS_SIZE: usize = 0x19000;
const BISHOP_ATTACKS_SIZE: usize = 0x1480;
const KING_ATTACKS_SIZE: usize = 64;
const KNIGHT_ATTACKS_SIZE: usize = 64;

lazy_static! {
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
}

pub trait BitboardGenerator {
    fn get_attacks(&self, position: SquarePosition, occupancy: BitBoard) -> BitBoard;
}

pub struct StepAttackBoard {
    piece_type: PieceType,
    attacks: Box<[BitBoard]>,
}

impl StepAttackBoard {
    pub fn new(piece_type: PieceType) -> StepAttackBoard {
        let directions: &[Direction] = &match piece_type {
            PieceType::King => *KING_DIRECTIONS,
            PieceType::Knight => *KNIGHT_DIRECTIONS,
            _ => panic!("Illegal piece type argument"),
        };

        let attack_array: Box<[BitBoard]> = match piece_type {
            PieceType::King => box [0 as BitBoard; KING_ATTACKS_SIZE],
            PieceType::Knight => box [0 as BitBoard; KNIGHT_ATTACKS_SIZE],
            _ => panic!("Illegal piece type argument"),
        };

        let mut step_board = StepAttackBoard {
            piece_type: piece_type,
            attacks: attack_array,
        };

        step_board.generate_step_attacks(&directions);

        step_board
    }

    fn generate_step_attacks(&mut self, directions: &[Direction]) {
        let steps = (0..64)
            .map(|x| (x, SquarePosition::try_from(x).unwrap()))
            .flat_map(|(index, pos)| directions.iter().map(move |dir| (index, pos + *dir)))
            .filter(|&(_, new_pos)| new_pos.is_some())
            .map(|(orig, new_pos)| (orig, new_pos.unwrap()));

        for (original, new_pos) in steps {
            self.attacks[original as usize] |= 1 << new_pos.to_square_index();
        }
    }
}

impl BitboardGenerator for StepAttackBoard {
    fn get_attacks(&self, position: SquarePosition, occupancy: BitBoard) -> BitBoard {
        self.attacks[position.to_square_index()]
    }
}

pub struct MagicAttackBoard {
    piece_type: PieceType,
    shifts: [usize; 64],
    offsets: [usize; 64],
    masks: [BitBoard; 64],
    magics: [BitBoard; 64],
    attacks: Box<[BitBoard]>,
}

impl MagicAttackBoard {
    pub fn new(piece_type: PieceType) -> MagicAttackBoard {
        let directions: &[Direction] = &match piece_type {
            PieceType::Rook => CARDINAL,
            PieceType::Bishop => INTERMEDIATE,
            _ => panic!("Illegal piece type argument"),
        };

        let attack_array: Box<[BitBoard]> = match piece_type {
            PieceType::Rook => box [0 as BitBoard; ROOK_ATTACKS_SIZE],
            PieceType::Bishop => box [0 as BitBoard; BISHOP_ATTACKS_SIZE],
            _ => panic!("Illegal piece type argument"),
        };

        let mut magic_board = MagicAttackBoard {
            piece_type: piece_type,
            masks: [0 as BitBoard; 64],
            shifts: [0 as usize; 64],
            offsets: [0 as usize; 64],
            magics: [0 as BitBoard; 64],
            attacks: attack_array,
        };

        magic_board.generate_magics(MAGIC_BOOSTERS, directions);

        magic_board
    }

    pub fn piece_type(&self) -> PieceType {
        self.piece_type
    }

    pub fn masks(&self) -> &[BitBoard] {
        &self.masks
    }

    pub fn shifts(&self) -> &[usize] {
        &self.shifts
    }

    pub fn offsets(&self) -> &[usize] {
        &self.offsets
    }

    pub fn magics(&self) -> &[BitBoard] {
        &self.magics
    }

    pub fn attacks(&self) -> &[BitBoard] {
        &self.attacks
    }

    pub fn compute_index(&self, position: SquarePosition, occupancy: BitBoard) -> usize {
        let square_index = position.to_square_index();
        self.offsets[square_index] +
        (((occupancy & self.masks[square_index]).wrapping_mul(self.magics[square_index])) >>
         self.shifts[square_index]) as usize
    }

    fn generate_magics(&mut self, boosters: [usize; 8], directions: &[Direction]) {
        let mut rkiss = RKISS::new(203);
        let mut reference = [0 as BitBoard; 4096];
        let mut occupancy = [0 as BitBoard; 4096];

        self.offsets[0] = 0;

        for square_index in 0..64 {
            let random_booster = boosters[(square_index >> 3) as usize];

            self.masks[square_index] = generate_mask(square_index, directions, false);
            self.shifts[square_index] = 64 - (self.masks[square_index].count_ones() as usize);

            let mut size = 0;
            for (subset_index, subset) in subsets_iterator(&self.masks[square_index]).enumerate() {
                occupancy[subset_index] = subset;
                reference[subset_index] =
                    generate_sliding_attack_bitboard(directions,
                                                     SquarePosition::try_from(square_index)
                                                         .unwrap(),
                                                     subset);

                size += 1;
            }

            if square_index < 63 {
                self.offsets[square_index + 1] = self.offsets[square_index] + size;
            }

            let index_range = self.offsets[square_index]..(self.offsets[square_index] + size);
            loop {
                self.magics[square_index] =
                    magic_few_bits(random_booster, &mut rkiss, self.masks[square_index]);

                for value in &mut self.attacks[index_range.clone()] {
                    *value = 0;
                }

                let mut failed = false;

                for index in 0..size {
                    let magic_index = magic_index(self.offsets[square_index],
                                                  self.shifts[square_index],
                                                  occupancy[index],
                                                  self.masks[square_index],
                                                  self.magics[square_index]);

                    if self.attacks[magic_index] != 0 &&
                       self.attacks[magic_index] != reference[index] {
                        failed = true;
                        break;
                    }

                    assert!(reference[index] != 0);

                    self.attacks[magic_index] = reference[index];
                }


                if !failed {
                    break;
                }
            }
        }
    }
}

impl BitboardGenerator for MagicAttackBoard {
    fn get_attacks(&self, position: SquarePosition, occupancy: BitBoard) -> BitBoard {
        self.attacks[self.compute_index(position, occupancy)]
    }
}

fn magic_index(offset: usize,
               shift: usize,
               occupancy: BitBoard,
               mask: BitBoard,
               magic: BitBoard)
               -> usize {
    offset + (((occupancy & mask).wrapping_mul(magic)) >> shift) as usize
}

fn magic_few_bits(random_booster: usize, rkiss: &mut RKISS, mask: BitBoard) -> BitBoard {
    let mut magic = 0;

    loop {
        magic = rkiss.magic_rand(random_booster) as BitBoard;

        if ((magic.wrapping_mul(mask)) >> 56).count_ones() < 6 {
            break;
        }
    }

    magic
}

pub fn generate_mask(square_index: usize, directions: &[Direction], edges: bool) -> BitBoard {
    let mut result: BitBoard = 0;
    if let Ok(position) = TryFrom::try_from(square_index) {
        for direction in directions {
            let mut rook_position: SquarePosition = position;
            while let Some(new_pos) = rook_position + *direction {
                rook_position = new_pos;

                result |= 1 << rook_position.to_square_index();
            }

            if !edges {
                result &= !(1 << rook_position.to_square_index());
            }
        }
    }
    result
}

pub fn generate_sliding_attacks(ref magic_attack_boards: &MagicAttackBoard,
                                ref mut attacks_array: &mut [BitBoard],
                                directions: &[Direction]) {
    for (square_index, attack_index_offset) in magic_attack_boards.offsets()
        .iter()
        .enumerate() {

        for (subset_index, subset_occupancy) in
            subsets_iterator(&magic_attack_boards.masks()[square_index]).enumerate() {
            let position = SquarePosition::try_from(square_index).unwrap();

            let attack = generate_sliding_attack_bitboard(directions, position, subset_occupancy);
            attacks_array[attack_index_offset + subset_index] = attack;
        }
    }
}

pub fn generate_sliding_attack_bitboards(mask: BitBoard,
                                         directions: &[Direction],
                                         position: SquarePosition)
                                         -> Vec<BitBoard> {
    subsets_iterator(&mask)
        .map(|x| generate_sliding_attack_bitboard(directions, position, x))
        .collect::<Vec<BitBoard>>()
}

fn generate_sliding_attack_bitboard(directions: &[Direction],
                                    position: SquarePosition,
                                    occupied: BitBoard)
                                    -> BitBoard {
    let mut result: BitBoard = 0;

    for &direction in directions {
        let mut ray_index = position.clone();
        while let Some(new_pos) = ray_index + direction {
            result |= new_pos.to_bit_board();
            ray_index = new_pos;

            if (occupied & new_pos.to_bit_board()) != 0 {
                break;
            }
        }
    }

    result
}

#[cfg(test)]
mod test {

    use std::mem::{size_of, size_of_val};

    use bit_boards::BitBoard;
    use square_position::{CARDINAL, Direction, SquarePosition};
    use move_gen::{AttackBoardCollection, BISHOP_ATTACKS_SIZE, KING_ATTACKS_SIZE,
                   KNIGHT_ATTACKS_SIZE, MagicAttackBoard, ROOK_ATTACKS_SIZE,
                   generate_sliding_attack_bitboard, generate_sliding_attack_bitboards};
    use piece::PieceType;

    #[test]
    fn get_size_of_test() {
        let rook_attacks: Box<[BitBoard]> = box [0 as BitBoard; ROOK_ATTACKS_SIZE];
        let bishop_attacks: Box<[BitBoard]> = box [0 as BitBoard; BISHOP_ATTACKS_SIZE];
        let knight_attacks: Box<[BitBoard]> = box [0 as BitBoard; KNIGHT_ATTACKS_SIZE];
        let king_attacks: Box<[BitBoard]> = box [0 as BitBoard; KING_ATTACKS_SIZE];

        assert_eq!(size_of_val(&*rook_attacks),
                   size_of::<BitBoard>() * ROOK_ATTACKS_SIZE);
        assert_eq!(size_of_val(&*bishop_attacks),
                   size_of::<BitBoard>() * BISHOP_ATTACKS_SIZE);
        assert_eq!(size_of_val(&*knight_attacks),
                   size_of::<BitBoard>() * KNIGHT_ATTACKS_SIZE);
        assert_eq!(size_of_val(&*king_attacks),
                   size_of::<BitBoard>() * KING_ATTACKS_SIZE);
    }

    #[test]
    fn rook_magic_index_test() {
        let board = MagicAttackBoard::new(PieceType::Rook);

        let positions: Vec<SquarePosition> = [(2, 4), (4, 2), (6, 2), (2, 1), (7, 4)]
            .iter()
            .map(|&(rank, file)| SquarePosition::new(rank, file))
            .collect();

        let occupancies: Vec<BitBoard> =
            vec![0x4114442008204d3, 0x101412c000024491, 0x70400080818181d7, 0x2000000080200, 0x0];

        let parameters: Vec<(SquarePosition, BitBoard)> =
            positions.into_iter().zip(occupancies.into_iter()).collect();

        let magic_iter = parameters.iter()
            .map(|&(position, occupancy)| {
                (position, occupancy, board.get_attacks(position, occupancy))
            });

        for (position, occupancy, magic_attack) in magic_iter {
            let reference_attack = generate_sliding_attack_bitboard(&CARDINAL, position, occupancy);

            if magic_attack != reference_attack {

            }

            assert_eq!(magic_attack, reference_attack);
        }
    }
}
