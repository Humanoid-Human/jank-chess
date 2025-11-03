pub mod pos;
use pos::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Colour { White, Black }

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PieceType { King, Queen, Rook, Bishop, Knight, Pawn }

#[derive(Clone, Copy)]
pub struct Move {
    pub start: Pos,
    pub end: Pos
}

#[derive(Clone, Copy)]
pub struct Piece {
    pub class: PieceType,
    pub colour: Colour
}

impl Piece {
    pub fn new(colour: Colour, class: PieceType) -> Piece {
        Piece { class, colour }
    }

    pub fn some(colour: Colour, class: PieceType) -> Option<Piece> {
        Some(Piece { class, colour })
    }
}