use crate::{cairo_runner::CairoRunnerConfig, CairoCompilerError};
use luminal::prelude::*;
use petgraph::visit::EdgeRef;
use std::path::PathBuf;
use std::sync::Arc;

use super::CairoOperation;

pub fn compile_mul(
    graph: &mut Graph,
    node: NodeIndex,
    sierra_file: PathBuf,
    runner_config: Arc<CairoRunnerConfig>,
) -> Result<(), CairoCompilerError> {
    // Get incoming edges
    let inputs: Vec<_> = graph
        .edges_directed(node, petgraph::Direction::Incoming)
        .filter_map(|edge| edge.weight().as_data().map(|data| (edge.source(), data)))
        .collect();

    if inputs.len() != 2 {
        return Err(CairoCompilerError::UnsupportedOperation(
            "Mul operation must have exactly 2 inputs".to_string(),
        ));
    }

    // Create a new node with CairoOperation
    let mut new_op = graph.add_op(CairoOperation::new(
        "mul".to_string(),
        sierra_file,
        runner_config,
    ));

    // Connect the inputs to the new node
    for (input_node, (_, output_order, shape)) in &inputs {
        new_op = new_op.input(*input_node, *output_order, *shape)
    }

    // Finish creating the new node and get its NodeIndex
    let new_node_index = new_op.finish();

    // Collect outgoing edges before modifying the graph
    let outgoing_edges: Vec<_> = graph
        .edges_directed(node, petgraph::Direction::Outgoing)
        .map(|edge| (edge.target(), *edge.weight(), edge.id()))
        .collect();

    // Redirect outgoing edges from the old node to the new node
    for (target, weight, edge_id) in outgoing_edges {
        graph.add_edge(new_node_index, target, weight);
        graph.remove_edge(edge_id);
    }

    // Remove the old node
    graph.remove_node(node);

    Ok(())
}
