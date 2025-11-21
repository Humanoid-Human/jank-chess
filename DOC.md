# Documentation
## Structs
- `Board`
  - Represents a single game of chess
- `Piece`
  - A single piece on the board (including pawns)
  - Stores a `PieceType` and a `Pos`
  - Derives `Clone` and `Copy`
- `Pos`
  - A pair of numbers, representing either a position on the board, or a change in position.
  - Implements `Add<Pos>`, `Sub<Pos>`, `Mul<i8>`, along with their `Assign` variants
  - Derives `Clone`, `Copy`, `PartialEq`, and `Eq`.

## Enums
All of the enums derive `Clone`, `Copy`, `PartialEq` and `Eq`.
- `Colour`
  - Referring to one of the sides. 
  - Variants: `Black`, `White`.
- `PieceType`
  - The type of chess piece a piece is.
  - Variants: `King`, `Queen`, `Rook`, `Bishop`, `Knight`, `Pawn`.
- `MoveError`
  - Indicates the reason a move failed.
  - Variants: `OutOfBoard`, `NoPiece`, `WrongColour`, `NotLegal`.
