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
    pub white_king_position: ChessPoint,
    pub black_king_position: ChessPoint,
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
            white_king_position: ChessPoint::new(4, 0),
            black_king_position: ChessPoint::new(4, 7),
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

        // Ensure that the move pattern is legal for the piece
        // This must be done before the collision check because the collision check will malfunction if the path is illegal
        let move_pattern_legality = moved_piece.can_make_move(queried_move);

        if !match move_is_capture {
            true => move_pattern_legality.capture,
            false => move_pattern_legality.standard,
        } {
            return Err(InvalidMovePattern);
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

        Ok(())
    }

    // Updates gamestate variables as necessary before a move is performed
    // This function should be called before move_piece() and after validate_move()
    // TODO: Error handling here
    fn update_gamestate(&mut self, move_to_perform: &ChessMove) {
        use ChessPieceColor::*;

        // Increment the piece's move count
        let moved_piece = self.board.piece_at_mut(move_to_perform.source()).unwrap();
        moved_piece.increment_move_count();
        
        // Check if a Pawn was moved and grab captured piece (if there is one) to check if the halfmove clock should be reset
        let moved_piece_is_pawn = moved_piece.kind == ChessPieceKind::Pawn;
        let captured_piece = self.board.piece_at(move_to_perform.destination());

        // Increment the move clocks
        if captured_piece.is_some() || moved_piece_is_pawn {
            self.halfmove_clock = 0;
        } else {
            self.halfmove_clock += 1;
        }

        self.fullmove_clock += 1;

        // Update the positions of both Kings
        self.white_king_position = self.find_king(White);
        self.black_king_position = self.find_king(Black);

        // Update the castling rights
        // Though these tiles do not always contain Rooks and Kings, the castling rights
        // would have been removed beforehand anyway if the tiles are no longer occupied by Rooks or Kings
        match (move_to_perform.source().x(), move_to_perform.source().y()) {
            (7, 0) => self.white_castle_kingside = false,
            (0, 0) => self.white_castle_queenside = false,
            (7, 7) => self.black_castle_kingside = false,
            (0, 7) => self.black_castle_queenside = false,
            (4, 0) => {
                self.white_castle_kingside = false;
                self.white_castle_queenside = false;
            }
            (4, 7) => {
                self.black_castle_kingside = false;
                self.black_castle_queenside = false;
            }
            _ => (),
        }

        // Swap the turn color
        self.turn = match self.turn {
            White => Black,
            Black => White,
        };
    }

    // Moves a piece from one square to another, without checking if the move is legal
    fn move_piece(&mut self, requested_move: &ChessMove) {
        self.board.pieces[requested_move.destination().y()][requested_move.destination().x()] =
            self.board.pieces[requested_move.source().y()][requested_move.source().x()];
        self.board.pieces[requested_move.source().y()][requested_move.source().x()] = None;
    }

    // Performs a move, updating the gamestate as necessary
    // This function provides a safer interface for performing a move, as anyone writing
    // external code do not need to worry about the order of the 3 functions
    pub fn perform_move(&mut self, move_to_perform: &ChessMove) -> Result<(), ChessError> {
        self.validate_move(move_to_perform)?;
        self.update_gamestate(move_to_perform);
        self.move_piece(move_to_perform);

        Ok(())
    }

    // Finds the position of the given color's King
    fn find_king(&self, color: ChessPieceColor) -> ChessPoint {
        for (y, row) in self.board.pieces.iter().enumerate() {
            for (x, piece) in row.iter().enumerate() {
                if let Some(piece) = piece {
                    if piece.kind == ChessPieceKind::King && piece.color == color {
                        return ChessPoint::new(x, y);
                    }
                }
            }
        }

        unreachable!("[INTERNAL ERROR] Unable to find King");
    }

    // Checks if the given color's King is in check
    // If using this for move validation (self-check rule) then the move should be performed first
    fn is_check(&self, friendly_color: ChessPieceColor) -> bool {
        let king_position = match friendly_color {
            ChessPieceColor::White => self.white_king_position,
            ChessPieceColor::Black => self.black_king_position,
        };

        // This can be optimized by only looking at the squares that enemy pieces could be in to threaten the King
        for (y, row) in self.board.pieces.iter().enumerate() {
            for (x, piece) in row.iter().enumerate() {
                if let Some(piece) = piece {
                    if piece.color != friendly_color {
                        let move_to_check = ChessMove::new(ChessPoint::new(x, y), king_position);

                        if self.validate_move(&move_to_check).is_ok() {
                            return true;
                        }
                    }
                }
            }
        }

        false
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
