//! # Two-Player Chess Program
//!
//! This module provides the foundation for implementing a two-player chess game in Rust.
//! The program will include the following key components:
//!
//! ## Features
//! - **Game Board Representation**: A data structure to represent an 8x8 chessboard, including pieces and their positions.
//! - **Piece Movement Rules**: Logic to validate legal moves for each type of chess piece (pawn, knight, bishop, rook, queen, king).
//! - **Turn-Based Gameplay**: Alternating turns between two players, enforcing the rules of chess.
//! - **Check and Checkmate Detection**: Mechanisms to detect when a king is in check or checkmate.
//! - **Stalemate and Draw Conditions**: Logic to handle stalemates, insufficient material, and other draw scenarios.
//! - **Move History**: A record of all moves made during the game for undo functionality or analysis.
//!
//! ## Plan
//! 1. **Define Data Structures**:
//!    - Create enums for `PieceType` (pawn, knight, etc.) and `Color` (white, black).
//!    - Define a `Piece` struct to represent a chess piece with its type and color.
//!    - Implement a `Board` struct to represent the chessboard as an 8x8 grid.
//!
//! 2. **Implement Movement Logic**:
//!    - Write functions to validate moves for each piece type based on chess rules.
//!    - Ensure moves respect the board boundaries and piece interactions (e.g., capturing, blocking).
//!
//! 3. **Game Flow Management**:
//!    - Create a `Game` struct to manage the state of the game, including the board, turn order, and move history.
//!    - Implement functions to switch turns and enforce valid moves.
//!
//! 4. **Special Rules**:
//!    - Add support for castling, en passant, and pawn promotion.
//!    - Handle check, checkmate, and stalemate conditions.
//!
//! 5. **User Interaction**:
//!    - Provide a text-based interface for players to input moves (e.g., "e2 to e4").
//!    - Display the board state after each move.
//!
//! 6. **Testing and Validation**:
//!    - Write unit tests for movement rules, game state transitions, and special conditions.
//!    - Ensure the program handles edge cases and invalid inputs gracefully.
//!
//! ## Future Enhancements
//! - Add support for a graphical user interface (GUI).
//! - Implement AI for single-player mode.
//! - Include a timer for timed games (e.g., blitz chess).
//! - Save and load game states for resuming later.

use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ChessPiece {
    Pawn(Colour),
    Knight(Colour),
    Bishop(Colour),
    Rook(Colour),
    Queen(Colour),
    King(Colour),
    Blank,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Colour {
    White,
    Black,
}

#[derive(Debug)]
struct GameState {
    board: [[ChessPiece; 8]; 8],
    current_player: Colour,
}

// > add an enum to represent the name of a square on the board
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Square {
    A1, A2, A3, A4, A5, A6, A7, A8,
    B1, B2, B3, B4, B5, B6, B7, B8,
    C1, C2, C3, C4, C5, C6, C7, C8,
    D1, D2, D3, D4, D5, D6, D7, D8,
    E1, E2, E3, E4, E5, E6, E7, E8,
    F1, F2, F3, F4, F5, F6, F7, F8,
    G1, G2, G3, G4, G5, G6, G7, G8,
    H1, H2, H3, H4, H5, H6, H7, H8,
}

// > implement a method to convert Square to a row and column
impl Square {
    fn to_row_col(self) -> (usize, usize) {
        let index = self as usize;
        let row = index % 8;
        let col = index / 8;
        (row, col)
    }
}

// > implement FromStr for square
impl FromStr for Square {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "A1" => Ok(Square::A1), "A2" => Ok(Square::A2), "A3" => Ok(Square::A3), "A4" => Ok(Square::A4),
            "A5" => Ok(Square::A5), "A6" => Ok(Square::A6), "A7" => Ok(Square::A7), "A8" => Ok(Square::A8),
            "B1" => Ok(Square::B1), "B2" => Ok(Square::B2), "B3" => Ok(Square::B3), "B4" => Ok(Square::B4),
            "B5" => Ok(Square::B5), "B6" => Ok(Square::B6), "B7" => Ok(Square::B7), "B8" => Ok(Square::B8),
            "C1" => Ok(Square::C1), "C2" => Ok(Square::C2), "C3" => Ok(Square::C3), "C4" => Ok(Square::C4),
            "C5" => Ok(Square::C5), "C6" => Ok(Square::C6), "C7" => Ok(Square::C7), "C8" => Ok(Square::C8),
            "D1" => Ok(Square::D1), "D2" => Ok(Square::D2), "D3" => Ok(Square::D3), "D4" => Ok(Square::D4),
            "D5" => Ok(Square::D5), "D6" => Ok(Square::D6), "D7" => Ok(Square::D7), "D8" => Ok(Square::D8),
            "E1" => Ok(Square::E1), "E2" => Ok(Square::E2), "E3" => Ok(Square::E3), "E4" => Ok(Square::E4),
            "E5" => Ok(Square::E5), "E6" => Ok(Square::E6), "E7" => Ok(Square::E7), "E8" => Ok(Square::E8),
            "F1" => Ok(Square::F1), "F2" => Ok(Square::F2), "F3" => Ok(Square::F3), "F4" => Ok(Square::F4),
            "F5" => Ok(Square::F5), "F6" => Ok(Square::F6), "F7" => Ok(Square::F7), "F8" => Ok(Square::F8),
            "G1" => Ok(Square::G1), "G2" => Ok(Square::G2), "G3" => Ok(Square::G3), "G4" => Ok(Square::G4),
            "G5" => Ok(Square::G5), "G6" => Ok(Square::G6), "G7" => Ok(Square::G7), "G8" => Ok(Square::G8),
            "H1" => Ok(Square::H1), "H2" => Ok(Square::H2), "H3" => Ok(Square::H3), "H4" => Ok(Square::H4),
            "H5" => Ok(Square::H5), "H6" => Ok(Square::H6), "H7" => Ok(Square::H7), "H8" => Ok(Square::H8),
            _ => Err(format!("Invalid square: {}", s)),
        }
    }
}

// > implement Display for GameState with labeled rows and columns
impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // let white_bg = "\x1b[47m"; // Escape code for white background
        // let blue_bg = "\x1b[44m"; // Escape code for grey background
        let cancel = "\x1b[0m"; // Escape code to cancel color
        let white_fg = "\x1b[37m"; // Escape code for white foreground
        let black_fg = "\x1b[30m"; // Escape code for black foreground
        writeln!(f, "  A B C D E F G H")?;
        for (i, row) in self.board.iter().rev().enumerate() {
            write!(f, "{} ", 8 - i)?; // Row labels (8 to 1)
            for (j, piece) in row.iter().enumerate() {
                use ChessPiece::*; // These were manual edits to make the code shorter.
                use Colour::*;
                let symbol = match piece {
                    Pawn(White) => "♟",
                    Knight(White) => "♞",
                    Bishop(White) => "♝",
                    Rook(White) => "♜",
                    Queen(White) => "♛",
                    King(White) => "♚",
                    Pawn(Black) => "♙",
                    Knight(Black) => "♘",
                    Bishop(Black) => "♗",
                    Rook(Black) => "♖",
                    Queen(Black) => "♕",
                    King(Black) => "♔",
                    Blank => ".",
                };
                write!(f, "{} {}", symbol, cancel)?;
            }
            writeln!(f)?;
        }
        writeln!(f, "  A B C D E F G H")?;
        writeln!(f, "Current Player: {:?}", self.current_player)
    }
}

impl GameState {
    // > add a constructor to GameState using an 8x8 const array
    fn new() -> Self {
        use ChessPiece::*; // These were manual edits to make the code shorter.
        use Colour::*;
        const INITIAL_BOARD: [[ChessPiece; 8]; 8] = [
            [Rook(White), Knight(White), Bishop(White), Queen(White),
                King(White), Bishop(White), Knight(White), Rook(White)],
            [Pawn(White); 8],
            [Blank; 8],
            [Blank; 8],
            [Blank; 8],
            [Blank; 8],
            [Pawn(Black); 8],
            [Rook(Black), Knight(Black), Bishop(Black), Queen(Black),
                King(Black), Bishop(Black), Knight(Black), Rook(Black)],
        ];

        GameState {
            board: INITIAL_BOARD,
            current_player: Colour::White,
        }
    }

    // > implement make_move
    fn make_move(&mut self, from: Square, to: Square) -> Result<(), String> {
        let (from_row, from_col) = from.to_row_col();
        let (to_row, to_col) = to.to_row_col();

        let piece = self.board[from_row][from_col];
        if piece == ChessPiece::Blank {
            return Err("No piece at the source square.".to_string());
        }

        if let ChessPiece::Pawn(colour) 
        | ChessPiece::Knight(colour) 
        | ChessPiece::Bishop(colour) 
        | ChessPiece::Rook(colour) 
        | ChessPiece::Queen(colour) 
        | ChessPiece::King(colour) = piece 
        {
            if colour != self.current_player {
                return Err("It's not your turn.".to_string());
            }
        }

        // For now, allow any move (basic implementation)
        self.board[to_row][to_col] = piece;
        self.board[from_row][from_col] = ChessPiece::Blank;

        // Switch the current player
        self.current_player = match self.current_player {
            Colour::White => Colour::Black,
            Colour::Black => Colour::White,
        };

        Ok(())
    }
}
// > test that to_row_col returns (0,0) for A1
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_row_col_a1() {
        let square = Square::A1;
        let (row, col) = square.to_row_col();
        assert_eq!((row, col), (0, 0));
    }

    #[test]
    fn test_to_row_col_h8() {
        let square = Square::H8;
        let (row, col) = square.to_row_col();
        assert_eq!((row, col), (7, 7));
    }

    #[test]
    fn test_to_row_col_e4() {
        let square = Square::E4;
        let (row, col) = square.to_row_col();
        assert_eq!((row, col), (4, 4));
    }

    #[test]
    fn test_from_str_valid_square() {
        let square: Square = "A1".parse().unwrap();
        assert_eq!(square, Square::A1);
    }

    #[test]
    fn test_from_str_invalid_square() {
        let square: Result<Square, _> = "Z9".parse();
        assert!(square.is_err());
    }

    #[test]
    fn test_game_state_initialization() {
        let game_state = GameState::new();
        assert_eq!(game_state.board[0][0], ChessPiece::Rook(Colour::White));
        assert_eq!(game_state.board[7][7], ChessPiece::Rook(Colour::Black));
        assert_eq!(game_state.current_player, Colour::White);
    }
}


// > add a loop to display the board and accept moves
fn main() {
    let mut game_state = GameState::new();

    loop {
        println!("{}", game_state);

        println!("Enter your move (e.g., 'e2 e4') or 'quit' to exit:");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read input");
        let input = input.trim();

        if input.eq_ignore_ascii_case("quit") {
            println!("Exiting the game. Goodbye!");
            break;
        }

        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.len() != 2 {
            println!("Invalid input. Please enter a move in the format 'e2 e4'.");
            continue;
        }

        let from_square = match parts[0].parse::<Square>() {
            Ok(square) => square,
            Err(err) => {
                println!("{}", err);
                continue;
            }
        };

        let to_square = match parts[1].parse::<Square>() {
            Ok(square) => square,
            Err(err) => {
                println!("{}", err);
                continue;
            }
        };

        if let Err(err) = game_state.make_move(from_square, to_square) {
            println!("Invalid move: {}", err);
            continue;
        }
    }
}

