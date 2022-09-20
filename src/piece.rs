use std::collections::HashMap;
use crate::colors::*;
use crate::piece_types::*;
use crate::size::*;


#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Piece {
    pub color: Colors,
    pub piece_type: PieceTypes,
    pub notation: String,
    pub emoji: String,
}

impl Piece {
    pub fn new(piece_type:PieceTypes, color:Colors) -> Piece {
        Piece {
            color,
            piece_type,
            notation: Self::get_notation_string(&piece_type.clone()),
            emoji: Self::get_emoji_string(&piece_type.clone(), &color.clone())
        }
    }
    // TODO: Very repetitive code, automate later
    fn get_piece_to_notation_hashmap() -> HashMap<PieceTypes, String> {
        let mut map: HashMap<PieceTypes, String> = HashMap::new();
        map.insert(PieceTypes::King, "K ".to_string());
        map.insert(PieceTypes::Rook, "R ".to_string());
        map.insert(PieceTypes::Bishop, "B ".to_string());
        map.insert(PieceTypes::Queen, "Q ".to_string());
        map.insert(PieceTypes::Knight, "Kn".to_string());
        map.insert(PieceTypes::Pawn, "P ".to_string());

        map
    }

    fn get_notation_string(piece: &PieceTypes) -> String {
        (&Self::get_piece_to_notation_hashmap()
            .get(&piece)
            .unwrap()).to_string()
    }

    // TODO: Very repetitive code, automate later
    fn get_white_emoji_hashmap() -> HashMap<PieceTypes, String> {
        let mut map: HashMap<PieceTypes, String> = HashMap::new();
        map.insert(PieceTypes::King, "♔ ".to_string());
        map.insert(PieceTypes::Rook, "♖ ".to_string());
        map.insert(PieceTypes::Bishop, "♗ ".to_string());
        map.insert(PieceTypes::Queen, "♕ ".to_string());
        map.insert(PieceTypes::Knight, "♘ ".to_string());
        map.insert(PieceTypes::Pawn, "♙ ".to_string());

        map
    }

    // TODO: Very repetitive code, automate later
    fn get_black_emoji_hashmap() -> HashMap<PieceTypes, String> {
        let mut map: HashMap<PieceTypes, String> = HashMap::new();
        map.insert(PieceTypes::King, "♚ ".to_string());
        map.insert(PieceTypes::Rook, "♜ ".to_string());
        map.insert(PieceTypes::Bishop, "♝ ".to_string());
        map.insert(PieceTypes::Queen, "♛ ".to_string());
        map.insert(PieceTypes::Knight, "♞ ".to_string());
        map.insert(PieceTypes::Pawn, "♟ ".to_string());

        map
    }

    // TODO: Should be fixed because of underlying functions duplication
    fn get_emoji_string(piece: &PieceTypes, color: &Colors) -> String {
        match color {
            Colors::White => {
                (&Self::get_white_emoji_hashmap()
                    .get(&piece)
                    .unwrap()).to_string()
            }
            Colors::Black => {
                (&Self::get_black_emoji_hashmap()
                    .get(&piece)
                    .unwrap()).to_string()
            }
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn create_white_pawn() {
//         let s = Piece{};
//         assert_eq!(s.w, 0);
//         assert_eq!(s.h, 0);
//     }
//
//     #[test]
//     fn assign_ones() {
//         let s = Size{w:1, h:1};
//         assert_eq!(s.w, 1);
//         assert_eq!(s.h, 1);
//     }
//
//     #[test]
//     fn assign_different_values() {
//         let s = Size{w:10, h:100};
//         assert_eq!(s.w, 10);
//         assert_eq!(s.h, 100);
//     }
//
//     #[test]
//     fn assign_u8_limit_values() {
//         let s = Size{w:255, h:255};
//         assert_eq!(s.w, 255);
//         assert_eq!(s.h, 255);
//     }
//
//     //TODO: How to write this type of test
//
//     // #[test]
//     // #[should_panic]
//     // fn assign_larger_value_than_u8() {
//     //     let s = Size{w:1000, h:1000};
//     // }
// }
