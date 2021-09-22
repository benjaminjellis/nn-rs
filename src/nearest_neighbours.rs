use anyhow::Result;
use nalgebra as na;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fs::File;
use std::path::PathBuf;

use crate::metrics::metric_factory;
use crate::types::{Distance, MetricFunction};

#[derive(Deserialize, Serialize)]
pub struct NearestNeighbours {
    metric: String,
    vectors: HashMap<String, na::DVector<f64>>,
}

impl NearestNeighbours {
    /// Create a new, empty NearestNeighbours struct
    ///
    /// ```rust
    /// use nn_rs::NearestNeighbours;
    ///
    /// let metric = String::from("cosine");
    /// let mut index = NearestNeighbours::new(metric);
    /// ```
    ///
    /// # Parameters
    /// - metric: distance metric to use. One of "cosine", "euclidean" or "manhattan"
    ///
    /// # Return Values
    /// - NearestNeighbours struct
    pub fn new(metric: String) -> Result<NearestNeighbours> {
        let vectors: HashMap<String, na::DVector<f64>> = HashMap::new();
        Ok(NearestNeighbours { metric, vectors })
    }

    /// Create a new NearestNeighbours struct from a json
    ///
    /// This constructor should be useful for loading vectors from python matrix libraries
    /// such as torch, tensorflow, jax, numpy etc.
    ///
    /// # Parameters
    /// - metric: distance metric. One of "cosine", "euclidean" or "manhattan"
    /// - vector_file: a json of {"id_1": [1.0, 2.0, ... ], "id_2": [1.0, 2.0, ... ], ... } format
    ///
    /// # Return Values
    /// - NearestNeighbours struct
    ///
    /// Example json format
    /// ```json
    /// {
    ///     "id_1": [1.0, 2.0, ... ],
    ///     "id_2": [1.0, 2.0, ... ],
    ///     .
    ///     .
    ///     .
    ///     "id_n": [1.0, 2.0, ... ],
    /// }
    /// ```
    pub fn from_json(metric: String, vectors_file: PathBuf) -> Result<NearestNeighbours> {
        // TODO: revisit this, it's very scrappy
        // load the vectors file and parse into vectors
        let mut vectors: HashMap<String, na::DVector<f64>> = HashMap::new();
        let input_file = File::open(vectors_file)?;
        let vector_vectors: Value  = serde_json::from_reader(input_file)?;
        for (key, value) in vector_vectors.as_object().unwrap(){
            let n = value.as_array().unwrap();
            let mut vec_dummy = vec![];
            for ns in n{
                vec_dummy.push(ns.to_owned().as_f64().unwrap());
            }
            let y = na::DVector::from_vec(vec_dummy);
            vectors.insert(key.to_owned(), y);
        }
        Ok(NearestNeighbours { metric, vectors })
    }

    /// Load a NearestNeighbours struct from a .nn file
    /// # Parameters
    /// - nn_file: .nn file to load
    /// # Return Values
    /// - NearestNeighbours struct
    pub fn load(nn_file: PathBuf) -> Result<NearestNeighbours> {
        // load the nn file and turn this into a NearestNeighbours struct a parse into vector
        let input_path = File::open(nn_file)?;
        let new_nn_struct: NearestNeighbours = serde_json::from_reader(input_path)?;
        Ok(new_nn_struct)
    }

    /// Add a new vector to the NearestNeighbour struct
    ///
    /// Note: the id should be unique. If it isn't, only the vector associated with the id that was most
    /// recently added will be kept
    ///
    /// # Parameters
    /// - id: the id of the vector, this can be any string but it must be unique
    /// - vector: the vector to add
    /// # Return Values
    /// - nothing
    pub fn add_vector(&mut self, id: String, vector: na::DVector<f64>) -> Result<()> {
        self.vectors.insert(id, vector);
        Ok(())
    }

    /// Save the NearestNeighbour struct to a .nn file
    /// # Parameters
    /// - output_path: path to save the struct to
    /// # Return Values
    /// - nothing
    pub fn save(&mut self, output_path: PathBuf) -> Result<()> {
        let output_file = File::create(output_path)?;
        serde_json::to_writer(output_file, &self)?;
        Ok(())
    }

    /// Find the nearest neighbour for a query vector
    ///
    /// # Parameters
    /// - query_vector: vector to find the nearest neighbour to
    /// - no_neighbours: the number of nearest neighbours to find
    ///
    /// # Return Values
    /// - ids of the nearest neighbours
    pub fn query_by_vector(
        &mut self,
        query_vector: na::DVector<f64>,
        no_neighbours: usize,
    ) -> Result<Vec<String>> {
        // calculate and store the distances between the vectors
        let mut distances = HashMap::new();
        let metric: MetricFunction = metric_factory(&self.metric)?;
        for (index, vector) in &self.vectors {
            distances.insert(Distance::new(metric(vector, &query_vector)), index);
        }

        // get the distances and sort to find the smallest distances
        let mut all_distances: Vec<&Distance> = distances.iter().map(|(dist, _id)| dist).collect();
        all_distances.sort();

        // find the ids for the smallest distances and return them
        let mut nns = vec![];
        for neighbour in all_distances.iter().take(no_neighbours) {
            let id: &String = &distances[neighbour];
            let id_owned: String = id.to_string();
            nns.push(id_owned);
        }
        Ok(nns)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use na::dvector;
    use std::fs::remove_file;
    use anyhow::Result;

    #[test]
    fn test_nearest_neighbours_new() -> Result<()>  {
        NearestNeighbours::new(String::from("cosine"))?;
        Ok(())
    }

    #[test]
    fn test_nearest_neighbours_from_json() -> Result<()> {
        let json_path = PathBuf::from("./tests/data/test.json");
        let metric = String::from("cosine");
        let _index = NearestNeighbours::from_json(metric, json_path)?;
        Ok(())
    }

    #[test]
    fn test_nearest_neighbours_load() -> Result<()> {
        let output_file = PathBuf::from(r"./tests/data/test.nn");
        let _index = NearestNeighbours::load(output_file)?;
        Ok(())
    }

    #[test]
    fn test_nearest_neighbours_add_vector() -> Result<()> {
        let mut index = NearestNeighbours::new(String::from("cosine"))?;
        let new_vector = dvector!(1.0, 2.0);
        index.add_vector(String::from("one"), new_vector)?;
        Ok(())
    }

    #[test]
    fn test_nearest_neighbours_save() -> Result<()> {
        let mut index = NearestNeighbours::new(String::from("cosine"))?;
        let new_vector = dvector!(1.0, 2.0);
        index.add_vector(String::from("one"), new_vector)?;
        let output_file = PathBuf::from(r"./tests/data/test_save.nn");
        index.save(output_file.clone())?;
        remove_file(output_file)?;
        Ok(())
    }

    #[test]
    fn test_nearest_neighbours_query_by_vector() -> Result<()>  {
        let mut index = NearestNeighbours::new(String::from("cosine"))?;
        let new_vector = dvector!(1.0, 2.0);
        index.add_vector(String::from("one"), new_vector)?;
        let new_vector_2 = dvector!(9.0, 7.0);
        index.add_vector(String::from("two"), new_vector_2)?;
        let new_vector_3 = dvector!(1.0, 2.0);
        let nn = index.query_by_vector(new_vector_3, 1)?;
        assert_eq!(nn, vec!["one"]);
        Ok(())
    }
}