#![feature(test)]
extern crate test;

extern crate boknafisk;

#[test]
fn it_works() {
    assert_eq!(4, 2 + 2);
}

#[test]
#[should_panic]
fn it_panics() {
    panic!("Expected panic")
}