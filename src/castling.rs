use std::fmt;
use std::str::FromStr;
use error_types::FromStrError;

bitflags! {
    pub flags CastlingFlags: u8 {
        const WHITE_QUEENSIDE = 0b0000001,
        const WHITE_KINGSIDE  = 0b0000010,
        const BLACK_QUEENSIDE = 0b0000100,
        const BLACK_KINGSIDE  = 0b0001000
    }
}

pub enum CastleType {
    Kingside,
    Queenside,
}

impl fmt::Display for CastlingFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result: String = String::new();

        if self.is_empty() {
            result += "-";
        } else {
            if self.intersects(WHITE_KINGSIDE) {
                result += "K";
            }

            if self.intersects(WHITE_QUEENSIDE) {
                result += "Q";
            }

            if self.intersects(BLACK_KINGSIDE) {
                result += "k";
            }

            if self.intersects(BLACK_QUEENSIDE) {
                result += "q";
            }
        }

        write!(f, "{}", result)
    }
}

impl FromStr for CastlingFlags {
    type Err = FromStrError;
    fn from_str(castling_string: &str) -> Result<Self, Self::Err> {
        if castling_string.len() == 0 || castling_string.len() > 4 {
            println!("Equals zero: {:?}", castling_string.len() == 0);
            println!("Greater than four {:?}", castling_string.len() > 4);
            println!("Both? {:?}",
                     castling_string.len() == 0 || castling_string.len() > 4);
            Err(FromStrError::InvalidInputLength("castling", 4, castling_string.len()))
        } else {
            Ok(castling_string.chars()
                .map(|x| {
                    match x {
                        'K' => Some(WHITE_KINGSIDE),
                        'Q' => Some(WHITE_QUEENSIDE),
                        'k' => Some(BLACK_KINGSIDE),
                        'q' => Some(BLACK_QUEENSIDE),
                        '-' => None,
                        _ => None,
                    }
                })
                .fold(CastlingFlags::empty(), |acc, x| {
                    match x {
                        Some(value) => acc | value,
                        None => acc,
                    }
                }))
        }
    }
}

#[cfg(test)]
mod test {
    use castling::{BLACK_KINGSIDE, BLACK_QUEENSIDE, CastlingFlags, WHITE_KINGSIDE, WHITE_QUEENSIDE};

    #[test]
    fn str_to_flag_test() {
        let test_flag_input_strings = vec!["K", "Q", "qK", "Qk", "qkK", "kqK", "kKq", "QkKq",
                                           "QkqK", "QqkK", "qQkK"];

        let expected_result_flags = vec![WHITE_KINGSIDE,
                                         WHITE_QUEENSIDE,
                                         BLACK_QUEENSIDE | WHITE_KINGSIDE,
                                         WHITE_QUEENSIDE | BLACK_KINGSIDE,
                                         BLACK_QUEENSIDE | BLACK_KINGSIDE | WHITE_KINGSIDE,
                                         BLACK_KINGSIDE | BLACK_QUEENSIDE | WHITE_KINGSIDE,
                                         BLACK_KINGSIDE | WHITE_KINGSIDE | BLACK_QUEENSIDE,
                                         CastlingFlags::all(),
                                         CastlingFlags::all(),
                                         CastlingFlags::all(),
                                         CastlingFlags::all()];

        let test_iter = test_flag_input_strings.into_iter().zip(expected_result_flags.into_iter());
        for (input, expected_output) in test_iter {
            if let Ok(test_output) = input.parse::<CastlingFlags>() {
                assert_eq!(test_output, expected_output);
            }
        }
    }

    #[test]
    fn flag_to_str() {
        let expected_output_str = vec!["KQkq", "KQk", "KQq", "Kkq", "Qkq", "Kq", "Qk", "Kk", "Qq",
                                       "K", "Q", "k", "q"];

        let test_flag_inputs = vec![CastlingFlags::all(),
                                    WHITE_KINGSIDE | WHITE_QUEENSIDE | BLACK_KINGSIDE,
                                    WHITE_KINGSIDE | WHITE_QUEENSIDE | BLACK_QUEENSIDE,
                                    WHITE_KINGSIDE | BLACK_KINGSIDE | BLACK_QUEENSIDE,
                                    WHITE_QUEENSIDE | BLACK_KINGSIDE | BLACK_QUEENSIDE,
                                    WHITE_KINGSIDE | BLACK_QUEENSIDE,
                                    WHITE_QUEENSIDE | BLACK_KINGSIDE,
                                    WHITE_KINGSIDE | BLACK_KINGSIDE,
                                    WHITE_QUEENSIDE | BLACK_QUEENSIDE,
                                    WHITE_KINGSIDE,
                                    WHITE_QUEENSIDE,
                                    BLACK_KINGSIDE,
                                    BLACK_QUEENSIDE];

        let test_iter = test_flag_inputs.into_iter().zip(expected_output_str.into_iter());
        for (input, expected_output) in test_iter {
            assert_eq!(input.to_string(), expected_output);
        }
    }

}
