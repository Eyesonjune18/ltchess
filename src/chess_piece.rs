use crate::MoveValidity;
use crate::ChessMove;

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

    pub fn can_move_to(&self, queried_move: ChessMove) -> MoveValidity {
        match self.kind {
            ChessPieceKind::Pawn => {
                MoveValidity {
                    standard: true,
                    capture: true,
                }
            }
            ChessPieceKind::Rook => {
                MoveValidity {
                    standard: true,
                    capture: true,
                }
            }
            ChessPieceKind::Knight => {
                MoveValidity {
                    standard: true,
                    capture: true,
                }
            }
            ChessPieceKind::Bishop => {
                MoveValidity {
                    standard: true,
                    capture: true,
                }
            }
            ChessPieceKind::Queen => {
                MoveValidity {
                    standard: true,
                    capture: true,
                }
            }
            ChessPieceKind::King => {
                MoveValidity {
                    standard: true,
                    capture: true,
                }
            }
        }
    }
}
