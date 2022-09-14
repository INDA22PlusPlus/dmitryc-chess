struct ChessEngine{
    pub board: Vec<Vec<Piece>>
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_engine() {
        let chess_engine = ChessEngine::new();
    }
}
