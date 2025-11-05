use super::pos::Pos;

pub const CARDINALS: [Pos; 8] = [
    Pos{row: 0, col: 1},
    Pos{row: 0, col: -1},
    Pos{row: 1, col: 0},
    Pos{row: -1, col: 0},
    Pos{row: 1, col: 1},
    Pos{row: 1, col: -1},
    Pos{row: -1, col: 1},
    Pos{row: -1, col: -1}
];

pub const STRAIGHTS: [Pos; 4] = [
    Pos{row: 0, col: 1},
    Pos{row: 0, col: -1},
    Pos{row: 1, col: 0},
    Pos{row: -1, col: 0}
];

pub const DIAGS: [Pos; 4] = [
    Pos{row: 1, col: 1},
    Pos{row: 1, col: -1},
    Pos{row: -1, col: 1},
    Pos{row: -1, col: -1}
];

pub const KNIGHT_MOVES: [Pos; 8] = [
    Pos{row: 1, col: 2},
    Pos{row: 1, col: -2},
    Pos{row: -1, col: 2},
    Pos{row: -1, col: -2},
    Pos{row: 2, col: 1},
    Pos{row: 2, col: -1},
    Pos{row: -2, col: 1},
    Pos{row: -2, col: -1}
];

pub const ROOK_POS: [Pos; 4] = [
    Pos{row: 0, col: 0},
    Pos{row: 0, col: 7},
    Pos{row: 7, col: 0},
    Pos{row: 7, col: 7}
];