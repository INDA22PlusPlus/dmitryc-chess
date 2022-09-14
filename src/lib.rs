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

    #[test]
    fn create_engine() {
        let chess_engine = ChessEngine{ board: vec![] };

    }

    #[test]
    fn print_board_using_loops() {
        // for y in chess_engine.board{
        //     for x in chess_engine.board{
        //
        //     }
        // }
        let chess_engine = ChessEngine{ board: vec![] };

        // println!("test out");

        chess_engine.board.iter().for_each(|it| {
            println!("{:#?}", it);
        })
    }
}
