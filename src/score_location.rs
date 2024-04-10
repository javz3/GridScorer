use std::fmt;

#[derive(Debug)]
pub struct ScoreLocation {
    pub x: usize,
    pub y: usize,
    pub score: i32,
}

impl ScoreLocation {
    pub fn new(x: usize, y: usize, score: i32) -> Self {
        ScoreLocation { x, y, score }
    }
}

impl fmt::Display for ScoreLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.score)
    }
}