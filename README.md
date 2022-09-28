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
            // Cretes a chess engine with standard chess board layout
            let chess_engine = ChessEngine::new();
            
            // Prints board to console
            chess_engine.print_board()
        }

## Useful methods:
