use orion_numbers::{F64, F64Impl};
use orion_dl::{Tensor, tensor_max_reduce_1d};

fn main(mut self: Tensor<F64>) -> Tensor<F64> {
    tensor_max_reduce_1d(self)
}
