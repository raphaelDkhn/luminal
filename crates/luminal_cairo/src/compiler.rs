use crate::cairo_runner::CairoRunnerConfig;
use crate::constants::COMPILED_CAIRO_PATH;
use crate::ops::reduce::compile_reduce;
use crate::ops::unary::compile_unary;
use crate::{ops::binary::compile_binary, CairoCompilerError};
use luminal::prelude::*;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;
use tracing::{debug, info};

#[derive(Default)]
pub struct CairoCompiler {
    runner_config: CairoRunnerConfig,
}

impl CairoCompiler {
    pub fn new(config: CairoRunnerConfig) -> Self {
        Self {
            runner_config: config,
        }
    }
}

impl Compiler for CairoCompiler {
    type Output = Result<(), CairoCompilerError>;

    fn compile<To: ToIdsMut>(&self, graph: &mut Graph, mut ids: To) -> Self::Output {
        info!("Starting CairoCompiler compilation");

        for node in graph.node_indices().collect::<Vec<_>>() {
            let op = graph.node_weight(node).unwrap();
            debug!("Processing node {:?} with operation {:?}", node, op);

            // Handle Tensor Load and Weight Load
            if op.as_any().is::<Function>() {
                debug!("Skipping Tensor/Weight Load operation");
                continue;
            }

            // Handle Constants
            if let Some(constant) = op.as_any().downcast_ref::<Constant>() {
                debug!("Processing Constant operation: {:?}", constant);
                // Convert the constant to a tensor
                let value = match constant.0 {
                    ConstantValue::Float(f) => vec![f],
                    ConstantValue::Expression(ref e) => {
                        vec![e.exec(&graph.dyn_map).unwrap() as f32]
                    }
                };
                let tensor = Tensor::new(value);
                graph.set_tensor(node, 0, tensor);
                continue;
            }

            // Binary ops
            if op.as_any().is::<Add>() {
                debug!("Compiling Add operation");
                let sierra_file = PathBuf::from_str(COMPILED_CAIRO_PATH)
                    .unwrap()
                    .join("add.sierra.json");
                compile_binary(
                    graph,
                    node,
                    &mut ids,
                    sierra_file,
                    Arc::new(self.runner_config.clone()),
                )?;
            } else if op.as_any().is::<Mul>() {
                debug!("Compiling Mul operation");
                let sierra_file = PathBuf::from_str(COMPILED_CAIRO_PATH)
                    .unwrap()
                    .join("mul.sierra.json");
                compile_binary(
                    graph,
                    node,
                    &mut ids,
                    sierra_file,
                    Arc::new(self.runner_config.clone()),
                )?;
            } else if op.as_any().is::<Mod>() {
                debug!("Compiling Mod operation");
                let sierra_file = PathBuf::from_str(COMPILED_CAIRO_PATH)
                    .unwrap()
                    .join("rem.sierra.json");
                compile_binary(
                    graph,
                    node,
                    &mut ids,
                    sierra_file,
                    Arc::new(self.runner_config.clone()),
                )?;
            } else if op.as_any().is::<LessThan>() {
                debug!("Compiling LessThan operation");
                let sierra_file = PathBuf::from_str(COMPILED_CAIRO_PATH)
                    .unwrap()
                    .join("lt.sierra.json");
                compile_binary(
                    graph,
                    node,
                    &mut ids,
                    sierra_file,
                    Arc::new(self.runner_config.clone()),
                )?;
            }
            // Unary ops
            else if op.as_any().is::<Log2>() {
                debug!("Compiling Log2 operation");
                let sierra_file = PathBuf::from_str(COMPILED_CAIRO_PATH)
                    .unwrap()
                    .join("log2.sierra.json");
                compile_unary(
                    graph,
                    node,
                    &mut ids,
                    sierra_file,
                    Arc::new(self.runner_config.clone()),
                )?;
            } else if op.as_any().is::<Exp2>() {
                debug!("Compiling Exp2 operation");
                let sierra_file = PathBuf::from_str(COMPILED_CAIRO_PATH)
                    .unwrap()
                    .join("exp2.sierra.json");
                compile_unary(
                    graph,
                    node,
                    &mut ids,
                    sierra_file,
                    Arc::new(self.runner_config.clone()),
                )?;
            } else if op.as_any().is::<Sin>() {
                debug!("Compiling Sin operation");
                let sierra_file = PathBuf::from_str(COMPILED_CAIRO_PATH)
                    .unwrap()
                    .join("sin.sierra.json");
                compile_unary(
                    graph,
                    node,
                    &mut ids,
                    sierra_file,
                    Arc::new(self.runner_config.clone()),
                )?;
            } else if op.as_any().is::<Sqrt>() {
                debug!("Compiling Sqrt operation");
                let sierra_file = PathBuf::from_str(COMPILED_CAIRO_PATH)
                    .unwrap()
                    .join("sqrt.sierra.json");
                compile_unary(
                    graph,
                    node,
                    &mut ids,
                    sierra_file,
                    Arc::new(self.runner_config.clone()),
                )?;
            } else if op.as_any().is::<Recip>() {
                debug!("Compiling Recip operation");
                let sierra_file = PathBuf::from_str(COMPILED_CAIRO_PATH)
                    .unwrap()
                    .join("recip.sierra.json");
                compile_unary(
                    graph,
                    node,
                    &mut ids,
                    sierra_file,
                    Arc::new(self.runner_config.clone()),
                )?;
            }
            // Reduce ops
            else if op.as_any().is::<SumReduce>() {
                debug!("Compiling SumReduce operation");
                let axis = op
                    .as_any()
                    .downcast_ref::<SumReduce>()
                    .map(|sum_reduce| sum_reduce.0)
                    .unwrap();

                let srcs = graph.get_sources(node);
                if srcs.len() != 1 {
                    return Err(CairoCompilerError::UnsupportedOperation(
                        "SumReduce operation must have exactly 1 input".to_string(),
                    ));
                }

                let shape: Vec<usize> = srcs[0].2.shape_usize();

                let sierra_file = if shape.len() == 1 {
                    PathBuf::from_str(COMPILED_CAIRO_PATH)
                        .unwrap()
                        .join("sum_reduce_1d.sierra.json")
                } else {
                    PathBuf::from_str(COMPILED_CAIRO_PATH)
                        .unwrap()
                        .join("sum_reduce_nd.sierra.json")
                };

                compile_reduce(
                    graph,
                    &srcs,
                    node,
                    &mut ids,
                    sierra_file,
                    Arc::new(self.runner_config.clone()),
                    axis,
                    &shape,
                )?;
            } else if op.as_any().is::<MaxReduce>() {
                debug!("Compiling MaxReduce operation");
                let axis = op
                    .as_any()
                    .downcast_ref::<MaxReduce>()
                    .map(|max_reduce| max_reduce.0)
                    .unwrap();

                let srcs = graph.get_sources(node);
                if srcs.len() != 1 {
                    return Err(CairoCompilerError::UnsupportedOperation(
                        "MaxReduce operation must have exactly 1 input".to_string(),
                    ));
                }

                let shape: Vec<usize> = srcs[0].2.shape_usize();

                let sierra_file = if shape.len() == 1 {
                    PathBuf::from_str(COMPILED_CAIRO_PATH)
                        .unwrap()
                        .join("max_reduce_1d.sierra.json")
                } else {
                    PathBuf::from_str(COMPILED_CAIRO_PATH)
                        .unwrap()
                        .join("max_reduce_nd.sierra.json")
                };

                compile_reduce(
                    graph,
                    &srcs,
                    node,
                    &mut ids,
                    sierra_file,
                    Arc::new(self.runner_config.clone()),
                    axis,
                    &shape,
                )?;
            } else if op.as_any().is::<Contiguous>() {
                debug!("Processing Contiguous operation");
                let srcs = graph.get_sources(node);
                let new_op = graph
                    .add_op(luminal::op::Contiguous)
                    .input(srcs[0].0, srcs[0].1, srcs[0].2)
                    .finish();

                move_outgoing_edge(node, new_op, graph);

                remap(node, new_op, &mut ids, graph);

                graph.remove_node(node);
            } else {
                println!("Unsupported operation: {:?}", op);
            }
        }

        info!("CairoCompiler compilation completed");
        Ok(())
    }
}
