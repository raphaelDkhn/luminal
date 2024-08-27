pub(crate) mod binary;
pub(crate) mod reduce;
pub(crate) mod unary;

use tracing::{debug, error};

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
        debug!("Processing CairoOperation with {} inputs", inp.len());
        for (i, (input, shape)) in inp.iter().enumerate() {
            debug!("Input {}: Shape = {:?}", i, shape);
            if let InputTensor::Owned(t) = input {
                debug!("Input {} data: {:?}", i, t.data);
            }
        }

        let tensor_inputs: Vec<(&Tensor, ShapeTracker)> = inp
            .iter()
            .map(|(input, shape)| {
                let tensor = match input {
                    InputTensor::Owned(t) => t,
                    InputTensor::Borrowed(t) => t,
                };
                (tensor, *shape)
            })
            .collect();

        let cairo_runner = CairoRunner::new((*self.runner_config).clone());

        let mut returns_dict = false;

        let inputs = match &self.op_category {
            OpCategory::Binary(metadata) => serialize_inputs_binary_op(tensor_inputs, metadata),
            OpCategory::Unary() => serialize_inputs_element_wise(tensor_inputs),
            OpCategory::Reduce(metadata) => {
                if let Some(meta) = metadata.as_ref() {
                    returns_dict = true;
                    serialize_inputs_reduce_nd(tensor_inputs, meta)
                } else {
                    serialize_inputs_element_wise(tensor_inputs)
                }
            }
        };

        debug!("Serialized inputs: {:?}", inputs);

        match cairo_runner.run(self.sierra_file.clone(), inputs, returns_dict) {
            Ok(result) => {
                debug!("Cairo execution result: {:?}", result);
                vec![result]
            }
            Err(e) => {
                error!("Error executing Cairo: {:?}", e);
                panic!("Error executing Cairo: {:?}", e);
            }
        }
    }
}
