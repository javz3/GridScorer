#[cfg(test)]
mod tests {
    use grid_scorer;

    #[test]
    fn test_get_top_scores_success() {
        // Arrange
        let count_of_high_scores = 2;
        let row_length = 4;
        let array = [4, 2, 3, 2, 0, 1, 2, 2, 1, 3, 0, 2, 2, 0, 1, 5];
        let expected_result = "(1, 2, 17)(1, 1, 16)".to_string();

        // Act
        let result = grid_scorer::get_top_scores(count_of_high_scores, row_length, &array);

        // Assert
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected_result);
    }

    #[test]
    fn test_get_top_scores_failure_invalid_grid() {
        // Arrange
        let count_of_high_scores = 2;
        let row_length = 0;
        let array = [4, 2, 3, 2, 0, 1, 2, 2, 1, 3, 0, 2, 2, 0, 1, 5];

        // Act
        let result = grid_scorer::get_top_scores(count_of_high_scores, row_length, &array);

        // Assert
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Invalid row_length provided".to_string());
    }

    #[test]
    fn test_get_top_scores_failure_empty_array() {
        // Arrange
        let count_of_high_scores = 2;
        let row_length = 4;
        let array = [];

        // Act
        let result = grid_scorer::get_top_scores(count_of_high_scores, row_length, &array);

        // Assert
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Array is null".to_string());
    }

    #[test]
    fn test_get_top_scores_failure_incorrect_array_length() {
        // Arrange
        let count_of_high_scores = 2;
        let row_length = 4;
        let array = [4, 2, 3, 2, 0, 1, 2, 2, 1, 3, 0, 2, 2, 0];

        // Act
        let result = grid_scorer::get_top_scores(count_of_high_scores, row_length, &array);

        // Assert
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Array length does not equal to matrix size".to_string());
    }
}