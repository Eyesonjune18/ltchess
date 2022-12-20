use crate::ChessPiece;
use crate::ChessMove;
use crate::ChessPieceColor;
use crate::ChessPieceKind;
use crate::ChessPoint;

pub struct ChessGamestate {
    pub board: ChessBoard,
    pub turn: ChessPieceColor,
    pub white_king: ChessPoint,
    pub black_king: ChessPoint,
    pub white_castle_kingside: bool,
    pub white_castle_queenside: bool,
    pub black_castle_kingside: bool,
    pub black_castle_queenside: bool,
    pub en_passant: Option<ChessPoint>,
    pub halfmove_clock: u32,
    pub fullmove_clock: u32,
}

pub struct ChessBoard {
    pieces: [[Option<ChessPiece>; 8]; 8],
}

impl ChessBoard {
    fn new() -> Self {
        use ChessPieceKind::*;
        use ChessPieceColor::*;

        const EMPTY_ROW: [Option<ChessPiece>; 8] = [None; 8];

        ChessBoard {
            pieces: [
                [
                    ChessPiece::new(Rook, White),
                    ChessPiece::new(Knight, White),
                    ChessPiece::new(Bishop, White),
                    ChessPiece::new(Queen, White),
                    ChessPiece::new(King, White),
                    ChessPiece::new(Bishop, White),
                    ChessPiece::new(Knight, White),
                    ChessPiece::new(Rook, White),
                ].map(Some),
                [ChessPiece::new(Pawn, White); 8].map(Some),
                EMPTY_ROW,
                EMPTY_ROW,
                EMPTY_ROW,
                EMPTY_ROW,
                [ChessPiece::new(Pawn, Black); 8].map(Some),
                [
                    ChessPiece::new(Rook, Black),
                    ChessPiece::new(Knight, Black),
                    ChessPiece::new(Bishop, Black),
                    ChessPiece::new(Queen, Black),
                    ChessPiece::new(King, Black),
                    ChessPiece::new(Bishop, Black),
                    ChessPiece::new(Knight, Black),
                    ChessPiece::new(Rook, Black),
                ].map(Some),
            ],
        }
    }
}

impl ChessGamestate {
    pub fn new() -> Self {
        ChessGamestate {
            board: ChessBoard::new(),
            turn: ChessPieceColor::White,
            white_king: ChessPoint::new(4, 0),
            black_king: ChessPoint::new(4, 7),
            white_castle_kingside: true,
            white_castle_queenside: true,
            black_castle_kingside: true,
            black_castle_queenside: true,
            en_passant: None,
            halfmove_clock: 0,
            fullmove_clock: 0,
        }
    }

    pub fn move_piece(&mut self, requested_move: ChessMove) {
        self.board.pieces[requested_move.to.y() as usize][requested_move.to.x() as usize] = self.board.pieces[requested_move.from.y() as usize][requested_move.from.x() as usize];
    }

    pub fn print_board(&self) {
        for row in self.board.pieces.iter().rev() {
            for piece in row.iter() {
                match piece {
                    Some(piece) => print!("[{} ]", piece.get_piece_char()),
                    None => print!("[  ]"),
                }
            }
            println!();
        }
    }
}
