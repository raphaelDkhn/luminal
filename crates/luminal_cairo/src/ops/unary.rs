use luminal::prelude::*;
use std::path::PathBuf;
use std::sync::Arc;

use super::{CairoOperation, OpCategory};
use crate::{cairo_runner::CairoRunnerConfig, CairoCompilerError};

pub(crate) fn compile_unary<To: ToIdsMut>(
    graph: &mut Graph,
    node: NodeIndex,
    ids: &mut To,
    sierra_file: PathBuf,
    runner_config: Arc<CairoRunnerConfig>,
) -> Result<(), CairoCompilerError> {
    // Get sources (inputs) of the add operation
    let srcs = graph.get_sources(node);
    if srcs.len() != 1 {
        return Err(CairoCompilerError::UnsupportedOperation(
            "Unary operation must have exactly 1 input".to_string(),
        ));
    }

    // Create a new node with CairoOperation
    let new_op = graph
        .add_op(CairoOperation::new(
            sierra_file,
            runner_config,
            OpCategory::Unary(),
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
