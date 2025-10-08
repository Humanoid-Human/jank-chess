#[derive(Clone, Copy)]
enum Colour { White, Black }
#[derive(Clone, Copy)]
enum PieceType { King, Queen, Rook, Bishop, Knight, Pawn }
use Colour::*;
use PieceType::*;

struct Piece {
    position: (u8, u8),
    colour: Colour,
    piece_type: PieceType,
    can_en_passant: bool
}

impl Piece {
    pub fn new(position: (u8, u8), colour: Colour, piece_type: PieceType) -> Piece {
        Piece { position, colour, piece_type, can_en_passant: false}
    }
}

struct Move {
    start: (u8, u8),
    end: (u8, u8)
}

struct Board {
    pieces: Vec<Piece>,
    white_castle: bool,
    black_castle: bool,
    turn: Colour
}

impl Board {
    pub fn starting_position() -> Board {
        let mut pieces = Vec::with_capacity(32);

        // pawns
        for i in 0..8 {
            pieces.push(Piece::new((i, 1), White, Pawn));
            pieces.push(Piece::new((i, 6), Black, Pawn));
        }

        // king and queen
        pieces.push(Piece::new((3, 0), White, Queen));
        pieces.push(Piece::new((3, 7), Black, Queen));
        pieces.push(Piece::new((4, 0), White, King));
        pieces.push(Piece::new((4, 7), Black, King));

        // other pieces
        let a = [Rook, Knight, Bishop];
        for i in 0..3 {
            pieces.push(Piece::new((i, 0), White, a[i as usize]));
            pieces.push(Piece::new((i, 7), Black, a[i as usize]));
        }

        Board { pieces, white_castle: true, black_castle: true, turn: Colour::White }
    }
}