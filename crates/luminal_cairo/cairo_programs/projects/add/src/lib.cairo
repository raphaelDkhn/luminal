use orion_numbers::{F64, F64Impl};
use orion_dl::Tensor;

fn main(x: Tensor<F64>, y: Tensor<F64>) -> Tensor<F64> {
    x + y
}
