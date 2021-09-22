use crate::utils;
use nalgebra::{Const, Dynamic, Matrix, VecStorage};

/// Type alias for metric factory return
pub type MetricFunction = fn(
    &Matrix<f64, Dynamic, Const<1>, VecStorage<f64, Dynamic, Const<1>>>,
    &Matrix<f64, Dynamic, Const<1>, VecStorage<f64, Dynamic, Const<1>>>,
) -> f64;

/// Type for distance between two vectors
/// This type is so that the distance can be use as the key of a hash map
#[derive(Hash, Eq, PartialEq, Debug, PartialOrd, Ord)]
pub struct Distance((u64, i16, i8));


impl Distance {
    /// Constructor to convert f64 into the distance struct
    pub fn new(val: f64) -> Distance {
        Distance(utils::integer_decode(val))
    }
}