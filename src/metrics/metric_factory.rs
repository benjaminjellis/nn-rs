use anyhow::Result;

use crate::metrics;
use crate::types::MetricFunction;

/// Factory function to create the metric function for calculating distance
/// between two vectors
///
/// # Parameters
/// - metric: which metric function to create
///
/// # Return values
/// - MetricFunction
pub fn metric_factory(metric: &str) -> Result<MetricFunction> {
    match metric {
        "cosine" => Ok(metrics::cosine_distance),
        "manhattan" => Ok(metrics::manhattan_distance),
        "euclidean" => Ok(metrics::euclidean_distance),
        _ => panic!("Did not recognise metric {}", metric),
    }
}


#[cfg(test)]
mod tests {

    use super::*;
    use anyhow::Result;
    use crate::types::MetricFunction;

    #[test]
    #[ignore]
    fn test_metric_factory() -> Result<()> {
        // is this redundant? we already check that the metrics are fine. so here
        // we want to check that the factory fn dispatches metric fns but this can't not be
        // true since in the case that it didn't it wouldn't compile
        let metrics = vec!["euclidean", "cosine", "manhattan"];
        for metric in metrics{
            let _metric_fn: MetricFunction  = metric_factory(metric)?;
        }
        Ok(())
    }
}
