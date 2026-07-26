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
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        for elem in [rank, file] {
            if !(0 <= elem && elem <= 7) {
                return None;
            }
        }
        Some(Self { x: rank, y: file })
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
