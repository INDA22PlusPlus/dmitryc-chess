use crate::coords::*;
use crate::colors::*;
use crate::piece_types::*;
use crate::piece::*;

#[derive(Clone)]
pub struct ChessEngine{
    pub board: Vec<Vec<Option<Piece>>>,
    size: Size,
    selected_piece: Option<Piece>,
}

impl ChessEngine {
    pub fn new() -> ChessEngine{
        // Self::create_engine_with_white_board()
        Self::create_engine_with_standard_board()
    }

    // TODO: Non-readable code, needs refactoring later
    fn create_engine_filled_with_white_board() -> ChessEngine {
        let temp_size = Size::new(8, 8);
        let mut temp_board = vec![vec![None;temp_size.w as usize]; temp_size.h as usize];
        for x in 0..temp_size.w  {
            for y in 0..temp_size.h {
                temp_board[x as usize][y as usize] = Some(Piece::new(PieceTypes::Pawn,
                                                                     Colors::White,
                                                                     Coords::new(x, y)));
            }
        }
        ChessEngine {
            size: temp_size,
            board: temp_board,
            selected_piece: None,
        }
    }

    fn create_engine_with_empty_board() -> ChessEngine {
        let temp_size = Size::new(8, 8);
        ChessEngine {
            size: temp_size,
            board: vec![vec![None; *&temp_size.w as usize];
                        *&temp_size.h as usize],
            selected_piece: None,
        }
    }

    // TODO: Refactor due to the ginormous size of the method
    fn create_engine_with_standard_board() -> ChessEngine {
        let mut temp_engine = Self::create_engine_with_empty_board();

        // Black --------------------------------------------------------------------------

        // Rooks --------------------------------------------------------------------------
        temp_engine.board[0][0] = Some(Piece::new(PieceTypes::Rook, Colors::Black, Coords::new(0,0)));

        temp_engine.board[0][7] = Some(Piece::new(PieceTypes::Rook, Colors::Black, Coords::new(7, 0)));

        // Knights --------------------------------------------------------------------------
        temp_engine.board[0][1] = Some(Piece::new(PieceTypes::Knight, Colors::Black, Coords::new(1, 0)));

        temp_engine.board[0][6] = Some(Piece::new(PieceTypes::Knight, Colors::Black, Coords::new(6, 0)));

        // Bishops --------------------------------------------------------------------------
        temp_engine.board[0][2] = Some(Piece::new(PieceTypes::Bishop, Colors::Black, Coords::new(2, 0)));

        temp_engine.board[0][5] = Some(Piece::new(PieceTypes::Bishop, Colors::Black, Coords::new(5, 0)));

        // Queen --------------------------------------------------------------------------
        temp_engine.board[0][3] = Some(Piece::new(PieceTypes::Queen, Colors::Black, Coords::new(3, 0)));

        // King --------------------------------------------------------------------------
        temp_engine.board[0][4] = Some(Piece::new(PieceTypes::King, Colors::Black, Coords::new(4, 0)));

        // Pawns --------------------------------------------------------------------------
        for x in 0..=7 {
            temp_engine.board[1][x] = Some(Piece::new(PieceTypes::Pawn, Colors::Black, Coords::new(x, 1)));
        }

        // White --------------------------------------------------------------------------

        // Rooks --------------------------------------------------------------------------
        temp_engine.board[7][0] = Some(Piece::new(PieceTypes::Rook, Colors::White, Coords::new(0, 7)));

        temp_engine.board[7][7] = Some(Piece::new(PieceTypes::Rook, Colors::White, Coords::new(7, 7)));

        // Knights --------------------------------------------------------------------------
        temp_engine.board[7][1] = Some(Piece::new(PieceTypes::Knight, Colors::White, Coords::new(1, 7)));

        temp_engine.board[7][6] = Some(Piece::new(PieceTypes::Knight, Colors::White, Coords::new(6, 7)));
        // Bishops --------------------------------------------------------------------------
        temp_engine.board[7][2] = Some(Piece::new(PieceTypes::Bishop, Colors::White, Coords::new(2, 7)));

        temp_engine.board[7][5] = Some(Piece::new(PieceTypes::Bishop, Colors::White, Coords::new(5, 7)));

        // Queen --------------------------------------------------------------------------
        temp_engine.board[7][3] = Some(Piece::new(PieceTypes::Queen, Colors::White, Coords::new(3, 7)));

        // King --------------------------------------------------------------------------
        temp_engine.board[7][4] = Some(Piece::new(PieceTypes::King, Colors::White, Coords::new(4, 7)));

        // Pawns --------------------------------------------------------------------------
        for x in 0..=7 {
            temp_engine.board[6][x] = Some(Piece::new(PieceTypes::Pawn, Colors::White, Coords::new(x, 6)));
        }

        temp_engine
    }

    pub fn get_piece_string_from_option(square: &Option<Piece>) -> String {
        if square.is_some() {square.as_ref().unwrap().emoji.to_string()} else {"  ".to_string()}
    }

    // TODO: Horrible code, needs refactoring later
    pub fn print_board(&self) {
        self.board.iter().for_each(|row| {
            println!("{}", "-".repeat(25));
            row.iter().for_each(|square| {
                print!("|{}", Self::get_piece_string_from_option(square));    // TODO Figure out the right way to do this
            });
            println!("|");
        });
        println!("{}", "-".repeat(25));
    }

    // TODO: Horrible code, needs refactoring later
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
                print!("|{}", Self::get_piece_string_from_option(square));    // TODO Figure out the right way to do this
            });
            println!("|");

            numbered_rank -= 1;
        });
        println!("  {}", "-".repeat((&self.size.w * 3 + 1) as usize));
    }

    // TODO: Fix scuffed moving of elements from one vec position to another
    pub fn force_move_piece_with_coords(&mut self, from: Coords, to:Coords) {
        if self.board[from.y][from.x].is_some(){
            // TODO: Fix creating of new object instead of moving existing
            self.board[to.y][to.x] = Some(Piece::new(
                self.board[from.y][from.x].as_ref().unwrap().piece_type,
                self.board[from.y][from.x].as_ref().unwrap().color,
                to));

            // self.board[to.y][to.x] = self.board[from.y][from.x].clone();
            // self.board[to.y][to.x].unwrap().set_coords(to);              \\ How to fix?

            self.board[from.y][from.x] = None;
        }
    }

    pub fn get_piece_option_with_coords(&self, x:usize, y:usize) -> Option<Piece> {
        self.board[y][x].clone()
    }

    pub fn force_move_selected_piece_with_coords(&mut self, x:usize, y:usize) {
        if self.selected_piece.is_some(){
            self.force_move_piece_with_coords(
                self.selected_piece.as_ref().unwrap().coords,
                Coords{x, y}
            );
            self.selected_piece = None;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // struct TestContents {
    //     chess_engine: ChessEngine,
    // }
    //
    // fn setup() -> TestContents {
    //     TestContents {
    //         chess_engine: ChessEngine::new(),
    //     }
    // }

    #[test]
    fn test_create_new() {
        let chess_engine = ChessEngine::new();
    }

    #[test]
    fn test_create_standard_board() {
        let chess_engine = ChessEngine::create_engine_with_standard_board();
    }

    #[test]
    fn test_create_empty_board() {
        let chess_engine = ChessEngine::create_engine_with_empty_board();
    }

    #[test]
    fn test_create_filled_board() {
        let chess_engine = ChessEngine::create_engine_filled_with_white_board();
    }

    #[test]
    fn test_new_selected_board_is_none() {
        let chess_engine = ChessEngine::new();
        assert_eq!(chess_engine.selected_piece, None)
    }

    #[test]
    fn test_standard_selected_board_is_none() {
        let chess_engine = ChessEngine::create_engine_with_standard_board();
        assert_eq!(chess_engine.selected_piece, None)
    }

    #[test]
    fn test_empty_selected_board_is_none() {
        let chess_engine = ChessEngine::create_engine_with_empty_board();
        assert_eq!(chess_engine.selected_piece, None)
    }

    #[test]
    fn test_filled_board_selected_is_none() {
        let chess_engine = ChessEngine::create_engine_filled_with_white_board();
        assert_eq!(chess_engine.selected_piece, None)
    }

    //TODO: Need to fix tests to compare to board states (or worst case strings)
    #[test]
    fn print_empty_board() {
        let chess_engine = ChessEngine::create_engine_with_empty_board();

        chess_engine.print_board();

    }

    //TODO: Need to fix tests to compare to board states (or worst case strings)
    #[test]
    fn print_filled_board() {
        let chess_engine = ChessEngine::create_engine_filled_with_white_board();

        chess_engine.print_board();
    }

    //TODO: Need to fix tests to compare to board states (or worst case strings)
    #[test]
    fn print_empty_board_with_ranks() {
        let chess_engine = ChessEngine::create_engine_with_empty_board();

        chess_engine.print_board_with_ranks();

    }

    //TODO: Need to fix tests to compare to board states (or worst case strings)
    #[test]
    fn print_filled_board_with_ranks() {
        let chess_engine = ChessEngine::create_engine_filled_with_white_board();

        chess_engine.print_board_with_ranks();
    }

    //TODO: Need to fix tests to compare to board states (or worst case strings)
    #[test]
    fn print_standard_board() {
        let chess_engine = ChessEngine::new();

        chess_engine.print_board();

    }

    //TODO: Need to fix tests to compare to board states (or worst case strings)
    #[test]
    fn print_standard_board_with_ranks() {
        let chess_engine = ChessEngine::new();

        chess_engine.print_board_with_ranks();
    }

    // TODO: DEFINITELY FIX THE RETURN VALUE PROBLEM I HATE THIS
    fn chess_engine_move_piece_from_to_assert(mut chess_engine:
                                              ChessEngine,
                                              from: Coords,
                                              to: Coords) -> ChessEngine {

        // chess_engine.print_board_with_ranks();
        // println!("{:?}", chess_engine.get_piece_option_with_coords(from.x, from.y));
        // println!("{:?}", chess_engine.get_piece_option_with_coords(to.x, to.y));

        //TODO: Need to fix this
        let mut piece_to_compare_to = if chess_engine.get_piece_option_with_coords(from.x, from.y).is_some() {
            Some(Piece::new(
                chess_engine.get_piece_option_with_coords(from.x, from.y).as_ref().unwrap().piece_type,
                chess_engine.get_piece_option_with_coords(from.x, from.y).as_ref().unwrap().color,
                to
            ))
        } else {chess_engine.get_piece_option_with_coords(to.x, to.y)};

        chess_engine.force_move_piece_with_coords(from, to);

        // chess_engine.print_board_with_ranks();
        // println!("{:?}", chess_engine.get_piece_option_with_coords(from.x, from.y));
        // println!("{:?}", chess_engine.get_piece_option_with_coords(to.x, to.y));

        assert_eq!(chess_engine.get_piece_option_with_coords(from.x, from.y), None);
        assert_eq!(chess_engine.get_piece_option_with_coords(to.x, to.y), piece_to_compare_to);

        chess_engine
    }

    #[test]
    fn test_piece_move_to_empty_square() {
        let mut chess_engine = ChessEngine::new();

        chess_engine_move_piece_from_to_assert(
            chess_engine,
            Coords{ x: 7, y: 7 },
            Coords { x: 3, y: 3 }
        );
    }

    #[test]
    fn test_piece_move_to_occupied_square() {
        let mut chess_engine = ChessEngine::new();

        chess_engine = chess_engine_move_piece_from_to_assert(
            chess_engine,
            Coords{ x: 7, y: 7 },
            Coords { x: 7, y: 0 }
        );

        chess_engine = chess_engine_move_piece_from_to_assert(
            chess_engine,
            Coords{ x: 7, y: 0 },
            Coords { x: 3, y: 3 }
        );

        // chess_engine.print_board_with_ranks();

    }

    //TODO: Need to fix tests to compare to board states (or worst case strings)
    #[test]
    fn test_empty_move_to_empty_square() {
        let mut chess_engine = ChessEngine::new();

        chess_engine = chess_engine_move_piece_from_to_assert(
            chess_engine,
            Coords{ x: 3, y: 3 },
            Coords { x: 4, y: 4 }
        );

    }

    //TODO: Need to fix tests to compare to board states (or worst case strings)
    #[test]
    fn test_empty_move_to_occupied_square() {
        let mut chess_engine = ChessEngine::new();

        chess_engine = chess_engine_move_piece_from_to_assert(
            chess_engine,
            Coords{ x: 3, y: 3 },
            Coords { x: 7, y: 7 }
        );
    }

    #[test]
    #[should_panic]
    fn test_exceeding_coords_from() {
        let mut chess_engine = ChessEngine::new();

        chess_engine.force_move_piece_with_coords(
            Coords::new(8, 8),
            Coords::new(7,7)
        );
    }

    #[test]
    #[should_panic]
    fn test_exceeding_coords_to() {
        let mut chess_engine = ChessEngine::new();

        chess_engine.force_move_piece_with_coords(
            Coords::new(3, 3),
            Coords::new(8,8)
        );
    }

    #[test]
    fn test_get_piece() {
        let mut chess_engine = ChessEngine::new();

        // println!("{}", ChessEngine::get_piece_string_from_option(&chess_engine.get_piece_with_coords(7, 7)));

        assert_eq!(chess_engine.get_piece_option_with_coords(7, 7), chess_engine.board[7][7].clone());
    }

    #[test]
    #[should_panic]
    fn test_get_piece_exceeding_coords() {
        let mut chess_engine = ChessEngine::new();

        chess_engine.get_piece_option_with_coords(8, 8);
    }

    #[test]
    fn test_manual_selection() {
        let mut chess_engine = ChessEngine::new();

        chess_engine.selected_piece = chess_engine.get_piece_option_with_coords(7, 7);

        // println!("{}", ChessEngine::get_piece_string_from_option(&chess_engine.selected_piece));

        assert_eq!(chess_engine.selected_piece, chess_engine.get_piece_option_with_coords(7, 7));
    }

    #[test]
    fn test_get_piece_string() {
        let mut chess_engine = ChessEngine::new();

        let piece_string = ChessEngine::get_piece_string_from_option(
            &chess_engine.get_piece_option_with_coords(7, 7)
        );

        println!("{}", piece_string);

        assert_eq!(piece_string, "♖ ");
    }

    #[test]
    fn test_manual_selection_and_moving() {
        let mut chess_engine = ChessEngine::new();

        let mut from = Coords{ x: 7, y: 7 };
        let mut to = Coords{ x: 7, y: 0 };

        chess_engine.selected_piece = chess_engine.get_piece_option_with_coords(from.x, from.y);

        chess_engine.force_move_selected_piece_with_coords(to.x, to.y);

        assert_eq!(chess_engine.get_piece_option_with_coords(from.x, from.y), None);
        assert_eq!(chess_engine.get_piece_option_with_coords(to.x, to.y), Some(Piece::new(
            PieceTypes::Rook, Colors::White, to)));

        let mut from = Coords{ x: 7, y: 0 };
        let mut to = Coords{ x: 3, y: 3 };

        chess_engine.selected_piece = chess_engine.get_piece_option_with_coords(from.x, from.y);


        chess_engine.force_move_selected_piece_with_coords(to.x, to.y);

        assert_eq!(chess_engine.get_piece_option_with_coords(from.x, from.y), None);
        assert_eq!(chess_engine.get_piece_option_with_coords(to.x, to.y), Some(Piece::new(
            PieceTypes::Rook, Colors::White, to)));

        // chess_engine.print_board_with_ranks();
        // println!("{:?}", chess_engine.get_piece_option_with_coords(from.x, from.y));
        // println!("{:?}", chess_engine.get_piece_option_with_coords(to.x, to.y));
        // println!("{:?}", chess_engine.selected_piece);
    }
}
