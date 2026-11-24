pub struct GridPoint {
    pub x: i16,
    pub y: i16,
}

impl Default for GridPoint {
    fn default() -> Self {
        GridPoint { x: 0, y: 0 }
    }
}

impl GridPoint {
    pub fn new(x: i16, y: i16) -> Self {
        GridPoint { x, y }
    }

    pub fn add(&self, other: &GridPoint) -> GridPoint {
        GridPoint {
            x: (self.x + other.x).clamp(0, 10),
            y: (self.y + other.y).clamp(0, 10),
        }
    }

}
