pub(crate) mod binary;
pub(crate) mod reduce;
pub(crate) mod unary;

use crate::{
    cairo_runner::{CairoRunner, CairoRunnerConfig},
    utils::serialization::{
        serialize_inputs_binary_op, serialize_inputs_element_wise, serialize_inputs_reduce_nd,
    },
};

use binary::BinaryOpMetadata;
use luminal::prelude::*;
use reduce::ReduceOpMetadata;
use std::{path::PathBuf, sync::Arc};

#[derive(Debug)]
enum OpCategory {
    Unary(),
    Binary(BinaryOpMetadata),
    Reduce(Option<ReduceOpMetadata>),
}

#[derive(Debug)]
struct CairoOperation {
    sierra_file: PathBuf,
    runner_config: Arc<CairoRunnerConfig>,
    op_category: OpCategory,
}

impl CairoOperation {
    fn new(
        sierra_file: PathBuf,
        runner_config: Arc<CairoRunnerConfig>,
        op_category: OpCategory,
    ) -> Self {
        Self {
            sierra_file,
            runner_config,
            op_category,
        }
    }
}

impl Operator for CairoOperation {
    fn process(&mut self, inp: Vec<(InputTensor, ShapeTracker)>) -> Vec<Tensor> {
        let cairo_runner = CairoRunner::new((*self.runner_config).clone());

        let inputs: Vec<(&Tensor, ShapeTracker)> = inp
            .iter()
            .map(|(input, shape)| (input.borrowed(), *shape))
            .collect();

        let mut returns_dict = false;

        let inputs = match &self.op_category {
            OpCategory::Binary(metadata) => serialize_inputs_binary_op(inputs, metadata),
            OpCategory::Unary() => serialize_inputs_element_wise(inputs),
            OpCategory::Reduce(metadata) => {
                if let Some(meta) = metadata.as_ref() {
                    returns_dict = true;
                    serialize_inputs_reduce_nd(inputs, meta)
                } else {
                    serialize_inputs_element_wise(inputs)
                }
            }
        };

        match cairo_runner.run(self.sierra_file.clone(), inputs, returns_dict) {
            Ok(result) => vec![result],
            Err(e) => {
                panic!("Error executing Cairo: {:?}", e);
            }
        }
    }
}
