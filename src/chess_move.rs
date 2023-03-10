use crate::ChessPoint;

pub struct ChessMove {
    source: ChessPoint,
    destination: ChessPoint,
}

pub struct MovePatternValidity {
    pub standard: bool,
    pub capture: bool,
}

impl ChessMove {
    // TODO: Decide on from/to or source/destination
    pub fn new(source: ChessPoint, destination: ChessPoint) -> Self {
        ChessMove {
            source,
            destination,
        }
    }

    // Parse a move from a combined tile position string, like "e2 e4"
    pub fn from(encoded_move: &str) -> Self {
        // TODO: Make sure there are only 2 substrings
        let mut substrings = encoded_move.split_whitespace();

        let from = substrings.next().unwrap();
        let to = substrings.next().unwrap();

        let from_point = ChessPoint::from(from);
        let to_point = ChessPoint::from(to);

        ChessMove::new(from_point, to_point)
    }

    pub fn change_in_x(&self) -> i8 {
        self.destination.x() as i8 - self.source.x() as i8
    }

    pub fn change_in_y(&self) -> i8 {
        self.destination.y() as i8 - self.source.y() as i8
    }

    pub fn source(&self) -> &ChessPoint {
        &self.source
    }

    pub fn destination(&self) -> &ChessPoint {
        &self.destination
    }
}
