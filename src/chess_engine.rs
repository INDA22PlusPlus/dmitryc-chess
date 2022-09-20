use crate::size::*;
use crate::colors::*;
use crate::piece_types::*;
use crate::piece::*;

use std::collections::HashMap;
use std::fmt;

pub struct ChessEngine{
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
            board: vec![vec![
                Some(Piece::new(PieceTypes::Pawn, Colors::White)
            ); *&size_temp.w as usize];
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
        struct TempValues { Colors:Colors, piece_type:PieceTypes }

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
        temp_engine.board[0][0] = Some(Piece::new(PieceTypes::Rook, Colors::Black));

        temp_engine.board[0][7] = Some(Piece::new(PieceTypes::Rook, Colors::Black));

        // Knights --------------------------------------------------------------------------
        temp_engine.board[0][1] = Some(Piece::new(PieceTypes::Knight, Colors::Black));

        temp_engine.board[0][6] = Some(Piece::new(PieceTypes::Knight, Colors::Black));

        // Bishops --------------------------------------------------------------------------
        temp_engine.board[0][2] = Some(Piece::new(PieceTypes::Bishop, Colors::Black));

        temp_engine.board[0][5] = Some(Piece::new(PieceTypes::Bishop, Colors::Black));

        // Queen --------------------------------------------------------------------------
        temp_engine.board[0][3] = Some(Piece::new(PieceTypes::Queen, Colors::Black));

        // King --------------------------------------------------------------------------
        temp_engine.board[0][4] = Some(Piece::new(PieceTypes::King, Colors::Black));

        // Pawns --------------------------------------------------------------------------
        for x in 0..=7 {
            temp_engine.board[1][x] = Some(Piece::new(PieceTypes::Pawn, Colors::Black));
        }

        // White --------------------------------------------------------------------------

        // Rooks --------------------------------------------------------------------------
        temp_engine.board[7][0] = Some(Piece::new(PieceTypes::Rook, Colors::White));

        temp_engine.board[7][7] = Some(Piece::new(PieceTypes::Rook, Colors::White));

        // Knights --------------------------------------------------------------------------
        temp_engine.board[7][1] = Some(Piece::new(PieceTypes::Knight, Colors::White));

        temp_engine.board[7][6] = Some(Piece::new(PieceTypes::Knight, Colors::White));
        // Bishops --------------------------------------------------------------------------
        temp_engine.board[7][2] = Some(Piece::new(PieceTypes::Bishop, Colors::White));

        temp_engine.board[7][5] = Some(Piece::new(PieceTypes::Bishop, Colors::White));

        // Queen --------------------------------------------------------------------------
        temp_engine.board[7][3] = Some(Piece::new(PieceTypes::Queen, Colors::White));

        // King --------------------------------------------------------------------------
        temp_engine.board[7][4] = Some(Piece::new(PieceTypes::King, Colors::White));

        // Pawns --------------------------------------------------------------------------
        for x in 0..=7 {
            temp_engine.board[6][x] = Some(Piece::new(PieceTypes::Pawn, Colors::White));
        }

        temp_engine
    }

    // TODO: Horrible code, needs refactoring later
    pub fn print_board(&self) {
        self.board.iter().for_each(|row| {
            println!("{}", "-".repeat(25));
            row.iter().for_each(|square| {
                print!("|{}", if square.is_some() {square.as_ref().unwrap().emoji.to_string()} else {"  ".to_string()});    // TODO Figure out the right way to do this
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
                print!("|{}", if square.is_some() {square.as_ref().unwrap().emoji.to_string()} else {"  ".to_string()});    // TODO Figure out the right way to do this
            });
            println!("|");

            numbered_rank -= 1;
        });
        println!("  {}", "-".repeat((&self.size.w * 3 + 1) as usize));
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
