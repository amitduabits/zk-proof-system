//! Utility functions and helpers

/// Calculate the number of rows needed for a given number of elements
pub fn calculate_rows(elements: usize) -> usize {
    elements.next_power_of_two()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_rows() {
        assert_eq!(calculate_rows(5), 8);
        assert_eq!(calculate_rows(8), 8);
        assert_eq!(calculate_rows(9), 16);
    }
}
