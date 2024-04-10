use std::fmt;

// Struct to hold the score along with its location
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

// Implementing Display trait for ScoreLocation to customize printing
impl fmt::Display for ScoreLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.score)
    }
}

// Main function to calculate and return top scores in formatted string
pub fn get_top_scores(count_of_high_scores: usize, row_length: usize, array: &[i32]) -> Result<String, String> {
    if row_length <= 0 {
        return Err("Invalid row_length provided".to_string());
    }
    if array.is_empty() {
        return Err("Array is null".to_string());
    }
    if array.len() != row_length * row_length {
        return Err("Array length does not equal to matrix size".to_string());
    }

    let grid = convert_array_to_grid(row_length, array);
    let scores = calculate_scores(&grid, row_length);
    let sorted_scores = sort_scores(scores, count_of_high_scores);
    Ok(format_scores(&sorted_scores))
}

// Converts a linear array to a 2D grid
fn convert_array_to_grid(row_length: usize, array: &[i32]) -> Vec<Vec<i32>> {
    array.chunks(row_length).map(|chunk| chunk.to_vec()).collect()
}

// Calculates scores for each grid location
fn calculate_scores(grid: &[Vec<i32>], row_length: usize) -> Vec<ScoreLocation> {
    let mut scores = Vec::with_capacity(row_length * row_length);

    for x in 0..row_length {
        for y in 0..row_length {
            let mut score = 0;
            for dx in -1..=1 {
                for dy in -1..=1 {
                    let nx = x as i32 + dx;
                    let ny = y as i32 + dy;
                    if nx >= 0 && nx < row_length as i32 && ny >= 0 && ny < row_length as i32 {
                        score += grid[nx as usize][ny as usize];
                    }
                }
            }
            scores.push(ScoreLocation::new(x, y, score));
        }
    }

    scores
}

// Sorts scores and locations, selecting top entries
fn sort_scores(mut scores: Vec<ScoreLocation>, count: usize) -> Vec<ScoreLocation> {
    scores.sort_by(|a, b| b.score.cmp(&a.score).then_with(|| a.y.cmp(&b.y)).then_with(|| a.x.cmp(&b.x)));
    scores.truncate(count);
    scores
}

// Formats the scores into the required string output
fn format_scores(scores: &[ScoreLocation]) -> String {
    scores.iter().map(|s| s.to_string()).collect::<String>()
}
