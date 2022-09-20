#[derive(Debug, PartialEq, Eq)]
pub struct Size {
    pub w:u8,
    pub h:u8,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Piece {
    pub color: Color,
    pub piece_type: PieceTypes,
    pub notation: String,
    pub emoji: String,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Color{
    White,
    Black
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum PieceTypes{
    King,
    Rook,
    Bishop,
    Queen,
    Knight,
    Pawn,
}