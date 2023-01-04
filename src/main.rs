mod chess_board;
mod chess_error;
mod chess_gamestate;
mod chess_move;
mod chess_piece;
mod chess_point;

pub use chess_board::ChessBoard;
pub use chess_error::ChessError;
pub use chess_gamestate::ChessGamestate;
pub use chess_move::ChessMove;
pub use chess_move::MovePatternValidity;
pub use chess_piece::ChessPiece;
pub use chess_piece::ChessPieceColor;
pub use chess_piece::ChessPieceKind;
pub use chess_point::ChessPoint;

use std::io::stdin;
use std::io::stdout;
use std::io::Write;

fn main() {
    let mut game = ChessGamestate::new();

    // This uses two loops to avoid printing the turn message on every failed move
    loop {
        clear_terminal();

        game.print_board();

        println!(
            "\nIt is {}'s turn.",
            match game.turn_color {
                ChessPieceColor::White => "white",
                ChessPieceColor::Black => "black",
            }
        );

        loop {
            print!("Enter a move: ");
            flush();

            let mut user_inputted_move = String::new();
            stdin().read_line(&mut user_inputted_move).unwrap();
            let move_to_make = ChessMove::from(&user_inputted_move);

            match game.perform_move(&move_to_make) {
                // If the move was valid, break out of the move repeat loop
                Ok(_) => break,
                Err(err) => {
                    use ChessError::*;

                    println!(
                        "{}\n",
                        match err {
                            InvalidMovePattern =>
                                "The piece you selected cannot move in the way specified.",
                            MoveCollisionOccurs =>
                                "Pieces other than Knights cannot move through other pieces.",
                            CannotCaptureFriendly => "You cannot capture your own pieces.",
                            CannotSelfCheck => "You cannot move into check.",
                            EnemyPieceAtMoveSource => "You cannot move an enemy piece.",
                            NoPieceAtMoveSource => "There is no piece at the selected tile.",
                        }
                    );
                }
            }
        }
    }
}

fn flush() {
    stdout().flush().unwrap();
}

fn clear_terminal() {
    print!("\x1B[2J\x1B[1;1H");
}
