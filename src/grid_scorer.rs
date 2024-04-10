use rayon::prelude::*;
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
fn calculate_scores_parallel(array: &[i32], row_length: usize) -> Vec<ScoreLocation> {
    (0..row_length*row_length).into_par_iter().map(|idx| {
        let x = idx / row_length;
        let y = idx % row_length;
        let mut score = 0;
        for dx in -1..=1 {
            for dy in -1..=1 {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                if nx >= 0 && nx < row_length as i32 && ny >= 0 && ny < row_length as i32 {
                    score += array[(nx as usize) * row_length + ny as usize];
                }
            }
        }
        ScoreLocation::new(x, y, score)
    }).collect()
}

/*
   - This function sorts the scores in parallel using par_sort_unstable_by, a parallel sorting algorithm provided by rayon. 
   - It's "unstable" because it doesn't guarantee the order of equal elements, which is fine here since we care about the order based on scores.
   - After sorting, it truncates the vector to keep only the top N scores.
 */
fn sort_scores_parallel(mut scores: Vec<ScoreLocation>, count: usize) -> Vec<ScoreLocation> {
    scores.par_sort_unstable_by(|a, b| b.score.cmp(&a.score).then_with(|| a.y.cmp(&b.y)).then_with(|| a.x.cmp(&b.x)));
    scores.truncate(count);
    scores
}

fn format_scores(scores: &[ScoreLocation]) -> String {
    scores.iter().map(|s| s.to_string()).collect::<Vec<_>>().join(", ")
}
