use std::fmt;

#[derive(Clone)]
struct CustomOption<Piece>(Option<Piece>);

impl<Piece> fmt::Display for CustomOption<Piece> {
    fn fmt(&self, formatter: &mut fmt::Formatter<>) -> fmt::Result {
        match self.0 {
            Some(ref piece) => write!(formatter, "P "),
            None => write!(formatter, "  "),
        }
    }
}

struct Size {
    w:u8,
    h:u8,
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

    fn create_engine_with_white_board() -> ChessEngine {
        let size_temp = Size {w:8, h:8};
        ChessEngine {
            size: Size {w: *&size_temp.w, h:*&size_temp.h},
            board: vec![vec![CustomOption(Some(Piece { color: Color::White, string_name: "P".to_string() })); *&size_temp.w as usize];
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
}

#[derive(Debug, Clone)]
struct Piece {
    color: Color,
    string_name: String,
}

#[derive(Debug, Clone)]
enum Color{
    White,
    Black
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

    // #[test]
    // fn create_engine() {
    //     let chess_engine = ChessEngine{ board: vec![] };
    //
    // }

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
