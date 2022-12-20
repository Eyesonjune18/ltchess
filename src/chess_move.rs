use crate::ChessPoint;

#[allow(dead_code)]
pub struct ChessMove {
    pub from: ChessPoint,
    pub to: ChessPoint,
}

pub struct MoveValidity {
    pub standard: bool,
    pub capture: bool,
}

impl ChessMove {
    pub fn new(from: ChessPoint, to: ChessPoint) -> Self {
        ChessMove { from, to }
    }

    // Parse a move from a combined algebraic position string, like "e2 e4"
    pub fn from(encoded_move: &str) -> Self {
        // TODO: Make sure there are only 2 substrings
        let mut substrings = encoded_move.split_whitespace();

        let from = substrings.next().unwrap();
        let to = substrings.next().unwrap();

        let from_point = ChessPoint::from(from);
        let to_point = ChessPoint::from(to);

        ChessMove::new(from_point, to_point)
    }
}
