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

    pub fn pawn_dir(&self) -> Pos {
        match self {
            Colour::White => Pos::new(1, 0),
            Colour::Black => Pos::new(-1, 0)
        }
    }
    pub fn start_row(&self) -> i8 {
        match self {
            Colour::White => 0,
            Colour::Black => 7
        }
    }
    pub fn pawn_row(&self) -> i8 {
        match self {
            Colour::White => 1,
            Colour::Black => 6
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Move {
    pub start: Pos,
    pub end: Pos
}

impl Move {
    pub fn delta(&self) -> Pos {
        self.end - self.start
    }

    pub fn castle_kingside(colour: Colour) -> Move {
        let row = match colour {
            Colour::White => 0,
            Colour::Black => 7
        };
        Move{start: Pos::new(row, 7), end: Pos::new(row, 5)}
    }

    pub fn castle_queenside(colour: Colour) -> Move {
        let row = match colour {
            Colour::White => 0,
            Colour::Black => 7
        };
        Move{start: Pos::new(row, 3), end: Pos::new(row, 5)}
    }
}

#[derive(Clone, Copy)]
pub struct CastleInfo {
    pub kingside: bool,
    pub queenside: bool
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
    pub fn can_castle(&self) -> bool {
        self.kingside || self.queenside
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum GameEnd {
    Win(Colour), Draw
}

#[derive(PartialEq, Eq)]
pub enum MoveError {
    OutOfBoard, NoPiece, WrongColour, NotLegal
}