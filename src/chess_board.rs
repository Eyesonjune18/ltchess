use crate::ChessPiece;
use crate::ChessPieceColor;
use crate::ChessPieceKind;
use crate::ChessPoint;

#[derive(Clone, Copy)]
pub struct ChessBoard {
    pub pieces: [[Option<ChessPiece>; 8]; 8],
}

impl ChessBoard {
    // Creates a board from the default starting position
    pub fn new() -> Self {
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
