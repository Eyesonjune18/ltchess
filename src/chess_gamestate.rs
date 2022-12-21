use crate::ChessError;
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
    // Mutable version is included below
    pub fn piece_at(&self, point: &ChessPoint) -> Option<&ChessPiece> {
        self.pieces[point.y()][point.x()].as_ref()
    }

    pub fn piece_at_mut(&mut self, point: &ChessPoint) -> Option<&mut ChessPiece> {
        self.pieces[point.y()][point.x()].as_mut()
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

    // Checks if a move is legal, based on a combination of the moved piece and the gamestate variables
    pub fn validate_move(&self, queried_move: &ChessMove) -> Result<(), ChessError> {
        use ChessError::*;

        // Ensure that the source tile is not empty
        let moved_piece = match self.board.piece_at(&queried_move.source()) {
            Some(piece) => piece,
            None => return Err(NoPieceAtMoveSource),
        };

        // Because there may not be a captured piece, it must be stored as an Option and unwrapped later
        let captured_piece = self.board.piece_at(&queried_move.destination());

        let move_is_capture = captured_piece.is_some();

        // Ensure that the source tile does not contain an enemy piece
        if moved_piece.color != self.turn {
            return Err(EnemyPieceAtMoveSource);
        }

        // Ensure that the piece does not collide with other pieces
        if moved_piece.kind != ChessPieceKind::Knight {
            let points_between =
                ChessPoint::get_points_between(queried_move.source(), queried_move.destination());

            for point in points_between {
                if self.board.piece_at(&point).is_some() {
                    return Err(MoveCollisionOccurs);
                }
            }
        }

        // Ensure that the captured piece (if there is one) is an enemy piece
        if move_is_capture {
            // Unwrap is safe here because there must be a destination piece for the move to be a capture
            if captured_piece.unwrap().color == self.turn {
                return Err(CannotCaptureFriendly);
            }
        }

        // Ensure that the move pattern is legal for the piece
        let move_pattern_legality = moved_piece.can_make_move(queried_move);

        if !match move_is_capture {
            true => move_pattern_legality.capture,
            false => move_pattern_legality.standard,
        } {
            return Err(InvalidMovePattern);
        }

        Ok(())
    }

    // Updates gamestate variables as necessary before a move is performed
    // This function should be called before move_piece() and after validate_move()
    // TODO: Error handling here
    pub fn update_gamestate(&mut self, performed_move: &ChessMove) {
        use ChessPieceColor::*;

        let moved_piece = self.board.piece_at_mut(performed_move.source()).unwrap();
        moved_piece.increment_move_count();
        let moved_piece_is_pawn = moved_piece.kind == ChessPieceKind::Pawn;

        let captured_piece = self.board.piece_at(performed_move.destination());

        if captured_piece.is_some() || moved_piece_is_pawn {
            self.halfmove_clock = 0;
        } else {
            self.halfmove_clock += 1;
        }

        self.fullmove_clock += 1;

        self.turn = match self.turn {
            White => Black,
            Black => White,
        };
    }

    // Moves a piece from one square to another, without checking if the move is legal
    pub fn move_piece(&mut self, requested_move: &ChessMove) {
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
