#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Colour { White, Black }
#[derive(Clone, Copy)]
pub enum PieceType { King, Queen, Rook, Bishop, Knight, Pawn }

#[derive(Clone, Copy)]
pub struct Move {
    start: (u8, u8),
    end: (u8, u8)
}

pub fn board_to_rankfile(p: u8) -> (u8, u8) {
    return (p / 8, p % 8);
}

pub fn rankfile_to_board(p: (u8, u8)) -> usize {
    return (p.0 * 8 + p.1) as usize;
}