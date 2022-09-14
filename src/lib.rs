struct ChessEngine{
    pub board: Vec<Vec<Piece>>
}

// impl ChessEngine {
//     fn create_empty(self){
//
//     }
// }

#[derive(Debug)]
struct Piece {
    color: Color,
}

#[derive(Debug)]
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
            chess_engine: ChessEngine { board: vec![] }
        }
    }

    fn setup_chess_engine() -> ChessEngine {
        ChessEngine {
            board: vec![]
        }
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
