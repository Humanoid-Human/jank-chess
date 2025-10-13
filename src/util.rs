#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Colour { White, Black }

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PieceType { King, Queen, Rook, Bishop, Knight, Pawn }

#[derive(Clone, Copy)]
pub struct Move {
    pub start: (u8, u8),
    pub end: (u8, u8)
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Piece {
    pub piece: PieceType,
    pub colour: Colour
}

impl Piece {
    pub fn new(colour: Colour, piece: PieceType) -> Piece { Piece { piece, colour } }
}

pub fn board_to_rankfile(p: usize) -> (u8, u8) {
    ((p / 8) as u8, (p % 8) as u8)
}

pub fn rankfile_to_board(p: (u8, u8)) -> usize {
    (p.0 * 8 + p.1) as usize
}