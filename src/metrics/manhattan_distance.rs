use nalgebra as na;

/// Calculate the manhattan distance between two vectors
///
/// # Parameters
/// - a: first vector
/// - b: second vector
///
/// # Return Values
/// - the manhattan distance between a and b
pub fn manhattan_distance(a: &na::DVector<f64>, b: &na::DVector<f64>) -> f64 {
    assert_eq!(
        a.shape(),
        b.shape(),
        "expected shape of a and b to be the same but got {:?} and {:?}",
        a.shape(),
        b.shape()
    );
    // manhattan distance is 0 if it's close  but cos is 1 if it's close
    (a - b).abs().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::dvector;

    #[test]
    /// test the manhattan distance returns the correct results
    fn test_manhattan_distance() {
        assert_eq!(
            manhattan_distance(&dvector!(1.0, 2.0, 3.0), &dvector!(1.0, 2.0, 3.0)),
            0.0
        );
        assert_eq!(
            manhattan_distance(&dvector!(1.0, 2.0, 3.0), &dvector!(4.0, 2.0, 7.0)),
            7.0
        );
    }
}
