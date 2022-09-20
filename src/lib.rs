mod piece;
use crate::piece::*;

use std::collections::HashMap;
use std::fmt;

struct ChessEngine{
    pub board: Vec<Vec<Option<Piece>>>,
    size: Size,
}

impl ChessEngine {
    pub fn new() -> ChessEngine{
        // Self::create_engine_with_white_board()
        Self::create_engine_with_empty_board()
    }

    // TODO: Non-readable code, needs refactoring later
    fn create_engine_with_white_board() -> ChessEngine {
        let size_temp = Size{w:8, h:8};
        ChessEngine {
            size: Size {w: *&size_temp.w, h:*&size_temp.h},
            board: vec![vec![Some(Piece {
                color: Color::Black,
                piece_type: PieceTypes::Pawn,
                notation: Self::get_notation_string(&PieceTypes::Pawn),      // TODO: References of this style need refactoring
                emoji: Self::get_emoji_string(&PieceTypes::Pawn, &Color::Black)
            }); *&size_temp.w as usize];
                        *&size_temp.h as usize],
        }
    }

    fn create_engine_with_empty_board() -> ChessEngine {
        let size_temp = Size {w:8, h:8};
        ChessEngine {
            size: Size {w: *&size_temp.w, h:*&size_temp.h},
            board: vec![vec![None; *&size_temp.w as usize];
                        *&size_temp.h as usize],
        }
    }

    // TODO: Refactor due to the ginormous size of the method
    fn create_engine_with_standard_board() -> ChessEngine {
        struct TempValues { color:Color, piece_type:PieceTypes }

        let mut temp_engine = Self::create_engine_with_empty_board();

        // let mut temp_values = TempValues{ color: Color::Black, piece_type: PieceTypes::Rook };
        // temp_engine.board[0][0] = Some(Piece{
        //     color: *temp_values.color,
        //     piece_type: *temp_values.piece_type,
        //     notation: Self::get_notation_string(*temp_values.piece_type),      // TODO: References of this style need refactoring
        //     emoji: Self::get_emoji_string(&*temp_values.piece_type, *temp_values.color)
        // }));

        // Black --------------------------------------------------------------------------

        // Rooks --------------------------------------------------------------------------
        temp_engine.board[0][0] = Some(Piece{
            color: Color::Black,
            piece_type: PieceTypes::Rook,
            notation: Self::get_notation_string(&PieceTypes::Rook),      // TODO: References of this style need refactoring
            emoji: Self::get_emoji_string(&PieceTypes::Rook, &Color::Black)
        });

        temp_engine.board[0][7] = Some(Piece{
            color: Color::Black,
            piece_type: PieceTypes::Rook,
            notation: Self::get_notation_string(&PieceTypes::Rook),      // TODO: References of this style need refactoring
            emoji: Self::get_emoji_string(&PieceTypes::Rook, &Color::Black)
        });

        // Knights --------------------------------------------------------------------------
        temp_engine.board[0][1] = Some(Piece{
            color: Color::Black,
            piece_type: PieceTypes::Knight,
            notation: Self::get_notation_string(&PieceTypes::Knight),      // TODO: References of this style need refactoring
            emoji: Self::get_emoji_string(&PieceTypes::Knight, &Color::Black)
        });

        temp_engine.board[0][6] = Some(Piece{
            color: Color::Black,
            piece_type: PieceTypes::Knight,
            notation: Self::get_notation_string(&PieceTypes::Knight),      // TODO: References of this style need refactoring
            emoji: Self::get_emoji_string(&PieceTypes::Knight, &Color::Black)
        });

        // Bishops --------------------------------------------------------------------------
        temp_engine.board[0][2] = Some(Piece{
            color: Color::Black,
            piece_type: PieceTypes::Bishop,
            notation: Self::get_notation_string(&PieceTypes::Bishop),      // TODO: References of this style need refactoring
            emoji: Self::get_emoji_string(&PieceTypes::Bishop, &Color::Black)
        });

        temp_engine.board[0][5] = Some(Piece{
            color: Color::Black,
            piece_type: PieceTypes::Bishop,
            notation: Self::get_notation_string(&PieceTypes::Bishop),      // TODO: References of this style need refactoring
            emoji: Self::get_emoji_string(&PieceTypes::Bishop, &Color::Black)
        });

        // Queen --------------------------------------------------------------------------
        temp_engine.board[0][3] = Some(Piece{
            color: Color::Black,
            piece_type: PieceTypes::Queen,
            notation: Self::get_notation_string(&PieceTypes::Queen),      // TODO: References of this style need refactoring
            emoji: Self::get_emoji_string(&PieceTypes::Queen, &Color::Black)
        });

        // King --------------------------------------------------------------------------
        temp_engine.board[0][4] = Some(Piece{
            color: Color::Black,
            piece_type: PieceTypes::King,
            notation: Self::get_notation_string(&PieceTypes::King),      // TODO: References of this style need refactoring
            emoji: Self::get_emoji_string(&PieceTypes::King, &Color::Black)
        });

        // Pawns --------------------------------------------------------------------------
        for x in 0..=7 {
            temp_engine.board[1][x] = Some(Piece{
                color: Color::Black,
                piece_type: PieceTypes::Pawn,
                notation: Self::get_notation_string(&PieceTypes::Pawn),      // TODO: References of this style need refactoring
                emoji: Self::get_emoji_string(&PieceTypes::Pawn, &Color::Black)
            });
        }

        // White --------------------------------------------------------------------------

        // Rooks --------------------------------------------------------------------------
        temp_engine.board[7][0] = Some(Piece{
            color: Color::White,
            piece_type: PieceTypes::Rook,
            notation: Self::get_notation_string(&PieceTypes::Rook),      // TODO: References of this style need refactoring
            emoji: Self::get_emoji_string(&PieceTypes::Rook, &Color::White)
        });

        temp_engine.board[7][7] = Some(Piece{
            color: Color::White,
            piece_type: PieceTypes::Rook,
            notation: Self::get_notation_string(&PieceTypes::Rook),      // TODO: References of this style need refactoring
            emoji: Self::get_emoji_string(&PieceTypes::Rook, &Color::White)
        });

        // Knights --------------------------------------------------------------------------
        temp_engine.board[7][1] = Some(Piece{
            color: Color::White,
            piece_type: PieceTypes::Knight,
            notation: Self::get_notation_string(&PieceTypes::Knight),      // TODO: References of this style need refactoring
            emoji: Self::get_emoji_string(&PieceTypes::Knight, &Color::White)
        });

        temp_engine.board[7][6] = Some(Piece{
            color: Color::White,
            piece_type: PieceTypes::Knight,
            notation: Self::get_notation_string(&PieceTypes::Knight),      // TODO: References of this style need refactoring
            emoji: Self::get_emoji_string(&PieceTypes::Knight, &Color::White)
        });

        // Bishops --------------------------------------------------------------------------
        temp_engine.board[7][2] = Some(Piece{
            color: Color::White,
            piece_type: PieceTypes::Bishop,
            notation: Self::get_notation_string(&PieceTypes::Bishop),      // TODO: References of this style need refactoring
            emoji: Self::get_emoji_string(&PieceTypes::Bishop, &Color::White)
        });

        temp_engine.board[7][5] = Some(Piece{
            color: Color::White,
            piece_type: PieceTypes::Bishop,
            notation: Self::get_notation_string(&PieceTypes::Bishop),      // TODO: References of this style need refactoring
            emoji: Self::get_emoji_string(&PieceTypes::Bishop, &Color::White)
        });

        // Queen --------------------------------------------------------------------------
        temp_engine.board[7][3] = Some(Piece{
            color: Color::White,
            piece_type: PieceTypes::Queen,
            notation: Self::get_notation_string(&PieceTypes::Queen),      // TODO: References of this style need refactoring
            emoji: Self::get_emoji_string(&PieceTypes::Queen, &Color::White)
        });

        // King --------------------------------------------------------------------------
        temp_engine.board[7][4] = Some(Piece{
            color: Color::White,
            piece_type: PieceTypes::King,
            notation: Self::get_notation_string(&PieceTypes::King),      // TODO: References of this style need refactoring
            emoji: Self::get_emoji_string(&PieceTypes::King, &Color::White)
        });

        // Pawns --------------------------------------------------------------------------
        for x in 0..=7 {
            temp_engine.board[6][x] = Some(Piece{
                color: Color::White,
                piece_type: PieceTypes::Pawn,
                notation: Self::get_notation_string(&PieceTypes::Pawn),      // TODO: References of this style need refactoring
                emoji: Self::get_emoji_string(&PieceTypes::Pawn, &Color::White)
            });
        }

        temp_engine
    }

    // TODO: Horrible code, needs refactoring later
    pub fn print_board(&self) {
        self.board.iter().for_each(|row| {
            println!("{}", "-".repeat(25));
            row.iter().for_each(|square| {
                print!("|{}", if square.is_some() {square.as_ref().unwrap().emoji.to_string()} else {"  ".to_string()});
            });
            println!("|");
        });
        println!("{}", "-".repeat(25));
    }

    // TODO: Horrible code, needs refactoring later
    pub fn print_board_with_ranks(&self) {
        print!("  ");
        for letter in b'A'..=b'H'{
            print!(" {} ", letter as char);
        }
        println!();

        let mut numbered_rank = 8;
        self.board.iter().for_each(|row| {
            println!("  {}", "-".repeat((&self.size.w * 3 + 1) as usize));
            print!("{} ", &numbered_rank);
            row.iter().for_each(|square| {
                print!("|{}", if square.is_some() {square.as_ref().unwrap().emoji.to_string()} else {"  ".to_string()});
            });
            println!("|");

            numbered_rank -= 1;
        });
        println!("  {}", "-".repeat((&self.size.w * 3 + 1) as usize));
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
    fn get_emoji_string(piece: &PieceTypes, color: &Color) -> String {
        match color {
            Color::White => {
                (&Self::get_white_emoji_hashmap()
                    .get(&piece)
                    .unwrap()).to_string()
            }
            Color::Black => {
                (&Self::get_black_emoji_hashmap()
                    .get(&piece)
                    .unwrap()).to_string()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestContents {
        chess_engine: ChessEngine,
    }

    fn setup() -> TestContents {
        TestContents {
            chess_engine: ChessEngine::new(),
        }
    }

    fn setup_chess_engine() -> ChessEngine {
        ChessEngine::new()
    }

    #[test]
    fn print_empty_board() {
        let chess_engine = ChessEngine::create_engine_with_empty_board();

        chess_engine.print_board()

    }

    #[test]
    fn print_filled_board() {
        let chess_engine = ChessEngine::create_engine_with_white_board();

        chess_engine.print_board()
    }

    #[test]
    fn print_empty_board_with_ranks() {
        let chess_engine = ChessEngine::create_engine_with_empty_board();

        chess_engine.print_board_with_ranks()

    }

    #[test]
    fn print_filled_board_with_ranks() {
        let chess_engine = ChessEngine::create_engine_with_white_board();

        chess_engine.print_board_with_ranks()
    }

    #[test]
    fn print_standard_board() {
        let chess_engine = ChessEngine::create_engine_with_standard_board();

        chess_engine.print_board()

    }

    #[test]
    fn print_standard_board_with_ranks() {
        let chess_engine = ChessEngine::create_engine_with_standard_board();

        chess_engine.print_board_with_ranks()
    }
}
