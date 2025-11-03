pub mod pos;
pub mod piece;
use pos::Pos;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Colour { White, Black }

#[derive(Clone, Copy)]
pub struct Move {
    pub start: Pos,
    pub end: Pos
}

pub struct CastleInfo {
    kingside: bool,
    queenside: bool
}

impl CastleInfo {
    pub fn new() -> CastleInfo { CastleInfo { kingside: true, queenside: true } }
}