use orion_numbers::{F64, F64Impl};
use orion_dl::{Tensor, tensor_sin};

fn main(mut self: Tensor<F64>) -> Tensor<F64> {
    tensor_sin(ref self)
}
