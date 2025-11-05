pub mod pos;
pub mod piece;
pub mod constants;
use pos::Pos;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Colour { White, Black }

impl Colour {
    pub fn opposite(&self) -> Colour {
        match self {
            Colour::White => Colour::Black,
            Colour::Black => Colour::White
        }
    }

    pub fn pawn_dir(&self) -> i8 {
        match self {
            Colour::White => 1,
            Colour::Black => -1
        }
    }
}

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
    pub fn king_moved(&mut self) {
        self.kingside = false;
        self.queenside = false;
    }
    pub fn kingside_rook(&mut self) {
        self.kingside = false;
    }
    pub fn queenside_rook(&mut self) {
        self.queenside = false;
    }
}