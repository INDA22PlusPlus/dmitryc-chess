struct Size {
    w:u8,
    h:u8,
}

struct ChessEngine{
    pub board: Vec<Vec<Piece>>,
    size: Size,
}

impl ChessEngine {
    pub fn new() -> ChessEngine{
        Self::create_engine_with_white_board()
    }

    fn create_engine_with_white_board() -> ChessEngine {
        let size_temp = Size {w:8, h:8};
        ChessEngine {
            size: Size {w: *&size_temp.w, h:*&size_temp.h},
            board: vec![vec![Piece{ color: Color::White }; *&size_temp.w as usize];
                        *&size_temp.h as usize],
        }
    }
}

#[derive(Debug, Clone)]
struct Piece {
    color: Color,
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

        chess_engine.board.iter().for_each(|it| {
            println!("{:#?}", it);
        })
    }
}
