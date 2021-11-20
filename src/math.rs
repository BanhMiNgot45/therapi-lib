pub mod Math {
    use rand::prelude::*;

    pub struct Math {
    }

    impl Math {
        pub fn matrix_multiplication(vec_one: Vec<Vec<f32>>, vec_two: Vec<Vec<f32>>) -> Vec<Vec<f32>> {
            if vec_one[0].len() != vec_two.len() {
                panic!("Dimensions of matrices are incompatible for multiplication!");
            }
            let mut result = Math::initialize_zero_matrix(vec_one.len(), vec_two[0].len());
            let mut i = 0;
            let mut j = 0;
            let mut k = 0;
            loop {
                loop {
                    loop {
                        result[i][j] = vec_one[i][k] * vec_two[k][j];
                        k += 1;
                        if k == vec_one[0].len() {
                            break;
                        }
                    }
                    j += 1;
                    if j == result[i].len() {
                        break;
                    } 
                }
                i += 1;
                if i == result.len() {
                    break;
                }
            }
            result
        }

        pub fn initialize_zero_matrix(rows: usize, cols: usize) -> Vec<Vec<f32>> {
            let result: Vec<Vec<f32>> = Vec::new();
            let mut r = 1;
            let mut c = 1;
            loop {
                let mut vec: Vec<f32> = Vec::new();
                loop {
                    vec.push(0.0);
                    c += 1;
                    if c > cols {
                        break;
                    }
                }
                r += 1;
                if r > rows {
                    break;
                }
            }
            result
        }

        pub fn initialize_rand_matrix(rows: usize, cols: usize) -> Vec<Vec<f32>> {
            let result: Vec<Vec<f32>> = Vec::new();
            let mut rng = rand::thread_rng();
            let mut r = 1;
            let mut c = 1;
            loop {
                let mut vec: Vec<f32> = Vec::new();
                loop {
                    vec.push(rng.gen());
                    c += 1;
                    if c > cols {
                        break;
                    }
                }
                r += 1;
                if r > rows {
                    break;
                }
            }
            result
        }

        pub fn matrix_softmax(vec: Vec<Vec<f32>>) -> Vec<Vec<f32>> {
            let mut result: Vec<Vec<f32>> = Vec::new();
            for v in vec {
                let mut dummy: Vec<f32> = Vec::new();
                for num in v {
                    dummy.push(std::f32::consts::E.powf(num));
                }
                let mut sum = 0.0;
                let mut a = 0;
                loop {
                    sum += dummy[a];
                    a += 1;
                    if a == dummy.len() {
                        break;
                    }
                }
                let mut temp: Vec<f32> = Vec::new();
                let mut i = 0;
                loop {
                    temp.push(dummy[i] / sum);
                    i += 1;
                    if i == dummy.len() {
                        break;
                    }
                }
                result.push(temp);
            }
            result
        }

        pub fn matrix_subtraction(vec_one: Vec<Vec<f32>>, vec_two: Vec<Vec<f32>>) -> Vec<Vec<f32>> {
            if vec_one.len() != vec_two.len() && vec_one[0].len() != vec_two[0].len() {
                panic!("Dimensions of matrices are incompatible for subtraction!");
            }
            let mut result: Vec<Vec<f32>> = Math::initialize_zero_matrix(vec_one.len(), vec_one[0].len());
            let mut i = 0;
            let mut j = 0;
            loop {
                loop {
                    result[i][j] = vec_one[i][j] - vec_two[i][j];
                    j += 1;
                    if j == vec_one[0].len() {
                        break;
                    }
                }
                i += 1;
                if i == vec_one.len() {
                    break;
                }
            }
            result
        }

        pub fn transpose(vec: Vec<Vec<f32>>) -> Vec<Vec<f32>> {
            let mut result: Vec<Vec<f32>> = Math::initialize_zero_matrix(vec[0].len(), vec.len());
            let mut i = 0;
            let mut j = 0;
            loop {
                loop {
                    result[i][j] = vec[j][i];
                    j += 1;
                    if j == vec.len() {
                        break;
                    }
                }
                i += 1;
                if i == vec[0].len() {
                    break;
                }
            }
            result
        }

        pub fn matrix_scalar_multiplication(scalar: f32, vec: Vec<Vec<f32>>) -> Vec<Vec<f32>> {
            let mut result: Vec<Vec<f32>> = Math::initialize_zero_matrix(vec.len(), vec[0].len());
            let mut i = 0;
            let mut j = 0;
            loop {
                loop {
                    result[i][j] = scalar * vec[i][j];
                    j += 1;
                    if j == vec[0].len() {
                        break;
                    }
                }
                i += 1;
                if i == vec.len() {
                    break;
                }
            }
            result
        }

        pub fn matrix_sum(vec: Vec<Vec<f32>>) -> f32 {
            let mut sum = 0.0;
            for v in vec {
                for num in v {
                    sum += num;
                }
            }
            sum
        }

        pub fn matrix_log(vec: Vec<Vec<f32>>) -> Vec<Vec<f32>> {
            let mut result: Vec<Vec<f32>> = Vec::new();
            for v in vec {
                let mut dummy: Vec<f32> = Vec::new();
                for num in v {
                    dummy.push(num.ln());
                }
                result.push(dummy);
            }
            result
        }
    }
}