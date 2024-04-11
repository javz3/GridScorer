use crate::score_location::ScoreLocation;

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

    for row_index in 0..row_length {
        for col_index in 0..row_length {
            let mut cell_score = 0;
            for neighbor_row_offset in -1..=1 {
                for neighbor_col_offset in -1..=1 {
                    let neighbor_row = row_index as i32 + neighbor_row_offset;
                    let neighbor_col = col_index as i32 + neighbor_col_offset;
                    if neighbor_row >= 0 && neighbor_row < row_length as i32 && neighbor_col >= 0 && neighbor_col < row_length as i32 {
                        cell_score += grid[neighbor_row as usize][neighbor_col as usize];
                    }
                }
            }
            scores.push(ScoreLocation::new(row_index, col_index, cell_score));
        }
    }    

    scores
}

// Sorts scores and locations, selecting top entries
fn sort_scores(mut score_locations: Vec<ScoreLocation>, top_n: usize) -> Vec<ScoreLocation> {
    score_locations.sort_by(|first, second| second.score.cmp(&first.score)
        .then_with(|| first.y.cmp(&second.y))
        .then_with(|| first.x.cmp(&second.x)));
    score_locations.truncate(top_n);
    score_locations
}


// Formats the scores into the required string output
fn format_scores(score_locations: &[ScoreLocation]) -> String {
    score_locations.iter().map(|score_location| score_location.to_string()).collect::<String>()
}
