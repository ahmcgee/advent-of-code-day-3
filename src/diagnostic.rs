use crate::grid::{DynamicHeightGrid, Transpose};

pub fn calculate_diagnostic_result(grid: &DynamicHeightGrid<bool>) -> (Vec<bool>, Vec<bool>) {
    let transposed_grid = grid.transpose();
    let gamma_value: Vec<bool> = transposed_grid.iter().map(| x | most_common(x.to_vec())).collect();
    let epsilon_value: Vec<bool> = gamma_value.iter().map(|x| ! x).collect();
    (gamma_value, epsilon_value)
}

fn most_common(bool_vec: Vec<bool>) -> bool {
    !unimplemented!()
}


#[cfg(test)]
mod tests {
    use std::vec;

    use crate::grid::DynamicHeightGrid;

    use super::calculate_diagnostic_result;

    #[test]
    fn calculate_diagnostic_result_example_pass() {
        // arrange
        let mut grid = DynamicHeightGrid::<bool>::new(5);

        grid.push(vec![false, false, true, false, false]).unwrap();
        grid.push(vec![true, true, true, true, false]).unwrap();
        grid.push(vec![true, false, true, true, false]).unwrap();
        grid.push(vec![true, false, true, true, true]).unwrap();
        grid.push(vec![true, false, true, false, true]).unwrap();
        grid.push(vec![false, true, true, true, true]).unwrap();
        grid.push(vec![false, false, true, true, true]).unwrap();
        grid.push(vec![true, true, true, false, false]).unwrap();
        grid.push(vec![true, false, false, false, false]).unwrap();
        grid.push(vec![true, true, false, false, true]).unwrap();
        grid.push(vec![false, false, false, true, false]).unwrap();
        grid.push(vec![false, true, false, true, false]).unwrap();

        // act
        let result = calculate_diagnostic_result(&grid);
        
        // assert 
        // Check gamma rate
        assert_eq!(result.0, vec![true, false, true, true, false]);
        // Check epsilon rate
        assert_eq!(result.1, vec![false, true, false, false, true]);

    }

    #[test]
    #[should_panic(expected="Input grid cannot be empty. Watcha doin!?")]
    fn calculate_diagnostic_result_raise_error_from_empty_grid() {
        // arrange
        let mut grid = DynamicHeightGrid::<bool>::new(5);

        // act
        let result = calculate_diagnostic_result(&grid);
        
        // panic 
        

    }
}
