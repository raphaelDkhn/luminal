use luminal::prelude::*;
use std::path::PathBuf;
use std::sync::Arc;

use super::{CairoOperation, OpCategory};
use crate::{
    cairo_runner::CairoRunnerConfig, precomputing::reduce::precompute_reduce_op_metadata,
    CairoCompilerError,
};

#[derive(Debug)]
pub(crate) struct ReduceOpMetadata {
    pub(crate) output_indices: Vec<usize>,
    pub(crate) output_size: usize,
}

pub(crate) fn compile_reduce<To: ToIdsMut>(
    graph: &mut Graph,
    node: NodeIndex,
    ids: &mut To,
    sierra_file: PathBuf,
    runner_config: Arc<CairoRunnerConfig>,
) -> Result<(), CairoCompilerError> {
    // Get sources (inputs) of the reduce operation
    let srcs = graph.get_sources(node);
    if srcs.len() != 1 {
        return Err(CairoCompilerError::UnsupportedOperation(
            "Unary operation must have exactly 1 input".to_string(),
        ));
    }

    let shape = srcs[0].2.shape_usize();
    let axis = 1;

    let metadata: Option<ReduceOpMetadata> = if shape.len() == 1 {
        None
    } else {
        Some(precompute_reduce_op_metadata(&shape, axis))
    };

    // Create a new node with CairoOperation
    let new_op = graph
        .add_op(CairoOperation::new(
            sierra_file,
            runner_config,
            OpCategory::Reduce(metadata),
        ))
        .input(srcs[0].0, srcs[0].1, srcs[0].2)
        .finish();

    // Move outgoing edges from the old node to the new node
    move_outgoing_edge(node, new_op, graph);

    // Remap any references to the old node to the new node
    remap(node, new_op, ids, graph);

    // Remove the old node
    graph.remove_node(node);

    Ok(())
}
