#[cfg(test)]
mod tests {
    use grid_scorer;

    #[test]
    fn test_get_top_scores() {

        // Arrange
        let count_of_high_scores = 2;
        let row_length = 4;
        let array = [4, 2, 3, 2, 0, 1, 2, 2, 1, 3, 0, 2, 2, 0, 1, 5];

        // Act
        let result = grid_scorer::get_top_scores(count_of_high_scores, row_length, &array);
        
        // Assert
        assert!(result.is_ok());
        let result_string = result.unwrap().to_string();
        assert_eq!(result_string, "(1, 2, 17)(1, 1, 16)".to_string());
    }
}
