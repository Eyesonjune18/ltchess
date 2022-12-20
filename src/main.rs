mod chess_gamestate;
mod chess_move;
mod chess_piece;
mod chess_point;

pub use chess_gamestate::ChessGamestate;
pub use chess_move::ChessMove;
pub use chess_move::MoveValidity;
pub use chess_piece::ChessPiece;
pub use chess_piece::ChessPieceColor;
pub use chess_piece::ChessPieceKind;
pub use chess_point::ChessPoint;

use std::io::Write;

fn main() {
    clear();

    let mut game = ChessGamestate::new();

    game.print_board();

    loop {
        print("\nEnter a move: ");

        let mut user_inputted_move = String::new();
        std::io::stdin().read_line(&mut user_inputted_move).unwrap();
        let move_to_make = ChessMove::from(&user_inputted_move);
        let piece_to_move = game.board.piece_at(move_to_make.source()).unwrap();
        
        if !piece_to_move.can_make_move(&move_to_make).standard {
            print("Move is invalid for that piece.\n");
            continue;
        }
        
        clear();
        
        piece_to_move.increment_move_count();
        game.move_piece(&move_to_make);
        game.print_board();
    }
}

fn print(output: &str) {
    print!("{}", output);
    std::io::stdout().flush().unwrap();
}

fn clear() {
    print!("\x1B[2J\x1B[1;1H");
}
