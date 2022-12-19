pub trait ChessPiece {
    fn is_valid_move_pattern() -> MoveValidity;
}

#[allow(dead_code)]
pub struct MoveValidity {
    standard: bool,
    capture: bool,
}
