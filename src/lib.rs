use std::collections::HashMap;
use std::fmt;

#[derive(Clone)]
struct CustomOption<Piece>(Option<Piece>);

impl fmt::Display for CustomOption<Piece> {
    fn fmt(&self, formatter: &mut fmt::Formatter<>) -> fmt::Result {
        match &self.0 {
            Some(ref piece) => write!(formatter, "{}", piece.string_name),
            None => write!(formatter, "  "),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Size {
    w:u8,
    h:u8,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Piece {
    color: Color,
    piece_type: PieceTypes,
    string_name: String,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum Color{
    White,
    Black
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum PieceTypes{
    King,
    Rook,
    Bishop,
    Queen,
    Knight,
    Pawn,
}

struct ChessEngine{
    pub board: Vec<Vec<CustomOption<Piece>>>,
    size: Size,
}

impl ChessEngine {
    pub fn new() -> ChessEngine{
        // Self::create_engine_with_white_board()
        Self::create_engine_with_empty_board()
    }

    // Non-readable code, needs refactoring later
    fn create_engine_with_white_board() -> ChessEngine {
        let size_temp = Size {w:8, h:8};
        ChessEngine {
            size: Size {w: *&size_temp.w, h:*&size_temp.h},
            board: vec![vec![CustomOption(Some(Piece { 
                color: Color::White,
                piece_type: PieceTypes::King,
                string_name: Self::enum_piece_to_notation(&PieceTypes::King)
            })); *&size_temp.w as usize];
                        *&size_temp.h as usize],
        }
    }

    fn create_engine_with_empty_board() -> ChessEngine {
        let size_temp = Size {w:8, h:8};
        ChessEngine {
            size: Size {w: *&size_temp.w, h:*&size_temp.h},
            board: vec![vec![CustomOption(None); *&size_temp.w as usize];
                        *&size_temp.h as usize],
        }
    }

    // Horrible code, needs refactoring later
    pub fn print_board(&self) {
        self.board.iter().for_each(|row| {
            println!("{}", "-".repeat(25));
            row.iter().for_each(|square| {
                print!("|{}", square);
            });
            println!("|");
        });
        println!("{}", "-".repeat(25));
    }

    // Horrible code, needs refactoring later
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
                print!("|{square}");
            });
            println!("|");

            numbered_rank -= 1;
        });
        println!("  {}", "-".repeat((&self.size.w * 3 + 1) as usize));
    }

    fn get_piece_to_notation_hash() -> HashMap<PieceTypes, String> {
        let mut map: HashMap<PieceTypes, String> = HashMap::new();
        map.insert(PieceTypes::King, "K ".to_string());
        map.insert(PieceTypes::Rook, "R ".to_string());
        map.insert(PieceTypes::Bishop, "B ".to_string());
        map.insert(PieceTypes::Queen, "Q ".to_string());
        map.insert(PieceTypes::Knight, "Kn".to_string());
        map.insert(PieceTypes::Pawn, "P ".to_string());

        map
    }

    fn enum_piece_to_notation(piece: &PieceTypes) -> String {
        (&Self::get_piece_to_notation_hash()
            .get(&piece)
            .unwrap()).to_string()
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
}
