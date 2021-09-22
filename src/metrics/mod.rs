pub use self::cosine_distance::cosine_distance;
mod cosine_distance;

pub use self::euclidean_distance::euclidean_distance;
mod euclidean_distance;

pub use self::manhattan_distance::manhattan_distance;
mod manhattan_distance;

pub use self::metric_factory::metric_factory;
mod metric_factory;
