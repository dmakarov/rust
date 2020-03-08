// run-pass
#![feature(const_discriminant)]

use std::mem::{discriminant, Discriminant};

fn identity<T>(x: T) -> T { x }

enum Test {
    A(u8),
    B,
    C { a: u8, b: u8 },
}

const TEST_A: Discriminant<Test> = discriminant(&Test::A(5));
const TEST_A_OTHER: Discriminant<Test> = discriminant(&Test::A(17));
const TEST_B: Discriminant<Test> = discriminant(&Test::B);

fn main() {
    assert_eq!(TEST_A, TEST_A_OTHER);
    assert_eq!(TEST_A, discriminant(identity(&Test::A(17))));
    assert_eq!(TEST_B, discriminant(identity(&Test::B)));
    assert_ne!(TEST_A, TEST_B);
    assert_ne!(TEST_B, discriminant(identity(&Test::C { a: 42, b: 7 })));
}
