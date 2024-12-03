pub trait MathService {}

pub trait AddWithOverflow: MathService {
    fn add_with_overflow(&self, a: u8, b: u8) -> u8;
}

pub trait SubWithOverflow: MathService {
    fn sub_with_overflow(&self, a: u8, b: u8) -> u8;
}

pub trait Xor: MathService {
    fn xor(&self, a: u8, b: u8) -> u8;
}
