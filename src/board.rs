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

        Board { pieces, white_castle: CastleInfo::new(), black_castle: CastleInfo::new(), turn: White }
    }

    pub fn get_legal_moves(&self, colour: Colour) -> Vec<Move> {
        let mut out: Vec<Move> = Vec::new();

        for (pos, maybe_piece) in self.pieces.iter().enumerate() {
            if let Some(piece) = maybe_piece && piece.colour == colour {
                match piece.piece {
                    Rook => {
                        self.get_rook_moves(pos, piece.colour);
                    },
                    Bishop => {
                        self.get_bishop_moves(pos, piece.colour);
                    },
                    Queen =>  {
                        self.get_rook_moves(pos, piece.colour);
                        self.get_bishop_moves(pos, piece.colour);
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

    pub fn piece_at(&self, pos: (u8, u8)) -> Option<Piece> {
        self.pieces[rankfile_to_board(pos)]
    }

    // TODO
    pub fn is_check_after(&self, moove: Move, colour: Colour) -> bool { false }

    fn get_rook_moves(&self, pos: usize, colour: Colour) -> Vec<Move> {
        let mut moves: Vec<(u8, u8)> = Vec::new();
        let rfpos = board_to_rankfile(pos);

        let bounds = [|x: u8| x > 0, |x: u8| x < 7];
        let it = [|x: &mut u8| *x -= 1, |x: &mut u8| *x += 1];

        for i in 0..2 {
            let mut phori = rfpos.1;
            while bounds[i](phori) {
                it[i](&mut phori);
                let maybe_piece = self.piece_at((rfpos.0, phori));
                if let Some(piece) = maybe_piece {
                    if piece.colour == colour {
                        break;
                    } else {
                        moves.push((rfpos.0, phori));
                        break;
                    }
                } else {
                    moves.push((rfpos.0, phori));
                }
            }

            let mut pvert = rfpos.0;
            while bounds[i](pvert) {
                it[i](&mut pvert);
                let maybe_piece = self.piece_at((pvert, rfpos.1));
                if let Some(piece) = maybe_piece {
                    if piece.colour == colour {
                        break;
                    } else {
                        moves.push((pvert, rfpos.1));
                        break;
                    }
                } else {
                    moves.push((pvert, rfpos.1));
                }
            }
        }

        moves.retain(
            |x| !self.is_check_after(Move{start: rfpos, end: *x}, colour));
        moves.iter().map(|x| Move {start: rfpos, end: *x}).collect()
    }

    // TODO
    fn get_bishop_moves(&self, pos: usize, colour: Colour) -> Vec<Move> {
        Vec::new()
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