mod add;
mod mul;

pub use add::compile_add;
pub use mul::compile_mul;

use crate::{
    cairo_runner::{CairoRunner, CairoRunnerConfig},
    utils::serialization::serialize_binary_op_inputs,
};
use luminal::prelude::*;
use std::{path::PathBuf, sync::Arc};

#[derive(Debug)]
pub(crate) struct BinaryOpMetadata {
    pub(crate) lhs_indices: Vec<usize>,
    pub(crate) rhs_indices: Vec<usize>,
}

#[derive(Debug)]
enum OpCategory {
    Binary(BinaryOpMetadata),
}

#[derive(Debug)]
struct CairoOperation {
    op_name: String,
    sierra_file: PathBuf,
    runner_config: Arc<CairoRunnerConfig>,
    op_category: OpCategory,
}

impl CairoOperation {
    fn new(
        op_name: String,
        sierra_file: PathBuf,
        runner_config: Arc<CairoRunnerConfig>,
        op_category: OpCategory,
    ) -> Self {
        Self {
            op_name,
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

        let inputs = match &self.op_category {
            OpCategory::Binary(metadata) => serialize_binary_op_inputs(inputs, metadata),
        };

        match cairo_runner.run(self.sierra_file.clone(), inputs) {
            Ok(result) => vec![result],
            Err(e) => {
                panic!("Error executing Cairo operation {}: {:?}", self.op_name, e);
            }
        }
    }
}
