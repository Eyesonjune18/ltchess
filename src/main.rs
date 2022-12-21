mod chess_error;
mod chess_gamestate;
mod chess_move;
mod chess_piece;
mod chess_point;

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
    clear_terminal();

    let mut game = ChessGamestate::new();

    game.print_board();

    loop {
        print("\nEnter a move: ");

        let mut user_inputted_move = String::new();
        stdin().read_line(&mut user_inputted_move).unwrap();
        let move_to_make = ChessMove::from(&user_inputted_move);

        match game.perform_move(&move_to_make) {
            Ok(_) => (),
            Err(err) => {
                use ChessError::*;

                print(match err {
                    InvalidMovePattern => "The piece you selected cannot move in the way specified.\n",
                    MoveCollisionOccurs => "Excluding knights, pieces cannot move through other pieces.\n",
                    CannotCaptureFriendly => "You cannot capture your own pieces.\n",
                    CannotSelfCheck => "You cannot move into check.\n",
                    EnemyPieceAtMoveSource => "You cannot move an enemy piece.\n",
                    NoPieceAtMoveSource => "There is no piece at the selected tile.\n",
                });

                continue;
            }
        }

        clear_terminal();

        game.print_board();
    }
}

fn print(output: &str) {
    print!("{}", output);
    stdout().flush().unwrap();
}

fn clear_terminal() {
    print!("\x1B[2J\x1B[1;1H");
}
