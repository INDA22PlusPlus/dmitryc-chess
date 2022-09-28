# Chess Engine
A Chess Engine Library

## Setting up with an example:

Cargo.toml:

    ...
    [dependencies]
    chess = { git = "https://github.com/INDA22PlusPlus/dmitryc-chess" }

main.rs:
    
    // Imports
    use chess::chess_engine::*;
    use chess::piece_types::*;
    use chess::colors::*;
    use chess::piece::*;
    use chess::coords::*;

    fn main() {
        // Creates a chess engine with standard chess board layout
        let chess_engine = ChessEngine::new();

        // Prints board to console
        chess_engine.print_board()
    }

## Useful methods for chess engine:

| Syntax                            | Description                               |
| -----------                       | -----------                               |
| get_board                         | Returns a cloned board of type            |
| get_piece_option_with_notation    | Takes a &str using algebraic notation, Returns a cloned piece option of. Contains either None if square has no piece or Some(Piece) if there is a piece on the square |
