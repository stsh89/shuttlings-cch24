use crate::core::services::{AddWithOverflow, MathService, SubWithOverflow, Xor};

pub struct BasicMathService;

impl MathService for BasicMathService {}

impl AddWithOverflow for BasicMathService {
    fn add_with_overflow(&self, a: u8, b: u8) -> u8 {
        a.wrapping_add(b)
    }
}

impl SubWithOverflow for BasicMathService {
    fn sub_with_overflow(&self, a: u8, b: u8) -> u8 {
        a.wrapping_sub(b)
    }
}

impl Xor for BasicMathService {
    fn xor(&self, a: u8, b: u8) -> u8 {
        a ^ b
    }
}
