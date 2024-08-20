use luminal::prelude::*;
use std::path::PathBuf;
use std::sync::Arc;

use super::{CairoOperation, OpCategory};
use crate::precomputing::binary::precompute_binary_op_metadata;
use crate::{cairo_runner::CairoRunnerConfig, CairoCompilerError};

#[derive(Debug)]
pub(crate) struct BinaryOpMetadata {
    pub(crate) lhs_indices: Vec<usize>,
    pub(crate) rhs_indices: Vec<usize>,
}

pub(crate) fn compile_binary<To: ToIdsMut>(
    graph: &mut Graph,
    node: NodeIndex,
    ids: &mut To,
    sierra_file: PathBuf,
    runner_config: Arc<CairoRunnerConfig>,
) -> Result<(), CairoCompilerError> {
    // Get sources (inputs) of the add operation
    let srcs = graph.get_sources(node);
    if srcs.len() != 2 {
        return Err(CairoCompilerError::UnsupportedOperation(
            "Binary operation must have exactly 2 inputs".to_string(),
        ));
    }

    // Precompute indices to avoid computing it in Cairo run-time
    let (lhs_indices, rhs_indices) =
        precompute_binary_op_metadata(&srcs[0].2.shape_usize(), &&srcs[1].2.shape_usize());

    // Create a new node with CairoOperation
    let new_op = graph
        .add_op(CairoOperation::new(
            sierra_file,
            runner_config,
            OpCategory::Binary(BinaryOpMetadata {
                lhs_indices,
                rhs_indices,
            }),
        ))
        .input(srcs[0].0, srcs[0].1, srcs[0].2)
        .input(srcs[1].0, srcs[1].1, srcs[1].2)
        .finish();

    // Move outgoing edges from the old node to the new node
    move_outgoing_edge(node, new_op, graph);

    // Remap any references to the old node to the new node
    remap(node, new_op, ids, graph);

    // Remove the old node
    graph.remove_node(node);

    Ok(())
}
