use std::collections::HashMap;
use crate::colors::*;
use crate::piece_types::*;
use crate::coords::*;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Piece {
    pub piece_type: PieceTypes,
    pub color: Colors,
    pub notation: String,
    pub emoji: String,
    pub coords: Coords,
}

impl Piece {
    pub fn new(piece_type:PieceTypes, color:Colors, coords:Coords) -> Piece {
        Piece {
            piece_type,
            color,
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

    pub fn get_coords_from_board(board: Vec<Vec<Option<Piece>>>, color: Colors) -> Vec<Coords> {
        let mut coords = vec![];
        for row in board {
            for square in row {
                if square.is_some() {
                    if square.as_ref().unwrap().color == color {
                        coords.push(square.unwrap().coords)
                    }
                }
            }

        }
        coords
    }

    pub fn get_all_legal_moves(&self, board: Vec<Vec<Option<Piece>>>) -> Vec<Coords> {
        let mut legal_moves = vec![];

        legal_moves.extend(self.get_moves(board.clone()));
        legal_moves.extend(self.get_special_moves(board.clone()));
        legal_moves.extend(self.get_attacked_squares(board.clone()));
        legal_moves.extend(self.get_attacked_pieces_squares(board.clone()));

        legal_moves
    }

    pub fn get_moves(&self, board: Vec<Vec<Option<Piece>>>) -> Vec<Coords> {
        let mut white_pieces = Self::get_coords_from_board(board.clone(),
                                                           Colors::White);
        let mut black_pieces = Self::get_coords_from_board(board.clone(),
                                                           Colors::Black);
        let mut legal_moves = vec![];

        // println!("{:?}", white_pieces);
        // println!("{:?}", black_pieces);

        match self.piece_type {
            PieceTypes::Pawn => {
                legal_moves.extend(self.get_pawn_moves(
                    white_pieces.clone(),
                    black_pieces.clone()
                ));
            }
            PieceTypes::Knight => {
                legal_moves.extend(self.get_knight_moves(
                    white_pieces.clone(),
                    black_pieces.clone()
                ));
            }
            PieceTypes::Bishop => {

            }
            PieceTypes::Rook => {

            }
            PieceTypes::Queen => {

            }
            PieceTypes::King => {

            }
            // _ => {panic!("Not implemented PieceType!")}
        }


        legal_moves
    }

    pub fn get_special_moves(&self, board: Vec<Vec<Option<Piece>>>) -> Vec<Coords> {
        let mut white_pieces = Self::get_coords_from_board(board.clone(),
                                                           Colors::White);
        let mut black_pieces = Self::get_coords_from_board(board.clone(),
                                                           Colors::Black);
        let mut legal_moves = vec![];

        // println!("{:?}", white_pieces);
        // println!("{:?}", black_pieces);

        match self.piece_type {
            PieceTypes::Pawn => {
                legal_moves.extend(self.get_pawn_special_moves(
                    white_pieces.clone(),
                    black_pieces.clone()
                ));
            }
            PieceTypes::Knight => {}
            PieceTypes::Bishop => {}
            PieceTypes::Rook => {

            }
            PieceTypes::Queen => {}
            PieceTypes::King => {

            }
            // _ => {panic!("Not implemented PieceType!")}
        }

        legal_moves
    }

    pub fn get_attacked_squares(&self, board: Vec<Vec<Option<Piece>>>) -> Vec<Coords> {
        let mut white_pieces = Self::get_coords_from_board(board.clone(),
                                                           Colors::White);
        let mut black_pieces = Self::get_coords_from_board(board.clone(),
                                                           Colors::Black);
        let mut legal_moves = vec![];

        // println!("{:?}", white_pieces);
        // println!("{:?}", black_pieces);

        match self.piece_type {
            PieceTypes::Pawn => {
                legal_moves.extend(self.get_pawn_attacked_squares(
                    white_pieces.clone(),
                    black_pieces.clone()
                ));
            }
            PieceTypes::Knight => {
                legal_moves.extend(self.get_knight_moves(
                    white_pieces.clone(),
                    black_pieces.clone()
                ));
            }
            PieceTypes::Bishop => {

            }
            PieceTypes::Rook => {

            }
            PieceTypes::Queen => {

            }
            PieceTypes::King => {

            }
            // _ => {panic!("Not implemented PieceType!")}
        }


        legal_moves
    }

    pub fn get_attacked_pieces_squares(&self, board: Vec<Vec<Option<Piece>>>) -> Vec<Coords> {
        let mut white_pieces = Self::get_coords_from_board(board.clone(),
                                                           Colors::White);
        let mut black_pieces = Self::get_coords_from_board(board.clone(),
                                                           Colors::Black);
        let mut legal_moves = vec![];

        // println!("{:?}", white_pieces);
        // println!("{:?}", black_pieces);

        match self.piece_type {
            PieceTypes::Pawn => {
                legal_moves.extend(self.get_pawn_attacked_pieces_squares(
                    white_pieces.clone(),
                    black_pieces.clone()
                ));
            }
            PieceTypes::Knight => {
                legal_moves.extend(self.get_knight_attacked_pieces_squares(
                    white_pieces.clone(),
                    black_pieces.clone()
                ));
            }
            PieceTypes::Bishop => {

            }
            PieceTypes::Rook => {

            }
            PieceTypes::Queen => {

            }
            PieceTypes::King => {

            }
            // _ => {panic!("Not implemented PieceType!")}
        }


        legal_moves
    }

    // Moves by piece type:

    // Knight Moves -------------------------------------------------

    fn get_knight_moves(&self, white_pieces: Vec<Coords>, black_pieces: Vec<Coords>) -> Vec<Coords> {
        let rel_moves = vec![
            RelCoords::new(2, 1),
            RelCoords::new(2, -1),
            RelCoords::new(-2, 1),
            RelCoords::new(-2, -1),
            RelCoords::new(1, 2),
            RelCoords::new(1, -2),
            RelCoords::new(-1, 2),
            RelCoords::new(-1, -2),
        ];

        let mut moves = vec![];

        for rel_move in rel_moves.clone() {
            // println!("{:?}", Coords::coords_and_rel_coords_result(self.coords, rel_move));
            if Coords::check_within_coords(self.coords, rel_move){
                let new_coords = Coords::coords_and_rel_coords_result(self.coords, rel_move);
                if !white_pieces.contains(&new_coords) && !black_pieces.contains(&new_coords){
                    moves.push(new_coords);
                }
            }
        }

        // println!("{:?}", moves);

        moves
    }

    fn get_knight_attacked_pieces_squares(&self, white_pieces: Vec<Coords>, black_pieces: Vec<Coords>) -> Vec<Coords> {
        let oposite_color_pieces;
        match self.color {
            Colors::White => {
                oposite_color_pieces = black_pieces.clone();
            }
            Colors::Black => {
                oposite_color_pieces = white_pieces.clone();
            }
        }

        let rel_moves = vec![
            RelCoords::new(2, 1),
            RelCoords::new(2, -1),
            RelCoords::new(-2, 1),
            RelCoords::new(-2, -1),
            RelCoords::new(1, 2),
            RelCoords::new(1, -2),
            RelCoords::new(-1, 2),
            RelCoords::new(-1, -2),
        ];

        let mut moves = vec![];

        for rel_move in rel_moves.clone() {
            // println!("{:?}", Coords::coords_and_rel_coords_result(self.coords, rel_move));
            if Coords::check_within_coords(self.coords, rel_move){
                let new_coords = Coords::coords_and_rel_coords_result(self.coords, rel_move);

                if oposite_color_pieces.contains(&new_coords){
                    moves.push(new_coords);
                }
            }
        }

        // println!("{:?}", moves);

        moves
    }

    // Pawn Moves -------------------------------------------------

    fn get_pawn_moves(&self, white_pieces: Vec<Coords>, black_pieces: Vec<Coords>) -> Vec<Coords> {
        let rel_moves;
        match self.color {
            Colors::White => {
                rel_moves = vec![
                    RelCoords::new(0, -1),
                ];
            }
            Colors::Black => {
                rel_moves = vec![
                    RelCoords::new(0, 1),
                ];
            }
        }

        let mut moves = vec![];
        for rel_move in rel_moves.clone() {
            // println!("{:?}", Coords::coords_and_rel_coords_result(self.coords, rel_move));
            if Coords::check_within_coords(self.coords, rel_move){
                let new_coords = Coords::coords_and_rel_coords_result(self.coords, rel_move);
                if !white_pieces.contains(&new_coords) && !black_pieces.contains(&new_coords){
                    moves.push(new_coords);
                }
            }
        }

        // println!("{:?}", moves);

        moves
    }

    fn get_pawn_special_moves(&self, white_pieces: Vec<Coords>, black_pieces: Vec<Coords>) -> Vec<Coords> {
        let rel_moves;
        let start_at_y;
        match self.color {
            Colors::White => {
                rel_moves = vec![
                    RelCoords::new(0, -2),
                ];
                start_at_y = 6;
            }
            Colors::Black => {
                rel_moves = vec![
                    RelCoords::new(0, 2),
                ];
                start_at_y = 1;
            }
        }

        let mut moves = vec![];
        if self.coords.y == start_at_y {
            for rel_move in rel_moves.clone() {
                // println!("{:?}", Coords::coords_and_rel_coords_result(self.coords, rel_move));
                if Coords::check_within_coords(self.coords, rel_move){
                    let new_coords = Coords::coords_and_rel_coords_result(self.coords, rel_move);
                    if !white_pieces.contains(&new_coords) && !black_pieces.contains(&new_coords){
                        moves.push(new_coords);
                    }
                }
            }
        }

        // println!("{:?}", moves);

        moves
    }

    fn get_pawn_attacked_squares(&self, white_pieces: Vec<Coords>, black_pieces: Vec<Coords>) -> Vec<Coords> {
        let rel_moves;
        match self.color {
            Colors::White => {
                rel_moves = vec![
                    RelCoords::new(-1, -1),
                    RelCoords::new(1, -1),
                ];
            }
            Colors::Black => {
                rel_moves = vec![
                    RelCoords::new(-1, 1),
                    RelCoords::new(1, 1),
                ];
            }
        }

        let mut moves = vec![];
        for rel_move in rel_moves.clone() {
            // println!("{:?}", Coords::coords_and_rel_coords_result(self.coords, rel_move));
            if Coords::check_within_coords(self.coords, rel_move){
                let new_coords = Coords::coords_and_rel_coords_result(self.coords, rel_move);
                if !white_pieces.contains(&new_coords) && !black_pieces.contains(&new_coords){
                    moves.push(new_coords);
                }
            }
        }

        moves
    }

    fn get_pawn_attacked_pieces_squares(&self, white_pieces: Vec<Coords>, black_pieces: Vec<Coords>) -> Vec<Coords> {
        let oposite_color_pieces;
        let rel_moves;
        match self.color {
            Colors::White => {
                rel_moves = vec![
                    RelCoords::new(-1, -1),
                    RelCoords::new(1, -1),
                ];
                oposite_color_pieces = black_pieces.clone();
            }
            Colors::Black => {
                rel_moves = vec![
                    RelCoords::new(-1, 1),
                    RelCoords::new(1, 1),
                ];
                oposite_color_pieces = white_pieces.clone();
            }
        }

        let mut moves = vec![];

        for rel_move in rel_moves.clone() {
            // println!("{:?}", Coords::coords_and_rel_coords_result(self.coords, rel_move));
            if Coords::check_within_coords(self.coords, rel_move){
                let new_coords = Coords::coords_and_rel_coords_result(self.coords, rel_move);
                if oposite_color_pieces.contains(&new_coords){
                    moves.push(new_coords);
                }
            }
        }

        // println!("{:?}", moves);

        moves
    }
}

#[cfg(test)]
mod tests {
    use crate::chess_engine::ChessEngine;
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

    #[test]
    #[should_panic]
    fn assign_larger_value_than_limit() {
        Piece::new(PieceTypes::King,
                   Colors::Black,
                   Coords::new(8, 8));
    }

    #[test]
    fn test_get_coords_from_board_white() {
        let mut chess_engine = ChessEngine::new();
        let white_pieces = Piece::get_coords_from_board(chess_engine.board,
                                                        Colors::White);

        println!("{:?}", white_pieces);
    }

    #[test]
    fn test_get_coords_from_board_one_piece() {
        let mut chess_engine = ChessEngine::create_engine_with_empty_board();

        let square = "d5";

        chess_engine.add_piece_with_notation(square, Some(Piece::new(
            PieceTypes::Knight,
            Colors::White,
            chess_engine.get_coords_from_notation(square))
        ));

        let white_pieces = Piece::get_coords_from_board(chess_engine.board,
                                                        Colors::White);

        assert_eq!(white_pieces, vec![Coords::new(3, 3)]);
    }

    // Testing piece movement

    // Knight

    #[test]
    fn test_get_all_legal_moves_knight_on_g2_with_other_pieces() {
        let mut chess_engine = ChessEngine::create_engine_with_empty_board();

        let square = "e3";
        chess_engine.add_piece_with_notation(square, Some(Piece::new(
            PieceTypes::Pawn,
            Colors::White,
            chess_engine.get_coords_from_notation(square))
        ));

        let square = "f4";
        chess_engine.add_piece_with_notation(square, Some(Piece::new(
            PieceTypes::Pawn,
            Colors::Black,
            chess_engine.get_coords_from_notation(square))
        ));

        let square = "g2";
        chess_engine.add_piece_with_notation(square, Some(Piece::new(
            PieceTypes::Knight,
            Colors::White,
            chess_engine.get_coords_from_notation(square))
        ));

        let pieces = chess_engine
            .get_piece_option_with_notation(square)
            .unwrap()
            .get_all_legal_moves(chess_engine.board.clone());

        chess_engine.print_board_with_ranks_and_files_with_all_legal_moves();

        chess_engine.force_move_piece_with_notation("g2", "e1");

        chess_engine.print_board_with_ranks_and_files_with_all_legal_moves();
    }

    #[test]
    fn test_get_all_legal_moves_knight_on_g2_with_other_pieces_colored() {
        let mut chess_engine = ChessEngine::create_engine_with_empty_board();

        let square = "e3";
        chess_engine.add_piece_with_notation(square, Some(Piece::new(
            PieceTypes::Pawn,
            Colors::White,
            chess_engine.get_coords_from_notation(square))
        ));

        let square = "f4";
        chess_engine.add_piece_with_notation(square, Some(Piece::new(
            PieceTypes::Pawn,
            Colors::Black,
            chess_engine.get_coords_from_notation(square))
        ));

        let square = "g2";
        chess_engine.add_piece_with_notation(square, Some(Piece::new(
            PieceTypes::Knight,
            Colors::White,
            chess_engine.get_coords_from_notation(square))
        ));

        let pieces = chess_engine
            .get_piece_option_with_notation(square)
            .unwrap()
            .get_all_legal_moves(chess_engine.board.clone());

        chess_engine.print_board_with_ranks_and_files_with_all_legal_moves_different_colors();

        chess_engine.force_move_piece_with_notation("g2", "e1");

        chess_engine.print_board_with_ranks_and_files_with_all_legal_moves_different_colors();
    }

    #[test]
    fn test_get_moves_alone_knight_on_d5() {
        let mut chess_engine = ChessEngine::create_engine_with_empty_board();

        let square = "d5";

        chess_engine.add_piece_with_notation(square, Some(Piece::new(
            PieceTypes::Knight,
            Colors::White,
            chess_engine.get_coords_from_notation(square))
        ));

        // let square = "e5";
        //
        // chess_engine.add_piece_with_notation(square, Some(Piece::new(
        //     PieceTypes::Knight,
        //     Colors::White,
        //     chess_engine.get_coords_from_notation(square))
        // ));

        let pieces = chess_engine
            .get_piece_option_with_notation(square)
            .unwrap()
            .get_moves(chess_engine.board.clone());

        // println!("{:?}", pieces);
        chess_engine.print_board_with_ranks_and_files_with_moves();
    }

    #[test]
    fn test_get_moves_knight_on_g2_with_other_pieces() {
        let mut chess_engine = ChessEngine::create_engine_with_empty_board();

        let square = "e3";
        chess_engine.add_piece_with_notation(square, Some(Piece::new(
            PieceTypes::Pawn,
            Colors::White,
            chess_engine.get_coords_from_notation(square))
        ));

        let square = "f4";
        chess_engine.add_piece_with_notation(square, Some(Piece::new(
            PieceTypes::Pawn,
            Colors::Black,
            chess_engine.get_coords_from_notation(square))
        ));

        let square = "g2";
        chess_engine.add_piece_with_notation(square, Some(Piece::new(
            PieceTypes::Knight,
            Colors::White,
            chess_engine.get_coords_from_notation(square))
        ));

        let pieces = chess_engine
            .get_piece_option_with_notation(square)
            .unwrap()
            .get_moves(chess_engine.board.clone());

        chess_engine.print_board_with_ranks_and_files_with_moves();

        chess_engine.force_move_piece_with_notation("g2", "e1");

        chess_engine.print_board_with_ranks_and_files_with_moves();
    }

    #[test]
    fn test_get_special_moves_knight_on_g2_with_other_pieces() {
        let mut chess_engine = ChessEngine::create_engine_with_empty_board();

        let square = "e3";
        chess_engine.add_piece_with_notation(square, Some(Piece::new(
            PieceTypes::Pawn,
            Colors::White,
            chess_engine.get_coords_from_notation(square))
        ));

        let square = "f4";
        chess_engine.add_piece_with_notation(square, Some(Piece::new(
            PieceTypes::Pawn,
            Colors::Black,
            chess_engine.get_coords_from_notation(square))
        ));

        let square = "g2";
        chess_engine.add_piece_with_notation(square, Some(Piece::new(
            PieceTypes::Knight,
            Colors::White,
            chess_engine.get_coords_from_notation(square))
        ));

        let pieces = chess_engine
            .get_piece_option_with_notation(square)
            .unwrap()
            .get_special_moves(chess_engine.board.clone());

        chess_engine.print_board_with_ranks_and_files_with_special_moves();

        chess_engine.force_move_piece_with_notation("g2", "e1");

        chess_engine.print_board_with_ranks_and_files_with_special_moves();
    }

    #[test]
    fn test_get_attacked_squares_knight_on_g2_with_other_pieces() {
        let mut chess_engine = ChessEngine::create_engine_with_empty_board();

        let square = "e3";
        chess_engine.add_piece_with_notation(square, Some(Piece::new(
            PieceTypes::Pawn,
            Colors::White,
            chess_engine.get_coords_from_notation(square))
        ));

        let square = "f4";
        chess_engine.add_piece_with_notation(square, Some(Piece::new(
            PieceTypes::Pawn,
            Colors::Black,
            chess_engine.get_coords_from_notation(square))
        ));

        let square = "g2";
        chess_engine.add_piece_with_notation(square, Some(Piece::new(
            PieceTypes::Knight,
            Colors::White,
            chess_engine.get_coords_from_notation(square))
        ));

        let pieces = chess_engine
            .get_piece_option_with_notation(square)
            .unwrap()
            .get_attacked_squares(chess_engine.board.clone());

        chess_engine.print_board_with_ranks_and_files_with_attacked_squares();

        chess_engine.force_move_piece_with_notation("g2", "e1");

        chess_engine.print_board_with_ranks_and_files_with_attacked_squares();
    }

    #[test]
    fn test_get_attacked_pieces_squares_knight_on_g2_with_other_pieces() {
        let mut chess_engine = ChessEngine::create_engine_with_empty_board();

        let square = "e3";
        chess_engine.add_piece_with_notation(square, Some(Piece::new(
            PieceTypes::Pawn,
            Colors::White,
            chess_engine.get_coords_from_notation(square))
        ));

        let square = "f4";
        chess_engine.add_piece_with_notation(square, Some(Piece::new(
            PieceTypes::Pawn,
            Colors::Black,
            chess_engine.get_coords_from_notation(square))
        ));

        let square = "g2";
        chess_engine.add_piece_with_notation(square, Some(Piece::new(
            PieceTypes::Knight,
            Colors::White,
            chess_engine.get_coords_from_notation(square))
        ));

        let pieces = chess_engine
            .get_piece_option_with_notation(square)
            .unwrap()
            .get_attacked_pieces_squares(chess_engine.board.clone());

        chess_engine.print_board_with_ranks_and_files_with_attacked_pieces_squares();

        chess_engine.force_move_piece_with_notation("g2", "e1");

        chess_engine.print_board_with_ranks_and_files_with_attacked_pieces_squares();
    }
}
