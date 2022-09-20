use crate::coords::*;
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
        let temp_size = Size{w:8, h:8};
        let mut temp_board = vec![vec![None;temp_size.w as usize]; temp_size.h as usize];
        for x in 0..temp_size.w  {
            for y in 0..temp_size.h {
                temp_board[x as usize][y as usize] = Some(Piece::new(PieceTypes::Pawn,
                                                                     Colors::White,
                                                                     Coords{x, y}));
            }
        }
        ChessEngine {
            size: Size {w: *&temp_size.w, h:*&temp_size.h},
            board: temp_board,
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
        temp_engine.board[0][0] = Some(Piece::new(PieceTypes::Rook, Colors::Black, Coords{x:0, y:0}));

        temp_engine.board[0][7] = Some(Piece::new(PieceTypes::Rook, Colors::Black, Coords{x:7, y:0}));

        // Knights --------------------------------------------------------------------------
        temp_engine.board[0][1] = Some(Piece::new(PieceTypes::Knight, Colors::Black, Coords{x:1, y:0}));

        temp_engine.board[0][6] = Some(Piece::new(PieceTypes::Knight, Colors::Black, Coords{x:6, y:0}));

        // Bishops --------------------------------------------------------------------------
        temp_engine.board[0][2] = Some(Piece::new(PieceTypes::Bishop, Colors::Black, Coords{x:2, y:0}));

        temp_engine.board[0][5] = Some(Piece::new(PieceTypes::Bishop, Colors::Black, Coords{x:5, y:0}));

        // Queen --------------------------------------------------------------------------
        temp_engine.board[0][3] = Some(Piece::new(PieceTypes::Queen, Colors::Black, Coords{x:3, y:0}));

        // King --------------------------------------------------------------------------
        temp_engine.board[0][4] = Some(Piece::new(PieceTypes::King, Colors::Black, Coords{x:4, y:0}));

        // Pawns --------------------------------------------------------------------------
        for x in 0..=7 {
            temp_engine.board[1][x] = Some(Piece::new(PieceTypes::Pawn, Colors::Black, Coords{x:(x as u8), y:1}));
        }

        // White --------------------------------------------------------------------------

        // Rooks --------------------------------------------------------------------------
        temp_engine.board[7][0] = Some(Piece::new(PieceTypes::Rook, Colors::White, Coords{x:0, y:7}));

        temp_engine.board[7][7] = Some(Piece::new(PieceTypes::Rook, Colors::White, Coords{x:7, y:7}));

        // Knights --------------------------------------------------------------------------
        temp_engine.board[7][1] = Some(Piece::new(PieceTypes::Knight, Colors::White, Coords{x:1, y:7}));

        temp_engine.board[7][6] = Some(Piece::new(PieceTypes::Knight, Colors::White, Coords{x:6, y:7}));
        // Bishops --------------------------------------------------------------------------
        temp_engine.board[7][2] = Some(Piece::new(PieceTypes::Bishop, Colors::White, Coords{x:2, y:7}));

        temp_engine.board[7][5] = Some(Piece::new(PieceTypes::Bishop, Colors::White, Coords{x:5, y:7}));

        // Queen --------------------------------------------------------------------------
        temp_engine.board[7][3] = Some(Piece::new(PieceTypes::Queen, Colors::White, Coords{x:3, y:7}));

        // King --------------------------------------------------------------------------
        temp_engine.board[7][4] = Some(Piece::new(PieceTypes::King, Colors::White, Coords{x:4, y:7}));

        // Pawns --------------------------------------------------------------------------
        for x in 0..=7 {
            temp_engine.board[6][x] = Some(Piece::new(PieceTypes::Pawn, Colors::White, Coords{x:(x as u8), y:6}));
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
