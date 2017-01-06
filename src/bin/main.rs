extern crate boknafisk;

use boknafisk::bit_boards::*;

fn main() {
    println!("{}", bit_to_str(&(*BISHOP_POST_MASKS)[23]));

    let iter = subsets_iterator(&(*BISHOP_POST_MASKS)[23]);
    let collection = subsets_iterator(&(*BISHOP_POST_MASKS)[23]).collect::<Vec<_>>();
    for subset in iter {
        println!("{}", bit_to_str(&subset));
    }
    println!("Size: {}", collection.len());
}