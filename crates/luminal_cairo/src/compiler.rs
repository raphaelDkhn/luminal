use crate::cairo_runner::CairoRunnerConfig;
use crate::constants::COMPILED_CAIRO_PATH;
use crate::ops::unary::compile_unary;
use crate::{ops::binary::compile_binary, CairoCompilerError};
use luminal::prelude::*;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;

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
        for node in graph.node_indices().collect::<Vec<_>>() {
            let op = graph.node_weight(node).unwrap();

            // Binary
            if op.as_any().is::<Add>() {
                let sierra_file = PathBuf::from_str(COMPILED_CAIRO_PATH)
                    .unwrap()
                    .join(format!("{}.sierra.json", "add"));
                compile_binary(
                    graph,
                    node,
                    &mut ids,
                    sierra_file,
                    Arc::new(self.runner_config.clone()),
                )?;
            } else if op.as_any().is::<Mul>() {
                let sierra_file = PathBuf::from_str(COMPILED_CAIRO_PATH)
                    .unwrap()
                    .join(format!("{}.sierra.json", "mul"));
                compile_binary(
                    graph,
                    node,
                    &mut ids,
                    sierra_file,
                    Arc::new(self.runner_config.clone()),
                )?;
            } else if op.as_any().is::<Mod>() {
                let sierra_file = PathBuf::from_str(COMPILED_CAIRO_PATH)
                    .unwrap()
                    .join(format!("{}.sierra.json", "rem"));
                compile_binary(
                    graph,
                    node,
                    &mut ids,
                    sierra_file,
                    Arc::new(self.runner_config.clone()),
                )?;
            } else if op.as_any().is::<LessThan>() {
                let sierra_file = PathBuf::from_str(COMPILED_CAIRO_PATH)
                    .unwrap()
                    .join(format!("{}.sierra.json", "lt"));
                compile_binary(
                    graph,
                    node,
                    &mut ids,
                    sierra_file,
                    Arc::new(self.runner_config.clone()),
                )?;
            }
            // Unary
            else if op.as_any().is::<Log2>() {
                let sierra_file = PathBuf::from_str(COMPILED_CAIRO_PATH)
                    .unwrap()
                    .join(format!("{}.sierra.json", "log2"));
                compile_unary(
                    graph,
                    node,
                    &mut ids,
                    sierra_file,
                    Arc::new(self.runner_config.clone()),
                )?;
            } else if op.as_any().is::<Exp2>() {
                let sierra_file = PathBuf::from_str(COMPILED_CAIRO_PATH)
                    .unwrap()
                    .join(format!("{}.sierra.json", "exp2"));
                compile_unary(
                    graph,
                    node,
                    &mut ids,
                    sierra_file,
                    Arc::new(self.runner_config.clone()),
                )?;
            } else if op.as_any().is::<Sin>() {
                let sierra_file = PathBuf::from_str(COMPILED_CAIRO_PATH)
                    .unwrap()
                    .join(format!("{}.sierra.json", "sin"));
                compile_unary(
                    graph,
                    node,
                    &mut ids,
                    sierra_file,
                    Arc::new(self.runner_config.clone()),
                )?;
            } else if op.as_any().is::<Sqrt>() {
                let sierra_file = PathBuf::from_str(COMPILED_CAIRO_PATH)
                    .unwrap()
                    .join(format!("{}.sierra.json", "sqrt"));
                compile_unary(
                    graph,
                    node,
                    &mut ids,
                    sierra_file,
                    Arc::new(self.runner_config.clone()),
                )?;
            } else if op.as_any().is::<Recip>() {
                let sierra_file = PathBuf::from_str(COMPILED_CAIRO_PATH)
                    .unwrap()
                    .join(format!("{}.sierra.json", "recip"));
                compile_unary(
                    graph,
                    node,
                    &mut ids,
                    sierra_file,
                    Arc::new(self.runner_config.clone()),
                )?;
            }
        }

        Ok(())
    }
}
