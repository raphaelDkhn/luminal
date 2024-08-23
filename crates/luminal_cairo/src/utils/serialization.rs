use cairo1_run::FuncArg;
use cairo_vm::Felt252;
use luminal::prelude::*;
use num_traits::FromPrimitive;

use crate::{
    ops::{binary::BinaryOpMetadata, reduce::ReduceOpMetadata},
    utils::fixed_point::*,
};

pub(crate) fn serialize_inputs_element_wise(inputs: Vec<(&Tensor, ShapeTracker)>) -> Vec<FuncArg> {
    let num_ops = inputs.len();
    let mut serialized: Vec<FuncArg> = Vec::with_capacity(num_ops);

    for (tensor, _) in inputs {
        let data_arg = FuncArg::Array(tensor.downcast_ref::<Vec<f32>>().map_or_else(
            || vec![],
            |data| {
                data.iter()
                    .map(|&ele| Felt252::from_i64(from_float_to_fp(ele)).unwrap())
                    .collect()
            },
        ));

        serialized.push(data_arg);
    }

    serialized
}

pub(crate) fn serialize_inputs_binary_op(
    inputs: Vec<(&Tensor, ShapeTracker)>,
    metadata: &BinaryOpMetadata,
) -> Vec<FuncArg> {
    let num_ops = inputs.len() + 2; // Include space for lhs_indices and rhs_indices
    let mut serialized: Vec<FuncArg> = Vec::with_capacity(num_ops);

    for (tensor, _) in inputs {
        let data_arg = FuncArg::Array(tensor.downcast_ref::<Vec<f32>>().map_or_else(
            || vec![],
            |data| {
                data.iter()
                    .map(|&ele| Felt252::from_i64(from_float_to_fp(ele)).unwrap())
                    .collect()
            },
        ));

        serialized.push(data_arg);
    }

    let lhs_indices = FuncArg::Array(
        metadata
            .lhs_indices
            .iter()
            .map(|&ele| Felt252::from_i64(ele as i64).unwrap())
            .collect(),
    );

    let rhs_indices = FuncArg::Array(
        metadata
            .rhs_indices
            .iter()
            .map(|&ele| Felt252::from_i64(ele as i64).unwrap())
            .collect(),
    );

    serialized.push(lhs_indices);
    serialized.push(rhs_indices);

    serialized
}

pub(crate) fn serialize_inputs_reduce_nd(
    inputs: Vec<(&Tensor, ShapeTracker)>,
    metadata: &ReduceOpMetadata,
) -> Vec<FuncArg> {
    let num_ops: usize = inputs.len() + 2; // Include space for output_indices and output_size
    let mut serialized: Vec<FuncArg> = Vec::with_capacity(num_ops);

    for (tensor, _) in inputs {
        let data_arg = FuncArg::Array(tensor.downcast_ref::<Vec<f32>>().map_or_else(
            || vec![],
            |data| {
                data.iter()
                    .map(|&ele| Felt252::from_i64(from_float_to_fp(ele)).unwrap())
                    .collect()
            },
        ));

        serialized.push(data_arg);
    }

    let output_indices = FuncArg::Array(
        metadata
            .output_indices
            .iter()
            .map(|&ele| Felt252::from_i64(ele as i64).unwrap())
            .collect(),
    );

    let output_size = FuncArg::Single(Felt252::from_i64(metadata.output_size as i64).unwrap());

    serialized.push(output_indices);
    serialized.push(output_size);

    serialized
}
