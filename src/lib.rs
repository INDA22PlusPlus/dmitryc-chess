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
    fn print_board_using_loops() {
        let chess_engine = setup_chess_engine();

        // println!("test out");

        chess_engine.board.iter().for_each(|row| {
            // print!("{:#?}", it);
            println!("{}", "-".repeat(25));
            row.iter().for_each(|square| {
                print!("|{}", square);
            });
            println!("|");
        });
        println!("{}", "-".repeat(25));

    }
}
