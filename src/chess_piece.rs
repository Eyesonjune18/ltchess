use crate::ChessMove;
use crate::MoveValidity;

#[derive(Clone, Copy)]
pub struct ChessPiece {
    pub kind: ChessPieceKind,
    pub color: ChessPieceColor,
    pub move_count: u32,
}

#[derive(Clone, Copy)]
pub enum ChessPieceKind {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

#[derive(Clone, Copy)]
pub enum ChessPieceColor {
    White,
    Black,
}

impl ChessPiece {
    pub fn new(kind: ChessPieceKind, color: ChessPieceColor) -> Self {
        ChessPiece {
            kind,
            color,
            move_count: 0,
        }
    }

    // Gets the Unicode character representing the piece
    pub fn get_piece_char(&self) -> char {
        use ChessPieceColor::*;
        use ChessPieceKind::*;

        match (&self.color, &self.kind) {
            (White, Pawn) => '♟',
            (White, Rook) => '♜',
            (White, Knight) => '♞',
            (White, Bishop) => '♝',
            (White, Queen) => '♛',
            (White, King) => '♚',
            (Black, Pawn) => '♙',
            (Black, Rook) => '♖',
            (Black, Knight) => '♘',
            (Black, Bishop) => '♗',
            (Black, Queen) => '♕',
            (Black, King) => '♔',
        }
    }

    // Checks whether the piece can move to the given square, based only on its movement patterns
    pub fn can_make_move(&self, queried_move: &ChessMove) -> MoveValidity {
        let change_in_x: u32 = (queried_move.destination().x() as i32 - queried_move.source().x() as i32).abs() as u32;
        let change_in_y_unadjusted = queried_move.destination().y() as i32 - queried_move.source().y() as i32;
        let change_in_y: u32 = change_in_y_unadjusted.abs() as u32;

        let vertical_or_horizontal_pattern = (change_in_x == 0 && change_in_y != 0) ^ (change_in_x != 0 && change_in_y == 0);
        let diagonal_pattern = change_in_x == change_in_y;

        use ChessPieceKind::*;
        use ChessPieceColor::*;

        match self.kind {
            Pawn => {
                // Pawn y-axis movement rules are inverted for black pieces,
                // then adjusted to two squares intead of one for the first move
                // This code is a bit verbose, but it's also more readable than boolean arithmetic
                let pawn_standard_move = {
                    // TODO: En passant is forced on first move, make this allow either
                    change_in_x == 0 && change_in_y_unadjusted == match (self.color, self.move_count) {
                        (White, 0) => 2,
                        (White, _) => 1,
                        (Black, 0) => -2,
                        (Black, _) => -1,
                    }
                };

                let pawn_capture_move = {
                    change_in_x == 1 && change_in_y_unadjusted == match self.color {
                        White => 1,
                        Black => -1,
                    }
                };

                MoveValidity {
                    standard: pawn_standard_move,
                    capture: pawn_capture_move,
                }
            },
            Rook => {
                let rook_move = vertical_or_horizontal_pattern;

                MoveValidity {                
                    standard: rook_move,
                    capture: rook_move,
                }
            },
            Knight => {
                let knight_move = (change_in_x == 1 && change_in_y == 2) ^ (change_in_x == 2 && change_in_y == 1);

                MoveValidity {
                    standard: knight_move,
                    capture: knight_move,
                }
            },
            Bishop => {
                let bishop_move = diagonal_pattern;

                MoveValidity {
                    standard: bishop_move,
                    capture: bishop_move,
                }
            },
            Queen => {
                let queen_move = vertical_or_horizontal_pattern ^ diagonal_pattern;

                MoveValidity {
                    standard: queen_move,
                    capture: queen_move,
                }
            },
            King => {
                let king_move = (change_in_x <= 1 && change_in_y <= 1) && !(change_in_x == 0 && change_in_y == 0);
                
                MoveValidity {
                    standard: king_move,
                    capture: king_move,
                }
            },
        }
    }
}
