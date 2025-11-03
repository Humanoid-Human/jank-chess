use crate::util::{*, Colour::*, PieceType::*, pos::*};

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

    pub fn get_legal_moves(&self, colour: Colour) -> Vec<Move> {
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

        out
    }

    // TODO
    pub fn is_check_after(&self, moove: Move, colour: Colour) -> bool { false }

    fn get_sliding_moves(&self, pos: Pos, class: PieceType, colour: Colour) -> Vec<Move> {
        let mut moves: Vec<Pos> = Vec::new();

        const DIR: [Pos; 8] = [
            Pos{row: 0, col: 1},
            Pos{row: 0, col: -1},
            Pos{row: 1, col: 0},
            Pos{row: -1, col: 0},
            Pos{row: 1, col: 1},
            Pos{row: 1, col: -1},
            Pos{row: -1, col: 1},
            Pos{row: -1, col: -1}
        ];

        let start = if class == Bishop {4} else {0};
        let end = if class == Rook {4} else {8};

        for dir in &DIR[start..end] {
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

        let mut out = moves.iter()
            .map(|x| Move {start: pos, end: *x})
            .collect::<Vec<Move>>();

        out.retain(|x| self.is_check_after(*x, colour));
        out
    }

    fn get_king_moves(&self, pos: Pos, colour: Colour) -> Vec<Move> {
        Vec::new()
    }

    fn get_knight_moves(&self, pos: Pos, colour: Colour) -> Vec<Move> {
        Vec::new()
    }

    fn get_pawn_moves(&self, pos: Pos, colour: Colour) -> Vec<Move> {
        Vec::new()
    }

    pub fn get_piece(&self, pos: Pos) -> Option<Piece> {
        self.pieces[(8 * pos.row + pos.col) as usize]
    }

    pub fn set_piece(&mut self, pos: Pos, piece: Option<Piece>) {
        self.pieces[(8 * pos.row + pos.col) as usize] = piece;
    }
}