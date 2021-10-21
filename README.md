![build status](https://github.com/benjaminjellis/nn-rs/actions/workflows/ci.yml/badge.svg)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://github.com/benjaminjellis/nn-rs/blob/main/LICENSE)
[![crates.io](https://img.shields.io/crates/v/nn-rs.svg)](https://crates.io/crates/nn-rs)
[![Documentation](https://docs.rs/nn-rs/badge.svg)](https://docs.rs/nn-rs)


# nn-rs
nn-rs is a pure Rust library for finding the nearest neighbours for 1-D vectors using [nalgebra](https://github.com/dimforge/nalgebra). 

## Examples 

You can create an empty NearestNeighbour Index and add vectors to it 
```rust
use nn_rs::NearestNeighbours;
use nalgebra;

// pick a metric to use 
let metric = String::from("cosine");
// create an empty index
let mut index: NearestNeighbours = NearestNeighbours::new(metric)?;

// create some dummy vectors 
let a: nalgebra::DVector<f64> = nalgebra::dvector!(1.0, 2.0, 3.0);
let b: nalgebra::DVector<f64> = nalgebra::dvector!(7.0, 2.0, 9.0);
let c: nalgebra::DVector<f64> = nalgebra::dvector!(4.0, 2.1, 3.4);
let d: nalgebra::DVector<f64> = nalgebra::dvector!(0.9, 8.2, 4.6);

// add these dummy vectors to the index
index.add_vector(String::from("a"), a)?;
index.add_vector(String::from("b"), b)?;
index.add_vector(String::from("c"), c)?;
index.add_vector(String::from("d"), d)?;
``` 

You can then save this to a .nn file which be can re-loaded  
```rust 
use std::path::PathBuf;

let save_path = PathBuf::from("./test.nn");
index.save(save_path)?;

let load_path = PathBuf::from("./test.nn");
let mut new_index = NearestNeighbours.load(load_path)?;
```

Alternatively, you can create the index from a json

```json 
{
    "a": [1.0, 2.0, 3.0],
    "b": [7.0, 2.0, 9.0],
    "c": [4.0, 2.1, 3.4],
    "d": [0.9, 8.2, 4.6]
}
```

```rust
let json_path = PathBuf::from("some.json");
let metric = String::from("cosine");
let mut index = NearestNeighbours::from_json(metric, json_path)?;
```

Once you have an index you can then query by vector to find the nearest <i>n</i> vectors 
```rust
let query_vector: nalgebra::DVector<f64> = nalgebra::dvector!(1.0, 2.0, 3.0);
// the number of neighbours to return
let n: uszie = 1;
// find just the single nearest neighbour in the index 
let nearest_neighbour = index.query_by_vector(query_vector, n)?;
```

## Install 
Add the following line to your Cargo.toml file:

```toml
[dependencies]
nn-rs = "0.1.2"
```

## Features 

- [cosine distance](https://en.wikipedia.org/wiki/Cosine_similarity), [euclidean distance](https://en.wikipedia.org/wiki/Euclidean_distance) and [manhattan distance](https://en.wikipedia.org/wiki/Taxicab_geometry)
- exact nearest neighbour(s)
- pure rust
- serialisable, once you've built an index it can easily be shared or saved
- dynamic, new vectors can be added to any index 

## 🗡️🗡️ Sharp Edges 🗡️🗡️

- duplicate ids can't be held, if a duplicate id is added it will overwrite the already present entry
- add_vector doesn't check that you're adding vectors of the same length, when you go 
to query this will throw an error so care should be taken to ensure all added vectors are of the same length

