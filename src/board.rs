use crate::util::{*, Colour::*, PieceType::*};

#[derive(Clone, Copy)]
pub struct Piece {
    pub colour: Colour,
    pub piece_type: PieceType,
    pub can_en_passant: bool
}

impl Piece {
    pub fn new(colour: Colour, piece_type: PieceType) -> Piece {
        Piece { colour, piece_type, can_en_passant: false }
    }
}

struct Board {
    pieces: [Option<Piece>; 64],
    white_castle: bool,
    black_castle: bool,
    turn: Colour
}

impl Board {
    pub fn starting_position() -> Board {
        let mut pieces = [None; 64];

        // pawns
        for i in 0..8 {
            pieces[rankfile_to_board((1, i))] = Some(Piece::new(White, Pawn));
            pieces[rankfile_to_board((6, i))] = Some(Piece::new(Black, Pawn));
        }

        // king and queen
        pieces[rankfile_to_board((0, 3))] = Some(Piece::new(White, Queen));
        pieces[rankfile_to_board((7, 3))] = Some(Piece::new(Black, Queen));
        pieces[rankfile_to_board((0, 4))] = Some(Piece::new(White, King));
        pieces[rankfile_to_board((7, 4))] = Some(Piece::new(Black, King));

        // other pieces
        let a = [Rook, Knight, Bishop];
        for i in 0..3 {
            pieces[rankfile_to_board((0, i))] = Some(Piece::new(White, a[i as usize]));
            pieces[rankfile_to_board((7, i))] = Some(Piece::new(Black, a[i as usize]));
        }

        Board { pieces, white_castle: true, black_castle: true, turn: White }
    }

    pub fn get_legal_moves(&self, colour: Colour) -> Vec<Move> {
        let mut out: Vec<Move> = Vec::new();

        for maybe_piece in &self.pieces {
            if let Some(piece) = maybe_piece {
                match piece.piece_type {
                    Rook => {
                        self.get_rook_moves(piece, &mut out);
                    },
                    Bishop => {
                        self.get_bishop_moves(piece, &mut out);
                    },
                    Queen =>  {
                        self.get_rook_moves(piece, &mut out);
                        self.get_bishop_moves(piece, &mut out);
                    },
                    Knight =>  {
                        self.get_knight_moves(piece, &mut out);
                    },
                    King => {
                        self.get_king_moves(piece, &mut out);
                    },
                    Pawn => {
                        self.get_pawn_moves(piece, &mut out);
                    }
                }
            }
        }

        out
    }

    pub fn get_piece_at(&self, pos: (u8, u8)) -> Option<Piece> {
        self.pieces[rankfile_to_board(pos) as usize]
    }

    fn get_rook_moves(&self, piece: &Piece, out: &mut Vec<Move>) {

    }

    fn get_bishop_moves(&self, piece: &Piece, out: &mut Vec<Move>) {
        
    }

    fn get_king_moves(&self, piece: &Piece, out: &mut Vec<Move>) {

    }

    fn get_knight_moves(&self, piece: &Piece, out: &mut Vec<Move>) {

    }

    fn get_pawn_moves(&self, piece: &Piece, out: &mut Vec<Move>) {

    }
}