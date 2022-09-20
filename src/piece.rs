use crate::colors::*;
use crate::piece_types::*;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Piece {
    pub color: Colors,
    pub piece_type: PieceTypes,
    pub notation: String,
    pub emoji: String,
}
