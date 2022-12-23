#[derive(Clone, Copy, Debug)]
pub struct ChessPoint {
    x: usize,
    y: usize,
}

impl ChessPoint {
    pub fn new(x: usize, y: usize) -> ChessPoint {
        if !validate_bounds(x, y) {
            panic!(
                "[INTERNAL ERROR] Invalid point coordinates on point creation: ({}, {})",
                x, y
            );
        }

        ChessPoint { x, y }
    }

    // Parse a point from an algebraic position string, like "e2"
    pub fn from(encoded_point: &str) -> Self {
        let mut chars = encoded_point.chars();

        let x = chars.next().unwrap();
        let y = chars.next().unwrap();

        // These are ASCII values, so we can subtract 97 and 49 to get the actual values
        let x = x as usize - 97;
        let y = y as usize - 49;

        ChessPoint::new(x, y)
    }

    pub fn x(&self) -> usize {
        self.x
    }

    pub fn y(&self) -> usize {
        self.y
    }

    pub fn set_x(&mut self, x: usize) {
        if !validate_bounds(x, self.y) {
            panic!(
                "[INTERNAL ERROR] Invalid point coordinates on point X-position update: ({}, {})",
                x, self.y
            );
        }

        self.x = x;
    }

    pub fn set_y(&mut self, y: usize) {
        if !validate_bounds(self.x, y) {
            panic!(
                "[INTERNAL ERROR] Invalid point coordinates on point Y-position update: ({}, {})",
                self.x, y
            );
        }

        self.y = y;
    }

    // Get a list of points between two points, excluding the source and destination
    // Assumes that the relation between source and destination is perfectly horizontal, vertical, or diagonal
    // TODO: Check for the above condition for error handling
    pub fn get_points_between(source: &ChessPoint, destination: &ChessPoint) -> Vec<ChessPoint> {
        let mut points_between = Vec::new();

        // Source is saved to be used as an iterator
        let mut source_x = source.x() as i32;
        let mut source_y = source.y() as i32;
        let destination_x = destination.x() as i32;
        let destination_y = destination.y() as i32;

        let change_in_x = destination_x - source_x;
        let change_in_y = destination_y - source_y;

        // Each coordinate must be incremented by 1 or -1, depending on the direction
        // of travel, or 0 if the piece is not moving in that direction
        let x_increment = (change_in_x as i32).signum();
        let y_increment = (change_in_y as i32).signum();

        while source_x != destination_x || source_y != destination_y {
            source_x += x_increment;
            source_y += y_increment;

            points_between.push(ChessPoint::new(source_x as usize, source_y as usize));
        }

        // Remove the destination point so as not to forbid a capture
        points_between.pop();

        points_between
    }
}

fn validate_bounds(x: usize, y: usize) -> bool {
    x < 8 && y < 8
}
