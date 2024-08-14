use orion_numbers::{F64, F64Impl};
use orion_dl::{tensor::{Tensor, tensor_add, BinaryOpMetadata}};

fn main(x: Tensor<F64>, y: Tensor<F64>, mut metadata: BinaryOpMetadata) -> Tensor<F64> {
    tensor_add(x, y, ref metadata)
}
