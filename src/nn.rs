pub mod nn {
    trait Operations {
        fn new() -> Self;
        fn forward(self, input: Vec<Vec<f32>>) -> Vec<Vec<f32>>;
        fn backward(self, output_grad: Vec<Vec<f32>>) -> Vec<Vec<f32>>;
        fn _output(self) -> Vec<Vec<f32>>;
        fn _input_grad(self, output_grad: Vec<Vec<f32>>) -> Vec<Vec<f32>>;
    }

    trait ParamOperations {
        fn new(param: Vec<Vec<f32>>) -> Self;
        fn forward(self, input: Vec<Vec<f32>>) -> Vec<Vec<f32>>;
        fn backward(self, output_grad: Vec<Vec<f32>>) -> Vec<Vec<f32>>;
        fn _output(self) -> Vec<Vec<f32>>;
        fn _input_grad(self, output_grad: Vec<Vec<f32>>) -> Vec<Vec<f32>>;
        fn _param_grad(self, output_grad: Vec<Vec<f32>>) -> Vec<Vec<f32>>;
    }

    pub struct WeightMultiply {

    }

    impl ParamOperations for WeightMultiply {
        
    }
}