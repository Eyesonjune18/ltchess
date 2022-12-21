pub enum ChessError {
    InvalidMovePattern,
    MoveCollisionOccurs,
    CannotCaptureFriendly,
    CannotSelfCheck,
    EnemyPieceAtMoveSource,
    NoPieceAtMoveSource,
}
