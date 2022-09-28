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

## Useful info about the library

ChessEngine is a struct with following fields:

	pub struct ChessEngine {  
		pub board: Vec<Vec<Option<Piece>>>,  
		pub size: Size,  
		pub selected_piece: Option<Piece>,  
		pub turn: Colors,  
		pub status: Status,  
	}

The `board` field is a 2-d vector, representing squares of type `Option<Piece>`, where `None` represents an empty square and `Some(Piece)` represents a piece struct of type `Piece`. Remember that if you are going to be accessing `board` values directly, that the order is **reversed**, because the first vector is rows and the second columns, which means that you access a square by `chess_engine.board[y][x]` and **not** by `chess_engine.board[x][y]`.

The Piece struct has following fields:

	pub struct Piece {  
	    pub piece_type: PieceTypes,  
		pub color: Colors,  
		pub notation: String,  
		pub emoji: String,  
		pub coords: Coords,  
	}

Useful fields are `type`,  `color` and `coords`.  Fields `type` and `color` are enums, while `coords` is a struct with values for x and y, representing the piece position on the board, starting at `Coords{x:0, y:0}` at the top left corner (`"a8"` in algebraic notation) and ending with `Coords{x:7, y:7}` in the right bottom corner (`"h1"`).

## Useful methods for chess engine

| Syntax                            | Description                               					|
| ----------------------------------|---------------------------------------------------------------|
| `get_board()`                     | Returns a cloned board of type `Vec<Vec<Option<Piece>>>`.		|
| `select_piece_notation(&str)`     | Takes a &str for square position using algebraic notation, e.g. `"a1"` or `"e5"` etc. Selects a piece of type `Option<Piece>`, storing it in the `selected_piece` struct field. |
| `play_selected_piece_with_notation(&str)` | Plays the selected piece to the given square using algebraic notation, e.g. `"a1"`. Deselects the piece if it was successfully played. |
| `get_piece_option_with_notation(&str)` | As in previous methods, takes a &str in algebraic notation, e.g. `"a1"`. Returns a cloned piece option of type `Option<Piece>`. Contains either `None` if square has no piece or `Some(Piece)` if there is a piece on the square. |
| `get_sellected_piece()`                 | Returns a cloned selected piece of type `Option<Piece>` 	|
