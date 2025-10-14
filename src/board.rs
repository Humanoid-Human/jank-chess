use crate::util::{*, Colour::*, PieceType::*};

pub struct CastleInfo {
    kingside: bool,
    queenside: bool
}

impl CastleInfo {
    fn new() -> CastleInfo { CastleInfo { kingside: true, queenside: true } }
}

struct Board {
    pieces: [Option<Piece>; 64],
    white_castle: CastleInfo,
    black_castle: CastleInfo,
    turn: Colour
}

impl Board {
    pub fn starting_position() -> Board {
        let mut pieces = [None; 64];

        // pawns
        for i in 0..8 {
            pieces[rf_to_board((1, i))] = Some(Piece::new(White, Pawn));
            pieces[rf_to_board((6, i))] = Some(Piece::new(Black, Pawn));
        }

        // king and queen
        pieces[rf_to_board((0, 3))] = Some(Piece::new(White, Queen));
        pieces[rf_to_board((7, 3))] = Some(Piece::new(Black, Queen));
        pieces[rf_to_board((0, 4))] = Some(Piece::new(White, King));
        pieces[rf_to_board((7, 4))] = Some(Piece::new(Black, King));

        // other pieces
        let a = [Rook, Knight, Bishop];
        for i in 0..3 {
            pieces[rf_to_board((0, i))] = Some(Piece::new(White, a[i as usize]));
            pieces[rf_to_board((7, i))] = Some(Piece::new(Black, a[i as usize]));
        }

        Board { pieces, white_castle: CastleInfo::new(), black_castle: CastleInfo::new(), turn: White }
    }

    pub fn get_legal_moves(&self, colour: Colour) -> Vec<Move> {
        let mut out: Vec<Move> = Vec::new();

        for (pos, maybe_piece) in self.pieces.iter().enumerate() {
            if let Some(piece) = maybe_piece && piece.colour == colour {
                match piece.class {
                    Rook => {
                        self.get_sliding_moves(pos, Rook, piece.colour);
                    },
                    Bishop => {
                        self.get_sliding_moves(pos, Bishop, piece.colour);
                    },
                    Queen =>  {
                        self.get_sliding_moves(pos, Queen, piece.colour);
                    },
                    Knight =>  {
                        self.get_knight_moves(pos, piece.colour);
                    },
                    King => {
                        self.get_king_moves(pos, piece.colour);
                    },
                    Pawn => {
                        self.get_pawn_moves(pos, piece.colour);
                    }
                }
            }
        }

        out
    }

    pub fn piece_at(&self, pos: usize) -> Option<Piece> {
        self.pieces[pos]
    }

    // TODO
    pub fn is_check_after(&self, moove: Move, colour: Colour) -> bool { false }

    fn get_sliding_moves(&self, pos: usize, class: PieceType, colour: Colour) -> Vec<Move> {
        let mut moves: Vec<usize> = Vec::new();

        let directions = [-1, 1, 8, -8, -9, -7, 9, 7];
        let start = if class == Bishop {4} else {0};
        let end = if class == Rook {4} else {8};

        let bounds_index = {
            let mut out = pos;

            // reflect over y axis
            if out % 8 > 3 {
                out = 8 * (out / 8) + 7 - (out % 8);
            }

            //reflect over x axis
            if out / 8  > 3{
                out = (out % 8) + 8 * (7 - (out / 8));
            }

            out
        };

        for direction in &directions[start..end] {
            let p = pos as i8;
            while false {

            }
        }

        moves.retain(
            |x| !self.is_check_after(Move{start: pos, end: *x}, colour));
        moves.iter().map(|x| Move {start: pos, end: *x}).collect()
    }

    fn get_king_moves(&self, pos: usize, colour: Colour) -> Vec<Move> {
        Vec::new()
    }

    fn get_knight_moves(&self, pos: usize, colour: Colour) -> Vec<Move> {
        Vec::new()
    }

    fn get_pawn_moves(&self, pos: usize, colour: Colour) -> Vec<Move> {
        Vec::new()
    }
}