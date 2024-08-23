use core::dict::Felt252Dict;
use orion_numbers::{F64, F64Impl};
use orion_dl::{Tensor, MutTensor, tensor_sum_reduce_nd, ReduceOpMetadata};

fn main(mut self: Tensor<F64>, mut metadata: ReduceOpMetadata) -> MutTensor<F64> {
    tensor_sum_reduce_nd(self, ref metadata)
}
