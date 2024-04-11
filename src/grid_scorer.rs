use rayon::prelude::*;
use crate::score_location::ScoreLocation;

// Main function to calculate and return top scores in formatted string
pub fn get_top_scores(count_of_high_scores: usize, row_length: usize, array: &[i32]) -> Result<String, String> {
    if row_length == 0 {
        return Err("Invalid row length provided".to_string());
    }
    if array.is_empty() {
        return Err("Array is null".to_string());
    }
    if array.len() != row_length * row_length {
        return Err("Array length does not match matrix size".to_string());
    }

    let scores = calculate_scores_parallel(array, row_length);
    let sorted_scores = sort_scores_parallel(scores, count_of_high_scores);
    Ok(format_scores(&sorted_scores))
}

/*
   - Instead of creating a 2D grid, this function works directly on the 1D array for memory efficiency.
   - It uses the rayon crate (a parallel iterator - .into_par_iter()) to parallelise the loop, computing
     each score in parallel to utilise multi-core processors effectively.
*/
fn calculate_scores_parallel(grid_values: &[i32], grid_size: usize) -> Vec<ScoreLocation> {
    (0..grid_size * grid_size).into_par_iter().map(|cell_index| {
        let row = cell_index / grid_size;
        let column = cell_index % grid_size;
        let mut cell_score = 0;
        for row_offset in -1..=1 {
            for column_offset in -1..=1 {
                let neighbor_row = row as i32 + row_offset;
                let neighbor_column = column as i32 + column_offset;
                if neighbor_row >= 0 && neighbor_row < grid_size as i32 && neighbor_column >= 0 && neighbor_column < grid_size as i32 {
                    cell_score += grid_values[(neighbor_row as usize) * grid_size + neighbor_column as usize];
                }
            }
        }
        ScoreLocation::new(row, column, cell_score)
    }).collect()
}


/*
   - This function sorts the scores in parallel using par_sort_unstable_by, a parallel sorting algorithm provided by rayon. 
   - It's "unstable" because it doesn't guarantee the order of equal elements, which is fine here since we care about the order based on scores.
   - After sorting, it truncates the vector to keep only the top N scores.
 */
fn sort_scores_parallel(mut score_locations: Vec<ScoreLocation>, top_count: usize) -> Vec<ScoreLocation> {
    score_locations.par_sort_unstable_by(|first_location, second_location| 
        second_location.score.cmp(&first_location.score)
        .then_with(|| first_location.y.cmp(&second_location.y))
        .then_with(|| first_location.x.cmp(&second_location.x))
    );
    score_locations.truncate(top_count);
    score_locations
}

fn format_scores(score_locations: &[ScoreLocation]) -> String {
    score_locations.iter()
    .map(ScoreLocation::to_string)
    .collect::<Vec<_>>()
    .join(", ")
}
