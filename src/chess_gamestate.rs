pub struct ChessGamestate {
    pub board: [[Option<ChessPiece>; 8]; 8],
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
