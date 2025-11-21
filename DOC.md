# Structs
- **Board**
  - Represents a single game of chess
- **Piece**
  - A single piece on the board (including pawns)
  - Stores a `PieceType` and a `Pos`
  - Derives `Clone` and `Copy`
- **Pos**
  - A pair of numbers, representing either a position on the board, or a change in position.
  - Implements `Add<Pos>`, `Sub<Pos>`, `Mul<i8>`, along with their `Assign` variants
  - Derives `Clone`, `Copy`, `PartialEq`, and `Eq`.

# Enums
- **Colour**
  - Black or White.
  - Derives `Clone`, `Copy`, `PartialEq` and `Eq`.
- **PieceType**
  - King, Queen, Rook, Bishop, Knight, or Pawn.
  - Derives `Clone`, `Copy`, `PartialEq` and `Eq`.
- **MoveError**
  - Indicates the reason a move failed.
  - OutOfBoard, NoPiece, WrongColour, or NotLegal.
  - Derives `Clone`, `Copy`, `PartialEq` and `Eq`.