use std::ops::{Add, AddAssign, Mul, MulAssign, Sub};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Pos { pub row: i8, pub col: i8 }

impl Pos {
    pub fn new(row: i8, col: i8) -> Pos {
        Pos {row, col}
    }

    pub fn from(i: usize) -> Pos {
        Pos { row: (i / 8) as i8, col: (i % 8) as i8 }
    }

    pub fn is_on_board(&self) -> bool {
        self.row >= 0 && self.row <= 7 && self.col >= 0 && self.col <= 7
    }

    pub fn bitmap(&self) -> u64 {
        (1_u64) << (self.row * 8 + self.col)
    }
}

impl Add for Pos {
    type Output = Pos;
    fn add(self, rhs: Pos) -> Pos {
        Pos {row: self.row + rhs.row, col: self.col + rhs.col}
    }
}

impl AddAssign for Pos {
    fn add_assign(&mut self, rhs: Pos) {
        self.row += rhs.row;
        self.col += rhs.col;
    }
}

impl Sub for Pos {
    type Output = Pos;
    fn sub(self, rhs: Self) -> Self::Output {
        Pos { row: self.row - rhs.row, col: self.col - rhs.col }
    }
}

impl Mul<i8> for Pos {
    type Output = Pos;

    fn mul(self, rhs: i8) -> Pos {
        Pos { col: self.col * rhs, row: self.row * rhs }
    }
}

impl MulAssign<i8> for Pos {
    fn mul_assign(&mut self, rhs: i8) {
        self.row *= rhs;
        self.col *= rhs;
    }
}