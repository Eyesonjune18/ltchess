pub struct ChessPoint {
    x: usize,
    y: usize,
}

impl ChessPoint {
    pub fn new(x: usize, y: usize) -> ChessPoint {
        if !validate_bounds(x, y) {
            panic!(
                "Invalid point coordinates on point creation: ({}, {})",
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
                "Invalid point coordinates on point X-position update: ({}, {})",
                x, self.y
            );
        }

        self.x = x;
    }

    pub fn set_y(&mut self, y: usize) {
        if !validate_bounds(self.x, y) {
            panic!(
                "Invalid point coordinates on point Y-position update: ({}, {})",
                self.x, y
            );
        }

        self.y = y;
    }
}

fn validate_bounds(x: usize, y: usize) -> bool {
    x < 8 && y < 8
}
