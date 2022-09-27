use std::collections::{HashMap, HashSet};
use std::iter::zip;
use crate::coords::*;
use crate::colors::*;
use crate::piece_types::*;
use crate::piece::*;
use colored::Colorize;
use crate::chess_engine::Status::{Checked, Normal};

#[derive(Debug, Clone)]
enum Status {
    Normal,
    Checked,
    Stalemated,
    Checkmated,
}

#[derive(Debug, Clone)]
pub struct ChessEngine{
    pub board: Vec<Vec<Option<Piece>>>,
    size: Size,
    selected_piece: Option<Piece>,
    turn: Colors,
    status: Status,
}

impl ChessEngine {
    pub fn new() -> ChessEngine{
        // Self::create_engine_with_white_board()
        Self::create_engine_with_standard_board()
    }

    // TODO: Non-readable code, needs refactoring later
    pub fn create_engine_filled_with_white_board() -> ChessEngine {
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
            turn: Colors::White,
            status: Status::Normal,
        }
    }

    pub fn create_engine_with_empty_board() -> ChessEngine {
        let temp_size = Size::new(8, 8);
        ChessEngine {
            size: temp_size,
            board: vec![vec![None; *&temp_size.w as usize];
                        *&temp_size.h as usize],
            selected_piece: None,
            turn: Colors::White,
            status: Status::Normal,
        }
    }

    // TODO: Refactor due to the ginormous size of the method
    pub fn create_engine_with_standard_board() -> ChessEngine {
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
    pub fn print_board_with_ranks_and_files(&self) {
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

    pub fn print_board_with_ranks_and_files_with_all_legal_moves(&self) {
        let moves = self.get_all_legal_moves();

        print!("  ");
        for letter in b'A'..=b'H'{
            print!(" {} ", letter as char);
        }
        println!();

        let mut numbered_rank = 8;
        for (row, y) in zip(self.board.clone(), 0..=7) {
            println!("  {}", "-".repeat((&self.size.w * 3 + 1) as usize));
            print!("{} ", &numbered_rank);
            for (square, x) in zip(row, 0..=7) {
                let coords = Coords::new(x, y);
                let mut square_str = &*Self::get_piece_string_from_option(&square);
                if moves.contains(&coords){
                    print!("|{}", square_str.on_purple());
                }
                else {
                    print!("|{}", square_str);    // TODO Figure out the right way to do this
                }
            }
            println!("|");

            numbered_rank -= 1;
        }
    }

    pub fn print_board_with_ranks_and_files_with_all_legal_moves_different_colors(&self) {
        let moves = self.get_moves();
        let attacked_squares = self.get_all_attacked_squares();
        let attacked_pieces_squares = self.get_all_attacked_pieces_squares();
        let special_moves = self.get_all_special_moves();

        print!("  ");
        for letter in b'A'..=b'H'{
            print!(" {} ", letter as char);
        }
        println!();

        let mut numbered_rank = 8;
        for (row, y) in zip(self.board.clone(), 0..=7) {
            println!("  {}", "-".repeat((&self.size.w * 3 + 1) as usize));
            print!("{} ", &numbered_rank);
            for (square, x) in zip(row, 0..=7) {
                let coords = Coords::new(x, y);
                let mut square_str = (&*Self::get_piece_string_from_option(&square)).normal();
                if special_moves.contains(&coords){
                    print!("|{}", square_str.on_green());
                    continue;
                }
                if attacked_pieces_squares.contains(&coords){
                    print!("|{}", square_str.on_red());
                    continue;
                }
                if attacked_squares.contains(&coords){
                    print!("|{}", square_str.on_bright_red());
                    continue;
                }
                if moves.contains(&coords){
                    print!("|{}", square_str.on_blue());
                    continue;
                }
                print!("|{}", square_str);    // TODO Figure out the right way to do this
            }
            println!("|");

            numbered_rank -= 1;
        }
    }

    pub fn print_board_with_ranks_and_files_with_moves(&self) {
        let moves = self.get_moves();

        print!("  ");
        for letter in b'A'..=b'H'{
            print!(" {} ", letter as char);
        }
        println!();

        let mut numbered_rank = 8;
        for (row, y) in zip(self.board.clone(), 0..=7) {
            println!("  {}", "-".repeat((&self.size.w * 3 + 1) as usize));
            print!("{} ", &numbered_rank);
            for (square, x) in zip(row, 0..=7) {
                let coords = Coords::new(x, y);
                let mut square_str = &*Self::get_piece_string_from_option(&square);
                if moves.contains(&coords){
                    print!("|{}", square_str.on_blue());
                }
                else {
                    print!("|{}", square_str);    // TODO Figure out the right way to do this
                }
            }
            println!("|");

            numbered_rank -= 1;
        }
    }

    pub fn print_board_with_ranks_and_files_with_special_moves(&self) {
        let moves = self.get_all_special_moves();

        print!("  ");
        for letter in b'A'..=b'H'{
            print!(" {} ", letter as char);
        }
        println!();

        let mut numbered_rank = 8;
        for (row, y) in zip(self.board.clone(), 0..=7) {
            println!("  {}", "-".repeat((&self.size.w * 3 + 1) as usize));
            print!("{} ", &numbered_rank);
            for (square, x) in zip(row, 0..=7) {
                let coords = Coords::new(x, y);
                let mut square_str = &*Self::get_piece_string_from_option(&square);
                if moves.contains(&coords){
                    print!("|{}", square_str.on_green());
                }
                else {
                    print!("|{}", square_str);    // TODO Figure out the right way to do this
                }
            }
            println!("|");

            numbered_rank -= 1;
        }
    }

    pub fn print_board_with_ranks_and_files_with_attacked_squares(&self) {
        let moves = self.get_all_attacked_squares();

        print!("  ");
        for letter in b'A'..=b'H'{
            print!(" {} ", letter as char);
        }
        println!();

        let mut numbered_rank = 8;
        for (row, y) in zip(self.board.clone(), 0..=7) {
            println!("  {}", "-".repeat((&self.size.w * 3 + 1) as usize));
            print!("{} ", &numbered_rank);
            for (square, x) in zip(row, 0..=7) {
                let coords = Coords::new(x, y);
                let mut square_str = &*Self::get_piece_string_from_option(&square);
                if moves.contains(&coords){
                    print!("|{}", square_str.on_bright_red());
                }
                else {
                    print!("|{}", square_str);    // TODO Figure out the right way to do this
                }
            }
            println!("|");

            numbered_rank -= 1;
        }
    }

    pub fn print_board_with_ranks_and_files_with_attacked_pieces_squares(&self) {
        let moves = self.get_all_attacked_pieces_squares();

        print!("  ");
        for letter in b'A'..=b'H'{
            print!(" {} ", letter as char);
        }
        println!();

        let mut numbered_rank = 8;
        for (row, y) in zip(self.board.clone(), 0..=7) {
            println!("  {}", "-".repeat((&self.size.w * 3 + 1) as usize));
            print!("{} ", &numbered_rank);
            for (square, x) in zip(row, 0..=7) {
                let coords = Coords::new(x, y);
                let mut square_str = &*Self::get_piece_string_from_option(&square);
                if moves.contains(&coords){
                    print!("|{}", square_str.on_red());
                }
                else {
                    print!("|{}", square_str);    // TODO Figure out the right way to do this
                }
            }
            println!("|");

            numbered_rank -= 1;
        }
    }

    pub fn get_piece_string_from_option(square: &Option<Piece>) -> String {
        if square.is_some() {square.as_ref().unwrap().emoji.to_string()} else {"  ".to_string()}
    }

    pub fn get_piece_option_with_coords(&self, x:usize, y:usize) -> Option<Piece> {
        self.board[y][x].clone()
    }

    /// Deselects (Converts option to None) current selected piece
    pub fn deselect(&mut self){
        self.selected_piece = None;
    }

    /// Selects piece INDEPENDENT of turn
    pub fn force_select_piece_with_coords(&mut self, x:usize, y:usize) {
        self.selected_piece = self.get_piece_option_with_coords(x, y);
    }

    /// Selects piece DEPENDENT of turn
    pub fn select_piece_with_coords(&mut self, x:usize, y:usize) {
        if self.get_piece_option_with_coords(x, y).is_some(){
            if self.turn == self.get_piece_option_with_coords(x, y).as_ref().unwrap().color{
                self.selected_piece = self.get_piece_option_with_coords(x, y);
            }
        }
    }

    // TODO: Fix scuffed moving of elements from one vec position to another
    /// Force moves by internal coordinates INDEPENDENT of turn turn to any square.
    /// Moves with structs named 'from' (Coords) and 'to' (Coords)
    pub fn force_move_piece_with_coords(&mut self, from: Coords, to:Coords) {
        if self.board[from.y][from.x].is_some(){
            // TODO: Fix creating of new object instead of moving existing
            // self.board[to.y][to.x] = Some(Piece::new(
            //     self.board[from.y][from.x].as_ref().unwrap().piece_type,
            //     self.board[from.y][from.x].as_ref().unwrap().color,
            //     to));

            self.add_piece_with_coords(to, Some(Piece::new(
                self.board[from.y][from.x].as_ref().unwrap().piece_type,
                self.board[from.y][from.x].as_ref().unwrap().color,
                to)));

            // self.board[to.y][to.x] = self.board[from.y][from.x].clone();
            // self.board[to.y][to.x].unwrap().coords.x = 0;             // How to fix?

            self.board[from.y][from.x] = None;
        }
    }

    /// Force moves selected piece by internal coordinates INDEPENDENT of turn to any square.
    /// Moves to given x (usize) and y (usize)
    pub fn force_move_selected_piece_with_coords(&mut self, x:usize, y:usize) {
        if self.selected_piece.is_some(){
            self.force_move_piece_with_coords(
                self.selected_piece.as_ref().unwrap().coords,
                Coords{x, y}
            );
            self.selected_piece = None;
        }
    }

    /// Force plays by internal coordinates DEPENDENT of turn to any square.
    /// Moves with structs named 'from' (Coords) and 'to' (Coords)
    pub fn force_play_with_coords(&mut self, from: Coords, to:Coords) {
        if self.board[from.y][from.x].is_some() {
            if self.board[from.y][from.x].as_ref().unwrap().color == self.turn {
                self.force_move_piece_with_coords(from, to);

                // TODO: Should be fixed
                match self.turn {
                    Colors::White => self.turn = Colors::Black,
                    Colors::Black => self.turn = Colors::White,
                }
            }
        }
    }

    /// Force plays selected piece by internal coordinates DEPENDENT of turn to any square.
    /// Moves to given x (usize) and y (usize)
    pub fn force_play_selected_piece_with_coords(&mut self, x:usize, y:usize) {
        if self.selected_piece.is_some() {
            if self.selected_piece.as_ref().unwrap().color == self.turn {
                self.force_move_selected_piece_with_coords(x, y);

                // TODO: Should be fixed
                match self.turn {
                    Colors::White => self.turn = Colors::Black,
                    Colors::Black => self.turn = Colors::White,
                }
            }
        }
    }

    pub fn get_coords_from_notation(&self, square: &str) -> Coords {
        if square.len() != 2 {
            panic!("Length of square str should be 2, in the format of LetterNumber \
            (where letter is a-h and number is 1-8), e.g a1 or h8")
        }

        let file = square.chars().nth(0).unwrap().to_ascii_lowercase();

        if !('a'..='h').contains(&file) {
            panic!("Wrong format! File should be a-h!")
        }

        let rank = square.chars().nth(1).unwrap().to_digit(9).unwrap() as usize;

        if Size::check_width(rank) {
            panic!("Wrong format! Rank should be 1-8!")
        }

        let mut coords = Coords::new(0, 0);

        // println!("{} {}", file, rank);

        for (file_letter, file_x) in zip('a'..='h', 0..=7){
            // println!("{} {}", file_letter, file_x);
            if file == file_letter{
                coords.x = file_x;
                break;
            }
        };

        for (rank_number, rank_x) in zip((1..=8).rev(), 0..=7){
            // println!("{} {}", rank_number, rank_x);
            if rank == rank_number{
                coords.y = rank_x;
                break;
            }
        };

        coords
    }

    /// Selects piece INDEPENDENT of turn, using algebraic notation (chess notation)
    pub fn force_select_piece_with_notation(&mut self, square: &str){
        let coords = self.get_coords_from_notation(square);
        self.selected_piece = self.get_piece_option_with_coords(coords.x, coords.y);
    }

    /// Selects piece DEPENDENT of turn, using algebraic notation (chess notation)
    pub fn select_piece_notation(&mut self, square: &str){
        let coords = self.get_coords_from_notation(square);
        if self.turn == self.get_piece_option_with_coords(coords.x, coords.y).as_ref().unwrap().color{
            self.selected_piece = self.get_piece_option_with_coords(coords.x, coords.y);
        }
    }

    pub fn get_piece_option_with_notation(&self, square: &str) -> Option<Piece> {
        let coords = self.get_coords_from_notation(square);
        self.get_piece_option_with_coords(coords.x, coords.y)
    }

    /// Force moves by algebraic notation (chess notation) INDEPENDENT of turn turn to any square.
    /// Moves with structs named 'from' (&str) and 'to' (&str)
    pub fn force_move_piece_with_notation(&mut self, from: &str, to:&str) {
        let from_coords = self.get_coords_from_notation(from);
        let to_coords = self.get_coords_from_notation(to);

        self.force_move_piece_with_coords(from_coords, to_coords);
    }

    /// Force moves selected piece by algebraic notation (chess notation)
    /// INDEPENDENT of turn to any square.
    pub fn force_move_selected_piece_notation(&mut self, square: &str) {
        let coords = self.get_coords_from_notation(square);

        self.force_move_selected_piece_with_coords(coords.x, coords.y);
    }

    /// Force plays by algebraic notation (chess notation) DEPENDENT of turn to any square.
    /// Moves with structs named 'from' (&str) and 'to' (&str)
    pub fn force_play_with_notation(&mut self, from: &str, to:&str){
        let from_coords = self.get_coords_from_notation(from);
        let to_coords = self.get_coords_from_notation(to);

        self.force_play_with_coords(from_coords, to_coords);
    }

    /// Force plays selected piece algebraic notation (chess notation) coordinates
    /// DEPENDENT of turn to any square.
    pub fn force_play_selected_piece_with_notation(&mut self, square: &str){
        let coords = self.get_coords_from_notation(square);

        self.force_play_selected_piece_with_coords(coords.x, coords.y);
    }

    pub fn add_piece_with_coords(&mut self, coords: Coords, piece_option: Option<Piece>) {
        self.board[coords.y][coords.x] = piece_option;
    }

    pub fn add_piece_with_notation(&mut self, square: &str, piece_option: Option<Piece>) {
        let coords = self.get_coords_from_notation(square);
        self.board[coords.y][coords.x] = piece_option;
    }

    fn get_all_legal_moves(&self) -> HashSet<Coords> {
        let mut moves = HashSet::new();

        for row in self.board.clone() {
            for square in row {
                if square.is_some() {
                    for piece_move in square.unwrap().get_all_legal_moves(self.board.clone()){
                        moves.insert(piece_move);
                    }
                }
            }
        }

        moves
    }

    fn get_moves(&self) -> HashSet<Coords> {
        let mut moves = HashSet::new();

        for row in self.board.clone() {
            for square in row {
                if square.is_some() {
                    for piece_move in square.unwrap().get_moves(self.board.clone()){
                        moves.insert(piece_move);
                    }
                }
            }
        }

        moves
    }

    fn get_all_special_moves(&self) -> HashSet<Coords> {
        let mut moves = HashSet::new();

        for row in self.board.clone() {
            for square in row {
                if square.is_some() {
                    for piece_move in square.unwrap().get_special_moves(self.board.clone()){
                        moves.insert(piece_move);
                    }
                }
            }
        }

        moves
    }

    fn get_all_attacked_squares(&self) -> HashSet<Coords> {
        let mut moves = HashSet::new();

        for row in self.board.clone() {
            for square in row {
                if square.is_some() {
                    for piece_move in square.unwrap().get_attacked_squares(self.board.clone()){
                        moves.insert(piece_move);
                    }
                }
            }
        }

        moves
    }

    fn get_all_attacked_pieces_squares(&self) -> HashSet<Coords> {
        let mut moves = HashSet::new();

        for row in self.board.clone() {
            for square in row {
                if square.is_some() {
                    for piece_move in square.unwrap().get_attacked_pieces_squares(self.board.clone()){
                        moves.insert(piece_move);
                    }
                }
            }
        }

        moves
    }

    /// Plays (following rules) by algebraic notation (chess notation) DEPENDENT of turn to any square.
    /// Moves variables named 'from' (&str) and 'to' (&str)
    pub fn play_with_notation(&mut self, from: &str, to:&str) {
        let from_coords = self.get_coords_from_notation(from);
        let to_coords = self.get_coords_from_notation(to);

        let piece = self.get_piece_option_with_coords(from_coords.x, from_coords.y);

        if piece.as_ref().is_some() {
            if self.turn == piece.clone().unwrap().color {
                if piece.clone().unwrap().get_all_legal_moves(self.board.clone()).contains(&to_coords) {
                    if !Self::is_checked(self.board.clone(),
                                         self.turn,
                                         from_coords.clone(),
                                         to_coords.clone()) {
                        self.force_play_with_coords(from_coords, to_coords);
                        self.update_status();
                    }
                }
            }
        }
    }

    /// Plays (following rules) selected piece algebraic notation (chess notation) coordinates
    /// DEPENDENT of turn to any square.
    pub fn play_selected_piece_with_notation(&mut self, square: &str) {
        if self.selected_piece.is_some(){
            if self.turn == self.selected_piece.as_ref().unwrap().color{
                let piece = self.selected_piece.as_ref().unwrap().clone();
                let coords = self.get_coords_from_notation(square);

                if piece.get_all_legal_moves(self.board.clone()).contains(&coords) {
                    if !Self::is_checked(self.board.clone(),
                                         self.turn,
                                         self.selected_piece.as_ref().unwrap().coords.clone(),
                                         coords.clone()) {
                        self.force_play_selected_piece_with_coords(coords.x, coords.y);
                        self.update_status();
                    }
                }
            }
        }
    }

    /// Returns a CLONED board
    pub fn get_board(&self) -> Vec<Vec<Option<Piece>>>{
        self.board.clone()
    }

    // TODO: Refactor, bad repeating code, need better solution
    pub fn is_checked(mut board: Vec<Vec<Option<Piece>>>,
                      checked_color: Colors,
                      from: Coords,
                      to: Coords) -> bool {
        board[to.y][to.x] = Some(Piece::new(
            board[from.y][from.x].as_ref().unwrap().piece_type,
            board[from.y][from.x].as_ref().unwrap().color,
            to));

        board[from.y][from.x] = None;

        let mut king = None;
        for row in board.clone() {
            for square in row {
                if square.as_ref().is_some() {
                    if square.clone().unwrap().color == checked_color &&
                        square.clone().unwrap().piece_type == PieceTypes::King {
                        // println!("{:?}", square.clone().unwrap().coords);
                        king = square;
                    }
                }
            }
        }

        if king.is_none() {
            // println!("No king!");
            return false
        }

        let mut attacked_pieces = HashSet::new();

        for row in board.clone() {
            for square in row {
                if square.is_some() {
                    for piece_move in square.unwrap().get_attacked_pieces_squares(board.clone()){
                        attacked_pieces.insert(piece_move);
                    }
                }
            }
        }

        attacked_pieces.contains(&king.unwrap().coords)
    }

    fn self_is_checked(&self) -> bool {
        let mut king = None;
        for row in self.board.clone() {
            for square in row {
                if square.as_ref().is_some() {
                    if square.clone().unwrap().color == self.turn &&
                        square.clone().unwrap().piece_type == PieceTypes::King {
                        // println!("{:?}", square.clone().unwrap().coords);
                        king = square;
                    }
                }
            }
        }

        if king.is_none() {
            // println!("No king!");
            return false
        }

        let mut attacked_pieces = HashSet::new();

        for row in self.board.clone() {
            for square in row {
                if square.is_some() {
                    for piece_move in square.unwrap().get_attacked_pieces_squares(self.board.clone()){
                        attacked_pieces.insert(piece_move);
                    }
                }
            }
        }

        attacked_pieces.contains(&king.unwrap().coords)
    }

    pub fn update_status(&mut self) {
        if self.self_is_checked() {
            self.status = Checked;
        }
        else {
            self.status = Normal;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

        chess_engine.print_board_with_ranks_and_files();

    }

    //TODO: Need to fix tests to compare to board states (or worst case strings)
    #[test]
    fn print_filled_board_with_ranks() {
        let chess_engine = ChessEngine::create_engine_filled_with_white_board();

        chess_engine.print_board_with_ranks_and_files();
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

        chess_engine.print_board_with_ranks_and_files();
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
    fn test_get_piece_string() {
        let mut chess_engine = ChessEngine::new();

        let piece_string = ChessEngine::get_piece_string_from_option(
            &chess_engine.get_piece_option_with_coords(7, 7)
        );

        println!("{}", piece_string);

        assert_eq!(piece_string, "♖ ");
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
            Coords::new(7, 7),
            Coords::new( 3, 3)
        );
    }

    #[test]
    fn test_piece_move_to_occupied_square() {
        let mut chess_engine = ChessEngine::new();

        chess_engine = chess_engine_move_piece_from_to_assert(
            chess_engine,
            Coords::new(7, 7),
            Coords::new(7, 0)
        );

        chess_engine = chess_engine_move_piece_from_to_assert(
            chess_engine,
            Coords::new(7, 0),
            Coords::new(3, 3)
        );

        // chess_engine.print_board_with_ranks();

    }

    #[test]
    fn test_empty_move_to_empty_square() {
        let mut chess_engine = ChessEngine::new();

        chess_engine = chess_engine_move_piece_from_to_assert(
            chess_engine,
            Coords::new(3, 3),
            Coords::new( 4, 4)
        );

    }

    #[test]
    fn test_empty_move_to_occupied_square() {
        let mut chess_engine = ChessEngine::new();

        chess_engine = chess_engine_move_piece_from_to_assert(
            chess_engine,
            Coords::new(3, 3),
            Coords::new( 7, 7)
        );
    }

    #[test]
    fn test_force_selection() {
        let mut chess_engine = ChessEngine::new();

        let coords = Coords::new(7, 7);

        chess_engine.force_select_piece_with_coords(coords.x, coords.y);

        // println!("{}", ChessEngine::get_piece_string_from_option(&chess_engine.selected_piece));

        assert_eq!(chess_engine.selected_piece,
                   chess_engine.get_piece_option_with_coords(coords.x, coords.y));
    }

    #[test]
    fn test_force_selection_and_moving() {
        let mut chess_engine = ChessEngine::new();

        let from = Coords::new(7, 7);
        let to = Coords::new(7, 0);

        chess_engine.force_select_piece_with_coords(from.x, from.y);

        chess_engine.force_move_selected_piece_with_coords(to.x, to.y);

        assert_eq!(chess_engine.get_piece_option_with_coords(from.x, from.y), None);
        assert_eq!(chess_engine.get_piece_option_with_coords(to.x, to.y), Some(Piece::new(
            PieceTypes::Rook, Colors::White, to)));

        let from = Coords::new(7, 0);
        let to = Coords::new(3, 3);

        chess_engine.force_select_piece_with_coords(from.x, from.y);

        chess_engine.force_move_selected_piece_with_coords(to.x, to.y);

        assert_eq!(chess_engine.get_piece_option_with_coords(from.x, from.y), None);
        assert_eq!(chess_engine.get_piece_option_with_coords(to.x, to.y), Some(Piece::new(
            PieceTypes::Rook, Colors::White, to)));

        // chess_engine.print_board_with_ranks();
        // println!("{:?}", chess_engine.get_piece_option_with_coords(from.x, from.y));
        // println!("{:?}", chess_engine.get_piece_option_with_coords(to.x, to.y));
        // println!("{:?}", chess_engine.selected_piece);
    }

    #[test]
    fn test_force_play_to_empty_one_move() {
        let mut chess_engine = ChessEngine::new();

        let from = Coords::new(7, 7);
        let to = Coords::new(7, 3);

        // chess_engine.print_board_with_ranks();

        chess_engine.force_play_with_coords(from, to);

        assert_eq!(chess_engine.get_piece_option_with_coords(from.x, from.y), None);
        assert_eq!(chess_engine.get_piece_option_with_coords(to.x, to.y), Some(Piece::new(
            PieceTypes::Rook, Colors::White, to)));

        // chess_engine.print_board_with_ranks();
    }

    // TODO: Use panics when wrong moves?
    #[test]
    fn test_force_play_to_empty_one_move_wrong_turn() {
        let mut chess_engine = ChessEngine::new();

        let from = Coords::new(7, 0);
        let to = Coords::new(7, 3);

        // chess_engine.print_board_with_ranks();

        chess_engine.force_play_with_coords(from, to);

        assert_eq!(chess_engine.get_piece_option_with_coords(from.x, from.y),
                   Some(Piece::new(PieceTypes::Rook, Colors::Black, from)));
        assert_eq!(chess_engine.get_piece_option_with_coords(to.x, to.y), None);

        // chess_engine.print_board_with_ranks();
    }

    #[test]
    fn test_force_play_to_occupied_one_move() {
        let mut chess_engine = ChessEngine::new();

        let from = Coords::new(7, 7);
        let to = Coords::new(7, 0);

        // chess_engine.print_board_with_ranks();

        chess_engine.force_play_with_coords(from, to);

        assert_eq!(chess_engine.get_piece_option_with_coords(from.x, from.y), None);
        assert_eq!(chess_engine.get_piece_option_with_coords(to.x, to.y), Some(Piece::new(
            PieceTypes::Rook, Colors::White, to)));

        // chess_engine.print_board_with_ranks();
    }

    // TODO: Use panics when wrong moves?
    #[test]
    fn test_force_play_to_occupied_one_move_wrong_turn() {
        let mut chess_engine = ChessEngine::new();

        let from = Coords::new(7, 0);
        let to = Coords::new(7, 7);

        // chess_engine.print_board_with_ranks();

        chess_engine.force_play_with_coords(from, to);

        assert_eq!(chess_engine.get_piece_option_with_coords(from.x, from.y),
                   Some(Piece::new(PieceTypes::Rook, Colors::Black, from)));
        assert_eq!(chess_engine.get_piece_option_with_coords(to.x, to.y),
                   Some(Piece::new(PieceTypes::Rook, Colors::White, to)));

        // chess_engine.print_board_with_ranks();
    }

    #[test]
    fn test_force_play_two_moves() {
        let mut chess_engine = ChessEngine::new();

        let from = Coords::new(7, 7);
        let to = Coords::new(7, 0);

        // chess_engine.print_board_with_ranks();

        chess_engine.force_play_with_coords(from, to);

        assert_eq!(chess_engine.get_piece_option_with_coords(from.x, from.y), None);
        assert_eq!(chess_engine.get_piece_option_with_coords(to.x, to.y), Some(Piece::new(
            PieceTypes::Rook, Colors::White, to)));
        assert_eq!(chess_engine.turn, Colors::Black);

        // chess_engine.print_board_with_ranks();

        let from = Coords::new(0, 0);
        let to = Coords::new(0, 7);

        chess_engine.force_play_with_coords(from, to);

        // chess_engine.print_board_with_ranks();

        assert_eq!(chess_engine.get_piece_option_with_coords(from.x, from.y), None);
        assert_eq!(chess_engine.get_piece_option_with_coords(to.x, to.y), Some(Piece::new(
            PieceTypes::Rook, Colors::Black, to)));
        assert_eq!(chess_engine.turn, Colors::White);
    }

    // TODO: Use panics when wrong moves?
    #[test]
    fn test_force_play_two_moves_wrong_turn() {
        let mut chess_engine = ChessEngine::new();

        let from = Coords::new(7, 7);
        let to = Coords::new(7, 0);

        // chess_engine.print_board_with_ranks();

        chess_engine.force_play_with_coords(from, to);

        assert_eq!(chess_engine.get_piece_option_with_coords(from.x, from.y), None);
        assert_eq!(chess_engine.get_piece_option_with_coords(to.x, to.y), Some(Piece::new(
            PieceTypes::Rook, Colors::White, to)));
        assert_eq!(chess_engine.turn, Colors::Black);

        // chess_engine.print_board_with_ranks();

        let from = Coords::new(7, 0);
        let to = Coords::new(7, 7);

        chess_engine.force_play_with_coords(from, to);

        // chess_engine.print_board_with_ranks();

        assert_eq!(chess_engine.get_piece_option_with_coords(from.x, from.y), Some(Piece::new(
            PieceTypes::Rook, Colors::White, from)));
        assert_eq!(chess_engine.get_piece_option_with_coords(to.x, to.y), None);
        assert_eq!(chess_engine.turn, Colors::Black);
    }

    #[test]
    fn test_selection_with_coords_occupied() {
        let mut chess_engine = ChessEngine::new();

        let coords = Coords::new(7, 7);

        chess_engine.select_piece_with_coords(coords.x, coords.y);

        assert_eq!(chess_engine.get_piece_option_with_coords(coords.x, coords.y),
                   chess_engine.selected_piece);
    }

    #[test]
    fn test_selection_with_coords_empty() {
        let mut chess_engine = ChessEngine::new();

        let coords = Coords::new(3, 3);

        chess_engine.select_piece_with_coords(coords.x, coords.y);

        assert_eq!(chess_engine.get_piece_option_with_coords(coords.x, coords.y),
                   chess_engine.selected_piece);
    }

    #[test]
    fn test_selection_with_coords_two_times() {
        let mut chess_engine = ChessEngine::new();

        let coords = Coords::new(7, 7);

        chess_engine.select_piece_with_coords(coords.x, coords.y);

        assert_eq!(chess_engine.get_piece_option_with_coords(coords.x, coords.y),
                   chess_engine.selected_piece);

        let coords = Coords::new(0, 7);

        chess_engine.select_piece_with_coords(coords.x, coords.y);

        assert_eq!(chess_engine.get_piece_option_with_coords(coords.x, coords.y),
                   chess_engine.selected_piece);

        // println!("{:?}", chess_engine.selected_piece);

    }

    #[test]
   fn test_deselect_occupied() {
        let mut chess_engine = ChessEngine::new();

        let coords = Coords::new(7, 7);

        chess_engine.select_piece_with_coords(coords.x, coords.y);

        assert_eq!(chess_engine.selected_piece,
                   Some(Piece::new(PieceTypes::Rook, Colors::White, coords)));

        chess_engine.deselect();

        assert_eq!(chess_engine.selected_piece, None);

    }

    #[test]
    fn test_deselect_empty() {
        let mut chess_engine = ChessEngine::new();

        let coords = Coords::new(3, 3);

        chess_engine.select_piece_with_coords(coords.x, coords.y);

        assert_eq!(chess_engine.selected_piece, None);

        chess_engine.deselect();

        assert_eq!(chess_engine.selected_piece, None);

    }

    #[test]
    fn test_force_play_selected_to_empty_one_move() {
        let mut chess_engine = ChessEngine::new();

        let from = Coords::new(7, 7);
        let to = Coords::new(7, 3);

        // chess_engine.print_board_with_ranks();

        chess_engine.select_piece_with_coords(from.x, from.y);

        assert_eq!(chess_engine.selected_piece, Some(Piece::new(
            PieceTypes::Rook, Colors::White, from)));
        assert_eq!(chess_engine.turn, Colors::White);

        chess_engine.force_play_selected_piece_with_coords(to.x, to.y);

        assert_eq!(chess_engine.get_piece_option_with_coords(from.x, from.y), None);
        assert_eq!(chess_engine.selected_piece, None);
        assert_eq!(chess_engine.get_piece_option_with_coords(to.x, to.y),
                   Some(Piece::new(PieceTypes::Rook, Colors::White, to)));
        assert_eq!(chess_engine.turn, Colors::Black);

        // chess_engine.print_board_with_ranks();
    }

    // TODO: Use panics when wrong moves?
    #[test]
    fn test_force_play_selected_to_empty_one_move_wrong_turn() {
        let mut chess_engine = ChessEngine::new();

        let from = Coords::new(7, 0);
        let to = Coords::new(7, 3);

        // chess_engine.print_board_with_ranks();

        chess_engine.select_piece_with_coords(from.x, from.y);

        assert_eq!(chess_engine.selected_piece, None);
        assert_eq!(chess_engine.turn, Colors::White);

        chess_engine.force_play_selected_piece_with_coords(to.x, to.y);

        assert_eq!(chess_engine.get_piece_option_with_coords(to.x, to.y), None);
        assert_eq!(chess_engine.selected_piece, None);
        assert_eq!(chess_engine.get_piece_option_with_coords(to.x, to.y), None);
        assert_eq!(chess_engine.turn, Colors::White);

        // chess_engine.print_board_with_ranks();
    }

    #[test]
    fn test_force_play_selected_to_occupied_one_move() {
        let mut chess_engine = ChessEngine::new();

        let from = Coords::new(7, 7);
        let to = Coords::new(7, 0);

        // chess_engine.print_board_with_ranks();

        chess_engine.select_piece_with_coords(from.x, from.y);

        assert_eq!(chess_engine.selected_piece,
                   Some(Piece::new(PieceTypes::Rook, Colors::White, from)));
        assert_eq!(chess_engine.turn, Colors::White);

        chess_engine.force_play_selected_piece_with_coords(to.x, to.y);

        assert_eq!(chess_engine.get_piece_option_with_coords(from.x, from.y), None);
        assert_eq!(chess_engine.selected_piece, None);
        assert_eq!(chess_engine.get_piece_option_with_coords(to.x, to.y),
                   Some(Piece::new(PieceTypes::Rook, Colors::White, to)));
        assert_eq!(chess_engine.turn, Colors::Black);

        // chess_engine.print_board_with_ranks();
    }

    // TODO: Use panics when wrong moves?
    #[test]
    fn test_force_play_selected_to_occupied_one_move_wrong_turn() {
        let mut chess_engine = ChessEngine::new();

        let from = Coords::new(7, 0);
        let to = Coords::new(7, 7);

        // chess_engine.print_board_with_ranks();

        chess_engine.select_piece_with_coords(from.x, from.y);

        assert_eq!(chess_engine.selected_piece, None);
        assert_eq!(chess_engine.turn, Colors::White);

        chess_engine.force_play_selected_piece_with_coords(to.x, to.y);

        assert_eq!(chess_engine.get_piece_option_with_coords(from.x, from.y),
                   Some(Piece::new(PieceTypes::Rook, Colors::Black, from)));
        assert_eq!(chess_engine.get_piece_option_with_coords(to.x, to.y),
                   Some(Piece::new(PieceTypes::Rook, Colors::White, to)));
        assert_eq!(chess_engine.turn, Colors::White);

        // chess_engine.print_board_with_ranks();
    }

    #[test]
    fn test_force_play_selected_two_moves() {
        let mut chess_engine = ChessEngine::new();

        let from = Coords::new(7, 7);
        let to = Coords::new(7, 0);

        // chess_engine.print_board_with_ranks();

        chess_engine.select_piece_with_coords(from.x, from.y);

        assert_eq!(chess_engine.selected_piece,
                   Some(Piece::new(PieceTypes::Rook, Colors::White, from)));
        assert_eq!(chess_engine.turn, Colors::White);

        chess_engine.force_play_selected_piece_with_coords(to.x, to.y);

        assert_eq!(chess_engine.get_piece_option_with_coords(from.x, from.y), None);
        assert_eq!(chess_engine.selected_piece, None);
        assert_eq!(chess_engine.get_piece_option_with_coords(to.x, to.y), Some(Piece::new(
            PieceTypes::Rook, Colors::White, to)));
        assert_eq!(chess_engine.turn, Colors::Black);

        // chess_engine.print_board_with_ranks();

        let from = Coords::new(0, 0);
        let to = Coords::new(0, 7);

        chess_engine.select_piece_with_coords(from.x, from.y);

        assert_eq!(chess_engine.selected_piece,
                   Some(Piece::new(PieceTypes::Rook, Colors::Black, from)));
        assert_eq!(chess_engine.turn, Colors::Black);

        chess_engine.force_play_selected_piece_with_coords(to.x, to.y);

        // chess_engine.print_board_with_ranks();

        assert_eq!(chess_engine.get_piece_option_with_coords(from.x, from.y), None);
        assert_eq!(chess_engine.selected_piece, None);
        assert_eq!(chess_engine.get_piece_option_with_coords(to.x, to.y), Some(Piece::new(
            PieceTypes::Rook, Colors::Black, to)));
        assert_eq!(chess_engine.turn, Colors::White);
    }

    // TODO: Use panics when wrong moves?
    #[test]
    fn test_force_play_selected_two_moves_wrong_turn() {
        let mut chess_engine = ChessEngine::new();

        let from = Coords::new(7, 7);
        let to = Coords::new(7, 0);

        // chess_engine.print_board_with_ranks();

        chess_engine.select_piece_with_coords(from.x, from.y);

        assert_eq!(chess_engine.selected_piece,
                   Some(Piece::new(PieceTypes::Rook, Colors::White, from)));
        assert_eq!(chess_engine.turn, Colors::White);

        chess_engine.force_play_selected_piece_with_coords(to.x, to.y);

        assert_eq!(chess_engine.get_piece_option_with_coords(from.x, from.y), None);
        assert_eq!(chess_engine.selected_piece, None);
        assert_eq!(chess_engine.get_piece_option_with_coords(to.x, to.y),
                   Some(Piece::new(PieceTypes::Rook, Colors::White, to)));
        assert_eq!(chess_engine.turn, Colors::Black);

        // chess_engine.print_board_with_ranks();

        let from = Coords::new(7, 0);
        let to = Coords::new(7, 7);

        chess_engine.select_piece_with_coords(from.x, from.y);

        assert_eq!(chess_engine.selected_piece, None);
        assert_eq!(chess_engine.turn, Colors::Black);

        chess_engine.force_play_selected_piece_with_coords(to.x, to.y);

        // chess_engine.print_board_with_ranks();

        assert_eq!(chess_engine.get_piece_option_with_coords(from.x, from.y),
                   Some(Piece::new(PieceTypes::Rook, Colors::White, from)));
        assert_eq!(chess_engine.selected_piece, None);
        assert_eq!(chess_engine.get_piece_option_with_coords(to.x, to.y), None);
        assert_eq!(chess_engine.turn, Colors::Black);
    }

    #[test]
    fn test_get_coords_from_notation_h1() {
        let mut chess_engine = ChessEngine::new();

        let coords = chess_engine.get_coords_from_notation("h1");

        // println!("{:?}", coords);

        assert_eq!(Coords::new(7, 7), coords);
    }

    #[test]
    fn test_get_coords_from_notation_a8() {
        let mut chess_engine = ChessEngine::new();

        let coords = chess_engine.get_coords_from_notation("a8");

        // println!("{:?}", coords);

        assert_eq!(Coords::new(0, 0), coords);
    }

    #[test]
    fn test_get_coords_from_notation_e4() {
        let mut chess_engine = ChessEngine::new();

        let coords = chess_engine.get_coords_from_notation("e4");

        // println!("{:?}", coords);

        assert_eq!(Coords::new(4, 4), coords);
    }

    #[test]
    #[should_panic]
    fn test_get_coords_from_notation_i5_should_panic() {
        let mut chess_engine = ChessEngine::new();

        chess_engine.get_coords_from_notation("i5");

    }

    #[test]
    #[should_panic]
    fn test_get_coords_from_notation_a0_should_panic() {
        let mut chess_engine = ChessEngine::new();

        chess_engine.get_coords_from_notation("a0");

    }

    #[test]
    #[should_panic]
    fn test_get_coords_from_notation_a9_should_panic() {
        let mut chess_engine = ChessEngine::new();

        chess_engine.get_coords_from_notation("a9");

    }

    #[test]
    #[should_panic]
    fn test_get_coords_from_notation_99_should_panic() {
        let mut chess_engine = ChessEngine::new();

        chess_engine.get_coords_from_notation("99");

    }

    #[test]
    #[should_panic]
    fn test_get_coords_from_notation_aa_should_panic() {
        let mut chess_engine = ChessEngine::new();

        chess_engine.get_coords_from_notation("aa");

    }

    #[test]
    #[should_panic]
    fn test_get_coords_from_notation_a11_should_panic() {
        let mut chess_engine = ChessEngine::new();

        chess_engine.get_coords_from_notation("a11");

    }

    #[test]
    #[should_panic]
    fn test_get_coords_from_notation_empty_str_should_panic() {
        let mut chess_engine = ChessEngine::new();

        chess_engine.get_coords_from_notation("");

    }

    #[test]
    #[should_panic]
    fn test_get_coords_from_notation_a_should_panic() {
        let mut chess_engine = ChessEngine::new();

        chess_engine.get_coords_from_notation("a");

    }

    #[test]
    #[should_panic]
    fn test_get_coords_from_notation_unsupported_types_should_panic() {
        let mut chess_engine = ChessEngine::new();

        chess_engine.get_coords_from_notation("--");

    }

    #[test]
    fn test_get_piece_with_notation() {
        let mut chess_engine = ChessEngine::new();

        let coords = Coords::new(7, 7);

        assert_eq!(chess_engine.get_piece_option_with_coords(coords.x, coords.y),
                   chess_engine.get_piece_option_with_notation("h1"));

    }

    #[test]
    fn test_select_notation_once() {
        let mut chess_engine = ChessEngine::new();

        let mut s = "h1";

        chess_engine.select_piece_notation(s);

        assert_eq!(chess_engine.selected_piece, chess_engine.get_piece_option_with_notation(s));

    }

    #[test]
    fn test_select_notation_once_wrong_turn() {
        let mut chess_engine = ChessEngine::new();

        let s = "h8";

        chess_engine.select_piece_notation(s);

        assert_eq!(chess_engine.selected_piece, None);
    }

    #[test]
    fn test_force_move_notation() {
        let mut chess_engine = ChessEngine::new();

        let from = "h1";
        let to = "h8";

        chess_engine.force_move_piece_with_notation(from, to);

        assert_eq!(chess_engine.get_piece_option_with_notation(from), None);
        assert_eq!(chess_engine.get_piece_option_with_notation(to),
                   Some(Piece::new(PieceTypes::Rook,
                                   Colors::White,
                                   chess_engine.get_coords_from_notation(to))
                   )
        );
    }

    #[test]
    fn test_force_move_selected_piece_notation() {
        let mut chess_engine = ChessEngine::new();

        let from = "h1";
        let to = "h8";

        chess_engine.select_piece_notation(from);

        assert_eq!(chess_engine.selected_piece, chess_engine.get_piece_option_with_notation(from));

        chess_engine.force_move_selected_piece_notation(to);

        assert_eq!(chess_engine.get_piece_option_with_notation(from), None);
        assert_eq!(chess_engine.selected_piece, None);
        assert_eq!(chess_engine.get_piece_option_with_notation(to),
                   Some(Piece::new(PieceTypes::Rook,
                                   Colors::White,
                                   chess_engine.get_coords_from_notation(to))
                   )
        );
    }

    #[test]
    fn test_force_play_notation() {
        let mut chess_engine = ChessEngine::new();

        let from = "h1";
        let to = "h8";

        chess_engine.force_play_with_notation(from, to);

        assert_eq!(chess_engine.get_piece_option_with_notation(from), None);
        assert_eq!(chess_engine.turn, Colors::Black);
        assert_eq!(chess_engine.get_piece_option_with_notation(to),
                   Some(Piece::new(PieceTypes::Rook,
                                   Colors::White,
                                   chess_engine.get_coords_from_notation(to))
                   )
        );
    }

    #[test]
    fn test_force_play_notation_wrong_turn() {
        let mut chess_engine = ChessEngine::new();

        let from = "h8";
        let to = "h1";

        chess_engine.force_play_with_notation(from, to);

        assert_eq!(chess_engine.get_piece_option_with_notation(from),
                   Some(Piece::new(PieceTypes::Rook,
                                   Colors::Black,
                                   chess_engine.get_coords_from_notation(from))
                   )
        );
        assert_eq!(chess_engine.turn, Colors::White);
        assert_eq!(chess_engine.get_piece_option_with_notation(to),
                   Some(Piece::new(PieceTypes::Rook,
                                   Colors::White,
                                   chess_engine.get_coords_from_notation(to))
                   )
        );
    }

    #[test]
    fn test_force_play_selected_piece_notation() {
        let mut chess_engine = ChessEngine::new();

        let from = "h1";
        let to = "h8";

        chess_engine.select_piece_notation(from);

        assert_eq!(chess_engine.selected_piece,
                   chess_engine.get_piece_option_with_notation(from));

        chess_engine.force_play_selected_piece_with_notation(to);

        assert_eq!(chess_engine.get_piece_option_with_notation(from), None);
        assert_eq!(chess_engine.turn, Colors::Black);
        assert_eq!(chess_engine.get_piece_option_with_notation(to),
                   Some(Piece::new(PieceTypes::Rook,
                                   Colors::White,
                                   chess_engine.get_coords_from_notation(to))
                   )
        );
    }

    #[test]
    fn test_force_play_selected_piece_notation_wrong_turn() {
        let mut chess_engine = ChessEngine::new();

        let from = "h8";
        let to = "h1";

        chess_engine.select_piece_notation(from);

        assert_eq!(chess_engine.selected_piece, None);

        chess_engine.force_play_selected_piece_with_notation(to);

        assert_eq!(chess_engine.get_piece_option_with_notation(from),
                   Some(Piece::new(PieceTypes::Rook,
                                   Colors::Black,
                                   chess_engine.get_coords_from_notation(from))
                   )
        );
        assert_eq!(chess_engine.turn, Colors::White);
        assert_eq!(chess_engine.get_piece_option_with_notation(to),
                   Some(Piece::new(PieceTypes::Rook,
                                   Colors::White,
                                   chess_engine.get_coords_from_notation(to))
                   )
        );
    }

    #[test]
    fn test_add_piece_with_coords() {
        let mut chess_engine = ChessEngine::new();

        chess_engine.add_piece_with_coords(Coords::new(3,3),
                                           Some(Piece::new(
                                               PieceTypes::Knight,
                                               Colors::White,
                                               Coords::new(3, 3)
                                           )));

        assert_eq!(chess_engine.get_piece_option_with_coords(3, 3),
                   Some(Piece::new(
                       PieceTypes::Knight,
                       Colors::White,
                       Coords::new(3, 3))));

        // chess_engine.print_board_with_ranks_and_files()
    }

    #[test]
    #[should_panic]
    fn test_add_piece_with_coords_wrong_square() {
        let mut chess_engine = ChessEngine::new();

        let coords = Coords::new(8,8);

        chess_engine.add_piece_with_coords(coords,
                                           Some(Piece::new(
                                               PieceTypes::Knight,
                                               Colors::White,
                                               coords
                                           )));

        assert_eq!(chess_engine.get_piece_option_with_coords(coords.x, coords.y),
                   Some(Piece::new(
                       PieceTypes::Knight,
                       Colors::White,
                       coords)));

        // chess_engine.print_board_with_ranks_and_files()
    }

    #[test]
    #[should_panic]
    fn test_add_piece_with_notation() {
        let mut chess_engine = ChessEngine::new();

        let square = "f9";

        chess_engine.add_piece_with_notation(square,
                                           Some(Piece::new(
                                               PieceTypes::Knight,
                                               Colors::White,
                                               chess_engine.get_coords_from_notation(square)
                                           )));

        assert_eq!(chess_engine.get_piece_option_with_notation(square),
                   Some(Piece::new(
                       PieceTypes::Knight,
                       Colors::White,
                       chess_engine.get_coords_from_notation(square))));

        // chess_engine.print_board_with_ranks_and_files()
    }

    fn test_play_with_notation_prepare(mut chess_engine: ChessEngine) -> ChessEngine {
        let square = "e3";
        chess_engine.add_piece_with_notation(square, Some(Piece::new(
            PieceTypes::Pawn,
            Colors::White,
            chess_engine.get_coords_from_notation(square))
        ));

        let square = "f4";
        chess_engine.add_piece_with_notation(square, Some(Piece::new(
            PieceTypes::Pawn,
            Colors::Black,
            chess_engine.get_coords_from_notation(square))
        ));

        let square = "g2";
        chess_engine.add_piece_with_notation(square, Some(Piece::new(
            PieceTypes::Knight,
            Colors::White,
            chess_engine.get_coords_from_notation(square))
        ));

        chess_engine.print_board_with_ranks_and_files_with_all_legal_moves_different_colors();

        chess_engine
    }

    // TODO: Real tests, not only prints
    #[test]
    fn test_play_with_notation_move_to_empty_valid() {
        let mut chess_engine = ChessEngine::create_engine_with_empty_board();

        chess_engine = test_play_with_notation_prepare(chess_engine);

        chess_engine.play_with_notation("g2", "e1");

        assert_eq!(chess_engine.turn, Colors::Black);

        chess_engine.print_board_with_ranks_and_files_with_all_legal_moves_different_colors();

    }

    #[test]
    fn test_play_with_notation_move_to_opposite_piece_valid() {
        let mut chess_engine = ChessEngine::create_engine_with_empty_board();

        chess_engine = test_play_with_notation_prepare(chess_engine);

        chess_engine.play_with_notation("g2", "f4");

        assert_eq!(chess_engine.turn, Colors::Black);

        chess_engine.print_board_with_ranks_and_files_with_all_legal_moves_different_colors();

    }

    #[test]
    fn test_play_with_notation_move_to_empty_not_valid() {
        let mut chess_engine = ChessEngine::create_engine_with_empty_board();

        chess_engine = test_play_with_notation_prepare(chess_engine);

        chess_engine.play_with_notation("g2", "e2");

        assert_eq!(chess_engine.turn, Colors::White);

        chess_engine.print_board_with_ranks_and_files_with_all_legal_moves_different_colors();

    }

    #[test]
    fn test_play_with_notation_move_to_own_piece_not_valid() {
        let mut chess_engine = ChessEngine::create_engine_with_empty_board();

        chess_engine = test_play_with_notation_prepare(chess_engine);

        chess_engine.play_with_notation("g2", "e3");

        assert_eq!(chess_engine.turn, Colors::White);

        chess_engine.print_board_with_ranks_and_files_with_all_legal_moves_different_colors();

    }

    #[test]
    fn test_play_with_notation_move_twice_wrong_turn() {
        let mut chess_engine = ChessEngine::create_engine_with_empty_board();

        chess_engine = test_play_with_notation_prepare(chess_engine);

        chess_engine.play_with_notation("g2", "f4");

        assert_eq!(chess_engine.turn, Colors::Black);

        chess_engine.print_board_with_ranks_and_files_with_all_legal_moves_different_colors();

        chess_engine.play_with_notation("f4", "d3");

        assert_eq!(chess_engine.turn, Colors::Black);

        chess_engine.print_board_with_ranks_and_files_with_all_legal_moves_different_colors();

        // chess_engine.play_with_notation("e3", "e2");
        //
        // assert_eq!(chess_engine.turn, Colors::Black);
        //
        // chess_engine.print_board_with_ranks_and_files_with_all_legal_moves_different_colors();

    }

    #[test]
    fn test_play_selected_with_notation_move_to_empty_valid_and_wrong_turn() {
        let mut chess_engine = ChessEngine::create_engine_with_empty_board();

        chess_engine = test_play_with_notation_prepare(chess_engine);

        chess_engine.select_piece_notation("g2");

        chess_engine.play_selected_piece_with_notation("e1");

        assert_eq!(chess_engine.turn, Colors::Black);
        assert_eq!(chess_engine.selected_piece, None);

        chess_engine.print_board_with_ranks_and_files_with_all_legal_moves_different_colors();

        chess_engine.select_piece_notation("e1");

        chess_engine.play_selected_piece_with_notation("d3");

        assert_eq!(chess_engine.turn, Colors::Black);
        assert_eq!(chess_engine.selected_piece, None);

        chess_engine.print_board_with_ranks_and_files_with_all_legal_moves_different_colors();

    }

    #[test]
    fn test_play_selected_with_notation_move_to_opposite_piece_valid() {
        let mut chess_engine = ChessEngine::create_engine_with_empty_board();

        chess_engine = test_play_with_notation_prepare(chess_engine);

        chess_engine.select_piece_notation("g2");

        chess_engine.play_selected_piece_with_notation("f4");

        assert_eq!(chess_engine.turn, Colors::Black);
        assert_eq!(chess_engine.selected_piece, None);

        chess_engine.print_board_with_ranks_and_files_with_all_legal_moves_different_colors();

    }

    #[test]
    fn quick_test_play_with_notation_pawn() {
        let mut chess_engine = ChessEngine::create_engine_with_empty_board();

        let square = "e2";
        chess_engine.add_piece_with_notation(square, Some(Piece::new(
            PieceTypes::Pawn,
            Colors::White,
            chess_engine.get_coords_from_notation(square))
        ));

        let square = "h2";
        chess_engine.add_piece_with_notation(square, Some(Piece::new(
            PieceTypes::Pawn,
            Colors::Black,
            chess_engine.get_coords_from_notation(square))
        ));

        let square = "d7";
        chess_engine.add_piece_with_notation(square, Some(Piece::new(
            PieceTypes::Pawn,
            Colors::Black,
            chess_engine.get_coords_from_notation(square))
        ));

        let square = "a3";
        chess_engine.add_piece_with_notation(square, Some(Piece::new(
            PieceTypes::Pawn,
            Colors::White,
            chess_engine.get_coords_from_notation(square))
        ));

        let square = "b4";
        chess_engine.add_piece_with_notation(square, Some(Piece::new(
            PieceTypes::Pawn,
            Colors::Black,
            chess_engine.get_coords_from_notation(square))
        ));

        chess_engine.print_board_with_ranks_and_files_with_all_legal_moves_different_colors();

        chess_engine.play_with_notation("e2", "e3");

        chess_engine.print_board_with_ranks_and_files_with_all_legal_moves_different_colors();

        chess_engine.play_with_notation("d7", "d6");

        chess_engine.print_board_with_ranks_and_files_with_all_legal_moves_different_colors();

    }

    #[test]
    fn quick_test_play_with_notation_pawn_special_moves() {
        let mut chess_engine = ChessEngine::create_engine_with_empty_board();

        let square = "e2";
        chess_engine.add_piece_with_notation(square, Some(Piece::new(
            PieceTypes::Pawn,
            Colors::White,
            chess_engine.get_coords_from_notation(square))
        ));

        let square = "h2";
        chess_engine.add_piece_with_notation(square, Some(Piece::new(
            PieceTypes::Pawn,
            Colors::Black,
            chess_engine.get_coords_from_notation(square))
        ));

        let square = "d7";
        chess_engine.add_piece_with_notation(square, Some(Piece::new(
            PieceTypes::Pawn,
            Colors::Black,
            chess_engine.get_coords_from_notation(square))
        ));

        let square = "a3";
        chess_engine.add_piece_with_notation(square, Some(Piece::new(
            PieceTypes::Pawn,
            Colors::White,
            chess_engine.get_coords_from_notation(square))
        ));

        let square = "b4";
        chess_engine.add_piece_with_notation(square, Some(Piece::new(
            PieceTypes::Pawn,
            Colors::Black,
            chess_engine.get_coords_from_notation(square))
        ));

        chess_engine.print_board_with_ranks_and_files_with_all_legal_moves_different_colors();

        chess_engine.play_with_notation("e2", "e4");

        chess_engine.print_board_with_ranks_and_files_with_all_legal_moves_different_colors();

        chess_engine.play_with_notation("d7", "d5");

        chess_engine.print_board_with_ranks_and_files_with_all_legal_moves_different_colors();

    }

    #[test]
    fn quick_test_play_with_notation_pawn_take_black() {
        let mut chess_engine = ChessEngine::create_engine_with_empty_board();

        let square = "e2";
        chess_engine.add_piece_with_notation(square, Some(Piece::new(
            PieceTypes::Pawn,
            Colors::White,
            chess_engine.get_coords_from_notation(square))
        ));

        let square = "d7";
        chess_engine.add_piece_with_notation(square, Some(Piece::new(
            PieceTypes::Pawn,
            Colors::Black,
            chess_engine.get_coords_from_notation(square))
        ));

        let square = "a3";
        chess_engine.add_piece_with_notation(square, Some(Piece::new(
            PieceTypes::Pawn,
            Colors::White,
            chess_engine.get_coords_from_notation(square))
        ));

        let square = "b4";
        chess_engine.add_piece_with_notation(square, Some(Piece::new(
            PieceTypes::Pawn,
            Colors::Black,
            chess_engine.get_coords_from_notation(square))
        ));

        chess_engine.print_board_with_ranks_and_files_with_all_legal_moves_different_colors();

        chess_engine.play_with_notation("a3", "b4");

        chess_engine.print_board_with_ranks_and_files_with_all_legal_moves_different_colors();

        // chess_engine.play_with_notation("d7", "d5");
        //
        // chess_engine.print_board_with_ranks_and_files_with_all_legal_moves_different_colors();

    }

    #[test]
    fn quick_test_play_with_notation_pawn_take_white() {
        let mut chess_engine = ChessEngine::create_engine_with_empty_board();

        let square = "e2";
        chess_engine.add_piece_with_notation(square, Some(Piece::new(
            PieceTypes::Pawn,
            Colors::White,
            chess_engine.get_coords_from_notation(square))
        ));

        let square = "d7";
        chess_engine.add_piece_with_notation(square, Some(Piece::new(
            PieceTypes::Pawn,
            Colors::Black,
            chess_engine.get_coords_from_notation(square))
        ));

        let square = "a3";
        chess_engine.add_piece_with_notation(square, Some(Piece::new(
            PieceTypes::Pawn,
            Colors::White,
            chess_engine.get_coords_from_notation(square))
        ));

        let square = "b4";
        chess_engine.add_piece_with_notation(square, Some(Piece::new(
            PieceTypes::Pawn,
            Colors::Black,
            chess_engine.get_coords_from_notation(square))
        ));

        chess_engine.print_board_with_ranks_and_files_with_all_legal_moves_different_colors();

        chess_engine.play_with_notation("e2", "e3");

        chess_engine.print_board_with_ranks_and_files_with_all_legal_moves_different_colors();

        chess_engine.play_with_notation("b4", "a3");

        chess_engine.print_board_with_ranks_and_files_with_all_legal_moves_different_colors();

    }

    #[test]
    fn quick_test_play_with_notation_pawn_go_wrong() {
        let mut chess_engine = ChessEngine::create_engine_with_empty_board();

        let square = "e2";
        chess_engine.add_piece_with_notation(square, Some(Piece::new(
            PieceTypes::Pawn,
            Colors::White,
            chess_engine.get_coords_from_notation(square))
        ));

        let square = "d7";
        chess_engine.add_piece_with_notation(square, Some(Piece::new(
            PieceTypes::Pawn,
            Colors::Black,
            chess_engine.get_coords_from_notation(square))
        ));

        let square = "a3";
        chess_engine.add_piece_with_notation(square, Some(Piece::new(
            PieceTypes::Pawn,
            Colors::White,
            chess_engine.get_coords_from_notation(square))
        ));

        let square = "b4";
        chess_engine.add_piece_with_notation(square, Some(Piece::new(
            PieceTypes::Pawn,
            Colors::Black,
            chess_engine.get_coords_from_notation(square))
        ));

        chess_engine.print_board_with_ranks_and_files_with_all_legal_moves_different_colors();

        chess_engine.play_with_notation("e2", "d3");

        chess_engine.print_board_with_ranks_and_files_with_all_legal_moves_different_colors();

        // chess_engine.play_with_notation("b4", "a3");
        //
        // chess_engine.print_board_with_ranks_and_files_with_all_legal_moves_different_colors();

    }

    #[test]
    fn quick_test_play_with_notation_rook() {
        let mut chess_engine = ChessEngine::create_engine_with_empty_board();

        // let square = "e2";
        // chess_engine.add_piece_with_notation(square, Some(Piece::new(
        //     PieceTypes::Pawn,
        //     Colors::White,
        //     chess_engine.get_coords_from_notation(square))
        // ));

        let square = "g2";
        chess_engine.add_piece_with_notation(square, Some(Piece::new(
            PieceTypes::Pawn,
            Colors::White,
            chess_engine.get_coords_from_notation(square))
        ));

        let square = "d7";
        chess_engine.add_piece_with_notation(square, Some(Piece::new(
            PieceTypes::Pawn,
            Colors::Black,
            chess_engine.get_coords_from_notation(square))
        ));
        //
        // let square = "a3";
        // chess_engine.add_piece_with_notation(square, Some(Piece::new(
        //     PieceTypes::Pawn,
        //     Colors::White,
        //     chess_engine.get_coords_from_notation(square))
        // ));
        //
        let square = "b4";
        chess_engine.add_piece_with_notation(square, Some(Piece::new(
            PieceTypes::Pawn,
            Colors::Black,
            chess_engine.get_coords_from_notation(square))
        ));

        let square = "b3";
        chess_engine.add_piece_with_notation(square, Some(Piece::new(
            PieceTypes::Rook,
            Colors::White,
            chess_engine.get_coords_from_notation(square))
        ));

        let square = "g7";
        chess_engine.add_piece_with_notation(square, Some(Piece::new(
            PieceTypes::Rook,
            Colors::Black,
            chess_engine.get_coords_from_notation(square))
        ));

        chess_engine.print_board_with_ranks_and_files_with_all_legal_moves_different_colors();

        chess_engine.play_with_notation("e2", "e3");

        chess_engine.print_board_with_ranks_and_files_with_all_legal_moves_different_colors();

        chess_engine.play_with_notation("d7", "d6");

        chess_engine.print_board_with_ranks_and_files_with_all_legal_moves_different_colors();

    }

    #[test]
    fn quick_test_play_with_notation_bishop() {
        let mut chess_engine = ChessEngine::create_engine_with_empty_board();

        // let square = "e2";
        // chess_engine.add_piece_with_notation(square, Some(Piece::new(
        //     PieceTypes::Pawn,
        //     Colors::White,
        //     chess_engine.get_coords_from_notation(square))
        // ));

        // let square = "g2";
        // chess_engine.add_piece_with_notation(square, Some(Piece::new(
        //     PieceTypes::Pawn,
        //     Colors::White,
        //     chess_engine.get_coords_from_notation(square))
        // ));

        let square = "e5";
        chess_engine.add_piece_with_notation(square, Some(Piece::new(
            PieceTypes::Pawn,
            Colors::White,
            chess_engine.get_coords_from_notation(square))
        ));
        //
        // let square = "a3";
        // chess_engine.add_piece_with_notation(square, Some(Piece::new(
        //     PieceTypes::Pawn,
        //     Colors::White,
        //     chess_engine.get_coords_from_notation(square))
        // ));
        //
        let square = "b5";
        chess_engine.add_piece_with_notation(square, Some(Piece::new(
            PieceTypes::Pawn,
            Colors::Black,
            chess_engine.get_coords_from_notation(square))
        ));

        let square = "e2";
        chess_engine.add_piece_with_notation(square, Some(Piece::new(
            PieceTypes::Bishop,
            Colors::White,
            chess_engine.get_coords_from_notation(square))
        ));

        let square = "g7";
        chess_engine.add_piece_with_notation(square, Some(Piece::new(
            PieceTypes::Bishop,
            Colors::Black,
            chess_engine.get_coords_from_notation(square))
        ));

        chess_engine.print_board_with_ranks_and_files_with_all_legal_moves_different_colors();

        chess_engine.play_with_notation("e2", "e3");

        chess_engine.print_board_with_ranks_and_files_with_all_legal_moves_different_colors();

        chess_engine.play_with_notation("d7", "d6");

        chess_engine.print_board_with_ranks_and_files_with_all_legal_moves_different_colors();

    }

    #[test]
    fn quick_test_play_with_notation_queen() {
        let mut chess_engine = ChessEngine::create_engine_with_empty_board();

        // let square = "e2";
        // chess_engine.add_piece_with_notation(square, Some(Piece::new(
        //     PieceTypes::Pawn,
        //     Colors::White,
        //     chess_engine.get_coords_from_notation(square))
        // ));

        // let square = "g2";
        // chess_engine.add_piece_with_notation(square, Some(Piece::new(
        //     PieceTypes::Pawn,
        //     Colors::White,
        //     chess_engine.get_coords_from_notation(square))
        // ));

        let square = "e5";
        chess_engine.add_piece_with_notation(square, Some(Piece::new(
            PieceTypes::Pawn,
            Colors::White,
            chess_engine.get_coords_from_notation(square))
        ));
        //
        // let square = "a3";
        // chess_engine.add_piece_with_notation(square, Some(Piece::new(
        //     PieceTypes::Pawn,
        //     Colors::White,
        //     chess_engine.get_coords_from_notation(square))
        // ));
        //
        let square = "b5";
        chess_engine.add_piece_with_notation(square, Some(Piece::new(
            PieceTypes::Pawn,
            Colors::Black,
            chess_engine.get_coords_from_notation(square))
        ));

        let square = "e2";
        chess_engine.add_piece_with_notation(square, Some(Piece::new(
            PieceTypes::Queen,
            Colors::White,
            chess_engine.get_coords_from_notation(square))
        ));

        // let square = "g7";
        // chess_engine.add_piece_with_notation(square, Some(Piece::new(
        //     PieceTypes::Queen,
        //     Colors::Black,
        //     chess_engine.get_coords_from_notation(square))
        // ));

        chess_engine.print_board_with_ranks_and_files_with_all_legal_moves_different_colors();

        chess_engine.play_with_notation("e2", "h5");

        chess_engine.print_board_with_ranks_and_files_with_all_legal_moves_different_colors();

        // chess_engine.play_with_notation("d7", "d6");
        // //
        // chess_engine.print_board_with_ranks_and_files_with_all_legal_moves_different_colors();

    }

    #[test]
    fn test_is_check() {
        let mut chess_engine = ChessEngine::create_engine_with_empty_board();

        let square = "g2";
        chess_engine.add_piece_with_notation(square, Some(Piece::new(
            PieceTypes::Queen,
            Colors::White,
            chess_engine.get_coords_from_notation(square))
        ));

        let square = "g1";
        chess_engine.add_piece_with_notation(square, Some(Piece::new(
            PieceTypes::King,
            Colors::White,
            chess_engine.get_coords_from_notation(square))
        ));

        let square = "g7";
        chess_engine.add_piece_with_notation(square, Some(Piece::new(
            PieceTypes::Queen,
            Colors::Black,
            chess_engine.get_coords_from_notation(square))
        ));

        let square = "g8";
        chess_engine.add_piece_with_notation(square, Some(Piece::new(
            PieceTypes::King,
            Colors::Black,
            chess_engine.get_coords_from_notation(square))
        ));

        chess_engine.print_board_with_ranks_and_files_with_all_legal_moves_different_colors();

        assert!(ChessEngine::is_checked(chess_engine.clone().board,
                                        chess_engine.clone().turn,
                                        chess_engine.clone().get_coords_from_notation("g7"),
                                        chess_engine.clone().get_coords_from_notation("a7"))
        );

        assert!(!ChessEngine::is_checked(chess_engine.clone().board,
                                         chess_engine.clone().turn,
                                         chess_engine.clone().get_coords_from_notation("g7"),
                                         chess_engine.clone().get_coords_from_notation("g6"))
        );
    }

    #[test]
    fn test_play_when_checked() {
        let mut chess_engine = ChessEngine::create_engine_with_empty_board();

        let square = "g2";
        chess_engine.add_piece_with_notation(square, Some(Piece::new(
            PieceTypes::Queen,
            Colors::White,
            chess_engine.get_coords_from_notation(square))
        ));

        let square = "g1";
        chess_engine.add_piece_with_notation(square, Some(Piece::new(
            PieceTypes::King,
            Colors::White,
            chess_engine.get_coords_from_notation(square))
        ));

        let square = "g7";
        chess_engine.add_piece_with_notation(square, Some(Piece::new(
            PieceTypes::Queen,
            Colors::Black,
            chess_engine.get_coords_from_notation(square))
        ));

        let square = "g8";
        chess_engine.add_piece_with_notation(square, Some(Piece::new(
            PieceTypes::King,
            Colors::Black,
            chess_engine.get_coords_from_notation(square))
        ));

        chess_engine.print_board_with_ranks_and_files_with_all_legal_moves_different_colors();

        chess_engine.play_with_notation("g2", "a2");

        // println!("{:?}", chess_engine.turn);

        println!("{:?}", chess_engine.status);
        chess_engine.print_board_with_ranks_and_files_with_all_legal_moves_different_colors();

        chess_engine.play_with_notation("g2", "g7");

        println!("{:?}", chess_engine.status);
        chess_engine.print_board_with_ranks_and_files_with_all_legal_moves_different_colors();


        // assert!(ChessEngine::is_check(chess_engine.clone().board,
        //                               chess_engine.clone().turn,
        //                               chess_engine.clone().get_coords_from_notation("g7"),
        //                               chess_engine.clone().get_coords_from_notation("a7"))
        // );

        // assert!(!ChessEngine::is_check(chess_engine.clone().board,
        //                                chess_engine.clone().turn,
        //                                chess_engine.clone().get_coords_from_notation("g7"),
        //                                chess_engine.clone().get_coords_from_notation("g6"))
        // );
    }
}
