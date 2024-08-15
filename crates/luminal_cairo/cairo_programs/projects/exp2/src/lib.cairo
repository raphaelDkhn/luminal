use orion_numbers::{F64, F64Impl};
use orion_dl::{Tensor, tensor_exp2};

fn main(mut self: Tensor<F64>) -> Tensor<F64> {
    tensor_exp2(ref self)
}
