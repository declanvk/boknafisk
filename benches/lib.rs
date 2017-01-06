#![feature(test)]
extern crate test;

extern crate boknafisk;

use test::Bencher;
use boknafisk::bit_boards::{bit_to_str, BISHOP_POST_MASKS};

#[bench]
fn bench_(b: &mut test::Bencher) {
    b.iter(|| bit_to_str(&(*BISHOP_POST_MASKS)[23]))
}