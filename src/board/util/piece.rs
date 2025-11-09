use super::Colour;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PieceType { King, Queen, Rook, Bishop, Knight, Pawn(bool) }

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

    pub fn is(&self, colour: Colour, class: PieceType) -> bool {
        self.colour == colour && self.class == class
    }
}