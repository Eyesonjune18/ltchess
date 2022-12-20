use crate::ChessMove;
use crate::ChessPiece;
use crate::ChessPieceColor;
use crate::ChessPieceKind;
use crate::ChessPoint;

pub struct ChessGamestate {
    // The array of pieces on the board
    pub board: ChessBoard,
    // The color of the player whose turn it is
    pub turn: ChessPieceColor,
    // The current position of each king
    pub white_king: ChessPoint,
    pub black_king: ChessPoint,
    // Castling rights for each side
    pub white_castle_kingside: bool,
    pub white_castle_queenside: bool,
    pub black_castle_kingside: bool,
    pub black_castle_queenside: bool,
    // The square where a pawn can be en passant-captured, if there is one
    pub en_passant: Option<ChessPoint>,
    // Moves since the last capture or pawn move
    pub halfmove_clock: u32,
    // Total number of moves in the game
    pub fullmove_clock: u32,
}

pub struct ChessBoard {
    pieces: [[Option<ChessPiece>; 8]; 8],
}

impl ChessBoard {
    // Creates a board from the default starting position
    fn new() -> Self {
        use ChessPieceColor::*;
        use ChessPieceKind::*;

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
                ]
                .map(Some),
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
                ]
                .map(Some),
            ],
        }
    }

    // Returns the piece at a given point, if there is one
    pub fn piece_at(&self, point: &ChessPoint) -> Option<&ChessPiece> {
        self.pieces[point.y()][point.x()].as_ref()
    }
}

impl ChessGamestate {
    // Creates a gamestate from the default starting position
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

    // Moves a piece from one square to another, without checking if the move is legal
    pub fn move_piece(&mut self, requested_move: ChessMove) {
        self.board.pieces[requested_move.destination().y()][requested_move.destination().x()] =
            self.board.pieces[requested_move.source().y()][requested_move.source().x()];
        self.board.pieces[requested_move.source().y()][requested_move.source().x()] = None;
    }

    // TODO: Probably move this to UI
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
