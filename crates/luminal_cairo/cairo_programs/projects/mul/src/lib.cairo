use orion_numbers::{F64, F64Impl};
use orion_dl::{Tensor, tensor_mul, BinaryOpMetadata};

fn main(x: Tensor<F64>, y: Tensor<F64>, mut metadata: BinaryOpMetadata) -> Tensor<F64> {
    tensor_mul(x, y, ref metadata)
}
