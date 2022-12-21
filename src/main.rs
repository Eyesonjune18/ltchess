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

        match game.validate_move(&move_to_make) {
            Ok(_) => (),
            Err(_) => {
                print("Move is invalid for that piece.\n");
                continue;
            }
        }

        game.update_gamestate(&move_to_make);
        game.move_piece(&move_to_make);
        
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
