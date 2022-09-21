use std::collections::HashMap;
use crate::colors::*;
use crate::piece_types::*;
use crate::coords::*;


#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Piece {
    pub color: Colors,
    pub piece_type: PieceTypes,
    pub notation: String,
    pub emoji: String,
    pub coords: Coords,
}

impl Piece {
    pub fn new(piece_type:PieceTypes, color:Colors, coords:Coords) -> Piece {
        Piece {
            color,
            piece_type,
            notation: Self::get_notation_string(&piece_type.clone()),
            emoji: Self::get_emoji_string(&piece_type.clone(), &color.clone()),
            coords,
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

    pub fn set_coords(&mut self, coords: Coords){
        self.coords = coords;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_white_pawn() {
        let p = Piece::new(PieceTypes::Pawn,
                           Colors::White,
                           Coords::new(0, 6));
        assert_eq!(p.piece_type, PieceTypes::Pawn);
        assert_eq!(p.color, Colors::White);
        assert_eq!(p.notation, "P ".to_string());
        assert_eq!(p.emoji, "♙ ".to_string());
        assert_eq!(p.coords.x, 0);
        assert_eq!(p.coords.y, 6);
    }

    #[test]
    fn create_black_king() {
        let p = Piece::new(PieceTypes::King,
                           Colors::Black,
                           Coords::new(4, 0));
        assert_eq!(p.piece_type, PieceTypes::King);
        assert_eq!(p.color, Colors::Black);
        assert_eq!(p.notation, "K ".to_string());
        assert_eq!(p.emoji, "♚ ".to_string());
        assert_eq!(p.coords.x, 4);
        assert_eq!(p.coords.y, 0);
    }

    //TODO: How to write this type of test

    // #[test]
    // #[should_panic]
    // fn assign_larger_value_than_u8() {
    //     let s = Size{w:1000, h:1000};
    // }
}
