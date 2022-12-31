use crate::ChessError;
use crate::ChessMove;
use crate::ChessPiece;
use crate::ChessPieceColor;
use crate::ChessPieceKind;
use crate::ChessPoint;

#[derive(Clone, Copy)]
pub struct ChessGamestate {
    // The array of pieces on the board
    pub board: ChessBoard,
    // The color of the player whose turn it is
    pub turn_color: ChessPieceColor,
    // The current position of each king
    pub white_king_position: ChessPoint,
    pub black_king_position: ChessPoint,
    // Castling rights for each side
    pub white_castle_kingside: bool,
    pub white_castle_queenside: bool,
    pub black_castle_kingside: bool,
    pub black_castle_queenside: bool,
    // The square where a pawn can be en passant-captured, if there is one
    pub en_passant_tile: Option<ChessPoint>,
    // Moves since the last capture or pawn move
    pub halfmove_clock: u32,
    // Total number of moves in the game
    pub fullmove_clock: u32,
}

#[derive(Clone, Copy)]
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

    // Sets a given tile on the board to a given piece (or empties the tile if given None)
    pub fn set_piece(&mut self, point: &ChessPoint, piece: Option<ChessPiece>) {
        self.pieces[point.y()][point.x()] = piece;
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
            turn_color: ChessPieceColor::White,
            white_king_position: ChessPoint::new(4, 0),
            black_king_position: ChessPoint::new(4, 7),
            white_castle_kingside: true,
            white_castle_queenside: true,
            black_castle_kingside: true,
            black_castle_queenside: true,
            en_passant_tile: None,
            halfmove_clock: 0,
            fullmove_clock: 0,
        }
    }

    // Checks if a move is legal, based on a combination of the moved piece and the gamestate variables
    // Check override is used to avoid calling is_check() recursively, as this function is called by is_check()
    fn validate_move(
        &self,
        queried_move: &ChessMove,
        check_override: bool,
    ) -> Result<(), ChessError> {
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
        if moved_piece.color != self.turn_color {
            return Err(EnemyPieceAtMoveSource);
        }

        // Ensure that the move pattern is legal for the piece
        // This must be done before the collision check because the collision check will malfunction if the path is illegal
        let move_pattern_legality = moved_piece.can_make_move(queried_move);

        // If the move is a capture, check the capture legality instead of the standard move legality
        // This is pretty much entirely because of Pawns, whose capture and standard move patterns are different
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
            if captured_piece.unwrap().color == self.turn_color {
                return Err(CannotCaptureFriendly);
            }
        }

        // Ensure that the move does not put the friendly King in check
        if !check_override {
            // Copy the gamestate and perform the move on the copy
            let mut hypothetical_gamestate = self.clone();
            hypothetical_gamestate.move_piece(queried_move);

            // A full gamestate update is unnecessary here, so the turn color and king positionsare updated individually
            // The color is updated in order to allow the is_check() validation calls to work properly without extra parameters
            hypothetical_gamestate.update_king_positions();
            hypothetical_gamestate.swap_turn_color();

            if hypothetical_gamestate.is_check() {
                return Err(CannotSelfCheck);
            }
        }

        Ok(())
    }

    // Updates gamestate variables as necessary before a move is performed
    // This function should be called after validate_move() and move_piece()
    // Because the move has already been performed, the function must be informed of whether or not the
    // move was a catpture in order to update the halfmove clock correctly
    fn update_gamestate(&mut self, performed_move: &ChessMove, move_was_capture: bool) {
        // As long as the move calling order is correct, the piece at the destination will be the moved piece
        let moved_piece = match self.board.piece_at_mut(performed_move.destination()) {
            Some(piece) => piece,
            None => unreachable!("[INTERNAL ERROR] Moved piece not found at destination after move"),
        };

        moved_piece.increment_move_count();

        // Check if a Pawn was moved to determine if the halfmove clock should be reset
        let moved_piece_was_pawn = moved_piece.kind == ChessPieceKind::Pawn;

        self.increment_move_clocks(move_was_capture || moved_piece_was_pawn);

        self.update_king_positions();

        self.update_castling_rights();

        self.swap_turn_color();
    }

    // Performs a "simple move" - a piece is moved from one tile to another, without checking any validity requirements
    fn move_piece(&mut self, requested_move: &ChessMove) -> bool {
        let move_is_capture = self.board.piece_at(requested_move.destination()).is_some();

        let moved_piece = self.board.piece_at(requested_move.source()).copied();

        self.board.set_piece(requested_move.destination(), moved_piece);
        self.board.set_piece(requested_move.source(), None);

        move_is_capture
    }

    // Performs a "complex move" - the move is validated, the simple move is performed, and the gamestate is updated
    // This function provides a safer interface for performing a move, as anyone writing
    // external code does not need to worry about the order of the 3 functions
    pub fn perform_move(&mut self, move_to_perform: &ChessMove) -> Result<(), ChessError> {
        self.validate_move(move_to_perform, false)?;
        let move_was_capture = self.move_piece(move_to_perform);
        self.update_gamestate(move_to_perform, move_was_capture);

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

    // Updates the King positions fields in the gamestate
    fn update_king_positions(&mut self) {
        self.white_king_position = self.find_king(ChessPieceColor::White);
        self.black_king_position = self.find_king(ChessPieceColor::Black);
    }

    // Updates the castling rights for both colors, based on the current board state
    // Though Rooks and Kings can move back to their original positions, the castling rights are never restored,
    // they are only removed; this means that the function does not need to check their move counts
    // This could be done through any number of methods but this one is the simplest
    fn update_castling_rights(&mut self) {
        // White King
        if self.board.piece_at(&ChessPoint::new(4, 0)).is_none() {
            self.white_castle_kingside = false;
            self.white_castle_queenside = false;
        }

        // Black King
        if self.board.piece_at(&ChessPoint::new(4, 7)).is_none() {
            self.black_castle_kingside = false;
            self.black_castle_queenside = false;
        }

        // White Rook (right)
        if self.board.piece_at(&ChessPoint::new(7, 0)).is_none() {
            self.white_castle_kingside = false;
        }

        // White Rook (left)
        if self.board.piece_at(&ChessPoint::new(0, 0)).is_none() {
            self.white_castle_queenside = false;
        }

        // Black Rook (right)
        if self.board.piece_at(&ChessPoint::new(7, 7)).is_none() {
            self.black_castle_kingside = false;
        }

        // Black Rook (left)
        if self.board.piece_at(&ChessPoint::new(0, 7)).is_none() {
            self.black_castle_queenside = false;
        }
    }

    // Checks if a given move was an en passant move (two-tile Pawn move)
    fn is_en_passant_move(queried_move: &ChessMove, moved_piece: &ChessPiece) -> bool {
        todo!();
    }

    // Checks if a given move was an en passant capture (a Pawn capture-pattern move whose destination is an en passant tile)
    fn is_en_passant_capture(queried_move: &ChessMove, moved_piece: &ChessPiece, en_passant_tile: &Option<ChessPoint>) -> bool {
        todo!();
    }

    // Sets the en passant tile of an en passant move
    // Assumes that the move is a valid en passant move
    // FIXME: Add unreachable statements here
    fn update_for_en_passant_move(&mut self, performed_move: &ChessMove) {
        let moved_pawn = self.board.piece_at(performed_move.destination()).unwrap();

        // Get the y-coordinate of the en passant tile, which is between the move's source and destination
        let en_passant_tile_y = match moved_pawn.color {
            ChessPieceColor::White => performed_move.destination().y() + 1,
            ChessPieceColor::Black => performed_move.destination().y() - 1,
        };

        // Source or destination would work interchangeably here
        let en_passant_tile = ChessPoint::new(performed_move.destination().x(), en_passant_tile_y);

        self.en_passant_tile = Some(en_passant_tile);
    }
    
    // Removes the Pawn target of an en passant capture
    // Assumes that the move is a valid en passant capture
    fn update_for_en_passant_capture(&mut self, performed_move: &ChessMove) {
        // When an en passant capture is performed, the captured Pawn is at the same
        // y-coordinate as the source point, and the same x-coordinate as the destination point
        let tile_to_clear = ChessPoint::new(performed_move.destination().x(), performed_move.source().y());

        self.board.set_piece(&tile_to_clear, None);
    }

    // Increments or resets the move clocks, based on the move that was performed
    fn increment_move_clocks(&mut self, reset_halfmove_clock: bool) {
        self.halfmove_clock += 1;
        self.fullmove_clock += 1;

        if reset_halfmove_clock {
            self.halfmove_clock = 0;
        }
    }

    // Switches the turn color, usually after a move is performed
    fn swap_turn_color(&mut self) {
        use ChessPieceColor::*;

        self.turn_color = match self.turn_color {
            White => Black,
            Black => White,
        };
    }

    // Checks if the enemy color's King is in check
    // This can be used after a hypothetical move to test if the move would put
    // the friendly King in check, but only after the turn color has been swapped
    fn is_check(&self) -> bool {
        let king_position = match self.turn_color {
            ChessPieceColor::White => self.black_king_position,
            ChessPieceColor::Black => self.white_king_position,
        };

        // TODO: This can be optimized by only looking at the squares that enemy pieces could be in to threaten the King
        for (y, row) in self.board.pieces.iter().enumerate() {
            for (x, piece) in row.iter().enumerate() {
                if let Some(piece) = piece {
                    if piece.color == self.turn_color {
                        let move_to_check = ChessMove::new(ChessPoint::new(x, y), king_position);

                        // If any of the enemy pieces can legally move to the King's position, then the King is in check
                        if self.validate_move(&move_to_check, true).is_ok() {
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
                    Some(piece) => print!("[{}]", piece.get_piece_char()),
                    None => print!("[\u{2001}]"),
                }
            }
            println!();
        }
    }
}
