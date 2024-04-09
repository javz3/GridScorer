mod grid_scorer;

fn main() {
    let count_of_high_scores = 2;
    let row_length = 4;
    let array = [4, 2, 3, 2, 0, 1, 2, 2, 1, 3, 0, 2, 2, 0, 1, 5];
    match grid_scorer::get_top_scores(count_of_high_scores, row_length, &array) {
        Ok(result) => println!("{}", result),
        Err(err) => println!("Error: {}", err),
    }
}
