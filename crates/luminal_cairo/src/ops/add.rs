use crate::{cairo_runner::CairoRunner, CairoCompilerError};
use luminal::prelude::*;
use petgraph::visit::EdgeRef;
use std::path::PathBuf;

pub fn compile_add(
    graph: &mut Graph,
    node: NodeIndex,
    cairo_runner: &CairoRunner,
    sierra_file: PathBuf,
) -> Result<(), CairoCompilerError> {
    // Get incoming edges
    let inputs: Vec<_> = graph
        .edges_directed(node, petgraph::Direction::Incoming)
        .filter_map(|edge| edge.weight().as_data().map(|data| (edge.source(), data)))
        .collect();

    if inputs.len() != 2 {
        return Err(CairoCompilerError::UnsupportedOperation(
            "Add operation must have exactly 2 inputs".to_string(),
        ));
    }
    
    let a: &Tensor = graph
        .get_tensor_ref(inputs[0].0, inputs[0].1 .1)
        .ok_or_else(|| CairoCompilerError::MissingTensor(inputs[0].0))?;
    let a_shape = inputs[0].1.2;

    let b: &Tensor = graph
        .get_tensor_ref(inputs[1].0, inputs[1].1 .1)
        .ok_or_else(|| CairoCompilerError::MissingTensor(inputs[1].0))?;
    let b_shape = inputs[1].1.2;

    let result = cairo_runner.run(sierra_file, vec![(a, a_shape), (b, b_shape)])?;

    // Create a new node with the result
    let mut new_op = graph.add_op(CairoOperation::new(result));

    // Connect the inputs to the new node
    for (input_node, (_, output_order, shape)) in &inputs {
        new_op = new_op.input(*input_node, *output_order, *shape);
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

#[derive(Debug)]
struct CairoOperation {
    result: Tensor,
}

impl CairoOperation {
    fn new(result: Tensor) -> Self {
        Self { result }
    }
}

impl Operator for CairoOperation {
    fn process(&mut self, _inp: Vec<(InputTensor, ShapeTracker)>) -> Vec<Tensor> {
        vec![self.result.clone()]
    }
}
