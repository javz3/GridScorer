#[cfg(test)]
mod tests {
    use grid_scorer;

    #[test]
    fn success_with_valid_input() {
        // Arrange
        let count_of_high_scores = 2;
        let row_length = 4;
        let array = [4, 2, 3, 2, 0, 1, 2, 2, 1, 3, 0, 2, 2, 0, 1, 5];

        // Act
        let result =  grid_scorer::get_top_scores(count_of_high_scores, row_length, &array);
        
        // Assert
        assert!(result.unwrap().contains("(1, 2, 17)"));
    }

    #[test]
    fn success_with_more_scores_requested_than_elements() {
        // Arrange
        let array = vec![1, 2, 3, 4];

        // Act
        let result =  grid_scorer::get_top_scores(10, 2, &array).unwrap();

        // Assert
        assert!(result.contains("(0, 0, 10)"));        
    }

    #[test]
    fn failure_with_zero_row_length() {
        // Arrange
        let array = vec![1, 2, 3, 4];

        // Act
        let result =  grid_scorer::get_top_scores(3, 0, &array);
        
        // Assert
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Invalid row_length provided");
    }

    #[test]
    fn failure_with_empty_array() {
        // Arrange
        let array = &[];

        // Act
        let result =  grid_scorer::get_top_scores(3, 3, array);

        // Assert
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Array is null");
    }

    #[test]
    fn failure_with_incorrect_array_length() {
        // Arrange
        let array = vec![1, 2, 3, 4, 5];

        // Act
        let result =  grid_scorer::get_top_scores(3, 2, &array);

        // Assert
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Array length does not match matrix size");
    }
}
