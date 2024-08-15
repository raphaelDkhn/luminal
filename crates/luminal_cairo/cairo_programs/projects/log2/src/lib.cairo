use orion_numbers::{F64, F64Impl};
use orion_dl::{Tensor, tensor_log2};

fn main(mut self: Tensor<F64>) -> Tensor<F64> {
    tensor_log2(ref self)
}
