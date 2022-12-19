use crate::ChessPoint;

#[allow(dead_code)]
pub struct ChessMove {
    from: ChessPoint,
    to: ChessPoint,
}

impl ChessMove {
    pub fn new(from: ChessPoint, to: ChessPoint) -> Self {
        ChessMove { from, to }
    }
}
