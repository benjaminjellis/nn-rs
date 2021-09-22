#[cfg(test)]
mod tests {
    use nn_rs;
    use nalgebra;
    use std::path::PathBuf;
    use std::fs::remove_file;

    #[test]
    fn test_integration() {

        fn create_populate_load_reload(metric: String){
            let mut index: nn_rs::NearestNeighbours = nn_rs::NearestNeighbours::new(metric).unwrap();

            let a: nalgebra::DVector<f64> = nalgebra::dvector!(1.0, 2.0, 3.0);
            let b: nalgebra::DVector<f64> = nalgebra::dvector!(7.0, 2.0, 9.0);
            let c: nalgebra::DVector<f64> = nalgebra::dvector!(4.0, 2.1, 3.4);
            let d: nalgebra::DVector<f64> = nalgebra::dvector!(0.9, 8.2, 4.6);

            let e = b.clone();
            let e_two = e.clone();

            index.add_vector(String::from("a"), a).unwrap();
            index.add_vector(String::from("b"), b).unwrap();
            index.add_vector(String::from("c"), c).unwrap();
            index.add_vector(String::from("d"), d).unwrap();

            let nn_one = index.query_by_vector(e, 1).unwrap();
            assert_eq!(&nn_one, &vec![String::from("b")]);
            let save_path = PathBuf::from("./tests/data/integration_test.nn");
            let load_path = save_path.clone();
            index.save(save_path).unwrap();

            let mut new_index: nn_rs::NearestNeighbours = nn_rs::NearestNeighbours::load(load_path).unwrap();

            let nn_two = new_index.query_by_vector(e_two, 1).unwrap();
            assert_eq!(nn_one, nn_two);

        }

        let metrics = vec![String::from("euclidean"), String::from("cosine"), String::from("manhattan")];

        for metric in metrics{
            create_populate_load_reload(metric);
        }

        remove_file("./tests/data/integration_test.nn").unwrap();

    }
}
