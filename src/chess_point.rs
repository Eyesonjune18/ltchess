pub struct ChessPoint {
    x: u8,
    y: u8,
}

impl ChessPoint {
    pub fn new(x: u8, y: u8) -> ChessPoint {
        if !validate_bounds(x, y) {
            panic!("Invalid point coordinates on point creation: ({}, {})", x, y);
        }

        ChessPoint { x, y }
    }

    pub fn set_x(&mut self, x: u8) {
        if !validate_bounds(x, self.y) {
            panic!("Invalid point coordinates on point X-position update: ({}, {})", x, self.y);
        }

        self.x = x;
    }

    pub fn set_y(&mut self, y: u8) {
        if !validate_bounds(self.x, y) {
            panic!("Invalid point coordinates on point Y-position update: ({}, {})", self.x, y);
        }

        self.y = y;
    }
}

fn validate_bounds(x: u8, y: u8) -> bool {
    x < 8 && y < 8
}
