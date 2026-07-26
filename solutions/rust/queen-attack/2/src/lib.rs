#[derive(Debug)]
pub struct ChessPosition {
    x: i32,
    y: i32,
}

#[derive(Debug)]
pub struct Queen {
    pos: ChessPosition,
}

impl ChessPosition {
    pub fn new(x: i32, y: i32) -> Option<Self> {
        if (0..=7).contains(&x) && (0..=7).contains(&y) {
            Some(Self { x, y })
        } else {
            None
        }
    }
}

impl Queen {
    pub fn new(pos: ChessPosition) -> Self {
        Self { pos }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        if self.pos.x == other.pos.x || self.pos.y == other.pos.y {
            return true;
        }
        let dx: i32 = (self.pos.x - other.pos.x).abs();
        let dy: i32 = (self.pos.y - other.pos.y).abs();
        dx == dy
    }
}
