use crate::util::{*, Colour::*, piece::{*, PieceType::*}, pos::*, constants::*};

struct Board {
    pieces: [Option<Piece>; 64],
    white_castle: CastleInfo,
    black_castle: CastleInfo,
    turn: Colour
}

impl Board {
    pub fn get_piece(&self, pos: Pos) -> Option<Piece> {
        self.pieces[(8 * pos.row + pos.col) as usize]
    }

    pub fn set_piece(&mut self, pos: Pos, piece: Option<Piece>) {
        self.pieces[(8 * pos.row + pos.col) as usize] = piece;
    }

    pub fn is_at(&self, pos: Pos, colour: Colour, class: PieceType) -> bool {
        if let Some(p) = self.get_piece(pos) && p.is(colour, class) {
            return true;
        }
        false
    }

    pub fn starting_position() -> Board {
        let pieces = [None; 64];
        let mut board = Board {
            pieces,
            white_castle: CastleInfo::new(),
            black_castle: CastleInfo::new(),
            turn: White
        };

        // pawns
        for i in 0..8 {
            board.set_piece(Pos::new(1, i), Piece::some(White, Pawn));
            board.set_piece(Pos::new(6, i), Piece::some(Black, Pawn));
        }

        // king and queen
        board.set_piece(Pos::new(0, 3), Piece::some(White, Queen));
        board.set_piece(Pos::new(7, 3), Piece::some(Black, Queen));
        board.set_piece(Pos::new(0, 4), Piece::some(White, King));
        board.set_piece(Pos::new(7, 4), Piece::some(Black, King));

        // other pieces
        let a = [Rook, Knight, Bishop];
        for i in 0..3 {
            let class = a[i as usize];
            board.set_piece(Pos::new(0, i), Piece::some(White, class));
            board.set_piece(Pos::new(7, i), Piece::some(Black, class));
            board.set_piece(Pos::new(0, 7-i), Piece::some(White, class));
            board.set_piece(Pos::new(7, 7-i), Piece::some(Black, class));
        }

        board
    }

    // TODO: castling
    pub fn get_legal_moves(&mut self, colour: Colour) -> Vec<Move> {
        let mut out: Vec<Move> = Vec::new();

        for (i, maybe_piece) in self.pieces.iter().enumerate() {
            if let Some(piece) = maybe_piece && piece.colour == colour {
                let pos = Pos::from(i);
                let mut moves = match piece.class {
                    Rook | Bishop | Queen => self.get_sliding_moves(pos, piece.class, piece.colour),
                    Knight => self.get_knight_moves(pos, piece.colour),
                    King => self.get_king_moves(pos, piece.colour),
                    Pawn => self.get_pawn_moves(pos, piece.colour)
                };
                out.append(&mut moves);
            }
        }

        out.retain(|x| !self.is_check_after(*x, colour));
        out
    }

    fn get_sliding_moves(&self, pos: Pos, class: PieceType, colour: Colour) -> Vec<Move> {
        let mut moves: Vec<Pos> = Vec::new();

        let start = if class == Bishop {4} else {0};
        let end = if class == Rook {4} else {8};

        for dir in &CARDINALS[start..end] {
            let mut p = pos + *dir;
            while p.is_on_board() {
                if let Some(piece) = self.get_piece(p) {
                    if piece.colour != colour {
                        moves.push(p);
                    }
                    break;
                }
                moves.push(p);
                p += *dir;
            }
        }

        moves.iter().map(|x| Move {start: pos, end: *x}).collect()
    }

    fn get_king_moves(&self, pos: Pos, colour: Colour) -> Vec<Move> {
        let mut out = Vec::new();

        for mov in &CARDINALS {
            let end = pos + *mov;
            if let Some(piece) = self.get_piece(end) && piece.colour == colour {
                continue;
            }
            out.push(Move{start: pos, end: pos + *mov});
        }

        out
    }

    fn get_knight_moves(&self, pos: Pos, colour: Colour) -> Vec<Move> {
        let mut out = Vec::new();

        for mov in &KNIGHT_MOVES {
            let end = pos + *mov;
            if let Some(piece) = self.get_piece(end) && piece.colour == colour {
                continue;
            }
            out.push(Move{start: pos, end: pos + *mov});
        }

        out
    }

    // TODO
    fn get_pawn_moves(&self, pos: Pos, colour: Colour) -> Vec<Move> {
        Vec::new()
    }

    pub fn make_move(&mut self, mov: Move) {
        let maybe_piece = self.get_piece(mov.start);
        if let Some(piece) = maybe_piece {
            if piece.class == King {
                self.get_castle_info(piece.colour).king_moved();
            } else if piece.class == Rook {
                let ci = self.get_castle_info(piece.colour);
                if mov.start.col == 0 || mov.start.col == 7 {
                    match mov.start.row {
                        0 => ci.queenside_rook(),
                        7 => ci.kingside_rook(),
                        _ => ()
                    }
                }
            }

            self.set_piece(mov.start, None);
            self.set_piece(mov.end, Some(piece));
        } 
    }

    pub fn is_check_after(&mut self, mov: Move, colour: Colour) -> bool { 
        let start = self.get_piece(mov.start);
        let end = self.get_piece(mov.end);
        self.make_move(mov);
        let out = self.is_in_check(colour);
        self.set_piece(mov.start, start);
        self.set_piece(mov.end, end);
        return out;
    }

    pub fn is_in_check(&self, colour: Colour) -> bool {
        let mut kingpos = Pos::new(-1, -1);
        for (index, maybe_piece) in self.pieces.iter().enumerate() {
            if let Some(piece) = maybe_piece && piece.is(Black, King) {
                kingpos = Pos::from(index);
                break;
            }
        }

        // no king found
        if !kingpos.is_on_board() {
            return false;
        }

        for dir in &STRAIGHTS {
            let mut p = kingpos + *dir;
            while p.is_on_board() {
                if let Some(piece) = self.get_piece(p) {
                    if piece.colour == colour {
                        break;
                    } else if piece.class == Queen || piece.class == Rook {
                        return true;
                    }
                }
                p += *dir;
            }
        }

        for dir in &DIAGS {
            let mut p = kingpos + *dir;
            if dir.row == colour.pawn_dir() && self.is_at(p, colour.opposite(), Pawn) {
                return true;
            }
            while p.is_on_board() {
                let maybe_piece = self.get_piece(p);
                if let Some(piece) = maybe_piece {
                    if piece.colour == colour {
                        break;
                    } else if piece.class == Queen || piece.class == Bishop {
                        return true;
                    }
                }
                p += *dir;
            }
        }

        for d in KNIGHT_MOVES {
            if self.is_at(kingpos + d, colour.opposite(), Knight) {
                return true;
            }
        }

        false
    }

    fn get_castle_info(&mut self, colour: Colour) -> &mut CastleInfo {
        match colour {
            White => &mut self.white_castle,
            Black => &mut self.black_castle
        }
    }
}