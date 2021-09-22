use nalgebra as na;

/// Calculate the cosine distance between two vectors
///
/// # Parameters
/// - a: first vector
/// - b: second vector
///
/// # Return Values
/// - the cosine distance between a and b
pub fn cosine_distance(a: &na::DVector<f64>, b: &na::DVector<f64>) -> f64 {
    assert_eq!(
        a.shape(),
        b.shape(),
        "expected shape of a and b to be the same but got {:?} and {:?}",
        a.shape(),
        b.shape()
    );
    1f64 - a.dot(b) / (a.norm() * b.norm())
}

#[cfg(test)]
mod tests {
    use super::*;
    use na::dvector;

    #[test]
    /// Test that cosine distances gives the correct results
    fn test_cosine_distance() {

        assert_eq!(
            cosine_distance(&dvector!(1.0, 2.0, 3.0), &dvector!(1.0, 2.0, 3.0)),
            0.0
        );
        assert_eq!(
            cosine_distance(&dvector!(1.0, 2.0, 3.0), &dvector!(4.0, 2.0, 7.0)),
            0.0669402944727091
        );
    }
}
