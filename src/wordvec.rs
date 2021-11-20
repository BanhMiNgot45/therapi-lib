pub mod wordvec {
    use crate::math::Math::Math;
    use std::collections::HashMap;

    #[derive(Clone)]
    pub struct Model {
        pub w1: Vec<Vec<f32>>,
        pub w2: Vec<Vec<f32>>
    }

    impl Model {
        pub fn new(vocab_size: usize, n_embeddings: usize) -> Model {
            Model {
                w1: Math::initialize_rand_matrix(vocab_size, n_embeddings),
                w2: Math::initialize_rand_matrix(n_embeddings, vocab_size)
            }
        }

        pub fn forward(self, x: Vec<Vec<f32>>) -> HashMap<String, Vec<Vec<f32>>> {
            let mut cache: HashMap<String, Vec<Vec<f32>>> = HashMap::new();
            cache.insert("a1".to_owned(), Math::matrix_multiplication(x, self.w1));
            cache.insert("a2".to_owned(), Math::matrix_multiplication(cache.get("a1").unwrap().to_vec(), self.w2));
            cache.insert("z".to_owned(), Math::matrix_softmax(cache.get("a2").unwrap().to_vec()));
            cache
        }

        pub fn backward(mut self, x: Vec<Vec<f32>>, y: Vec<Vec<f32>>, alpha: f32) -> f32 {
            let cache = self.clone().forward(x.clone());
            let da2: Vec<Vec<f32>> = Math::matrix_subtraction(cache.get("z").unwrap().to_vec(), y.clone());
            let dw2: Vec<Vec<f32>> = Math::matrix_multiplication(Math::transpose(cache.get("a1").unwrap().to_vec()), da2.clone());
            let da1: Vec<Vec<f32>> = Math::matrix_multiplication(da2, Math::transpose(self.w2.clone()));
            let dw1: Vec<Vec<f32>> = Math::matrix_multiplication(Math::transpose(x), da1);
            self.w1 = Math::matrix_subtraction(self.w1, Math::matrix_scalar_multiplication(alpha, dw1));
            self.w2 = Math::matrix_subtraction(self.w2, Math::matrix_scalar_multiplication(alpha, dw2));
            -Math::matrix_sum(Math::matrix_multiplication(Math::matrix_log(cache.get("z").unwrap().to_vec()), y))
        }
    }
}