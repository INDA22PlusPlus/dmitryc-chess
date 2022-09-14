struct ChessEngine{
    pub board: Vec<Vec<Piece>>
}

struct Piece{
    color: Color,
}

enum Color{
    White,
    Black
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_engine() {
        let chess_engine = ChessEngine::new();
    }
}
