use crate::ChessPoint;

#[allow(dead_code)]
pub struct ChessMove {
    from: ChessPoint,
    to: ChessPoint,
}

pub struct MoveValidity {
    pub standard: bool,
    pub capture: bool,
}

impl ChessMove {
    pub fn new(from: ChessPoint, to: ChessPoint) -> Self {
        ChessMove { from, to }
    }
}
