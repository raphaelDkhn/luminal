use crate::cairo_runner::CairoRunnerConfig;
use crate::constants::COMPILED_CAIRO_PATH;
use crate::ops::*;
use crate::CairoCompilerError;
use luminal::prelude::*;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;

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

            if op.as_any().is::<Add>() {
                let sierra_file = PathBuf::from_str(COMPILED_CAIRO_PATH)
                    .unwrap()
                    .join(format!("{}.json", "add"));
                compile_add(
                    graph,
                    node,
                    &mut ids,
                    sierra_file,
                    Arc::new(self.runner_config.clone()),
                )?;
            } else if op.as_any().is::<Mul>() {
                let sierra_file = PathBuf::from_str(COMPILED_CAIRO_PATH)
                    .unwrap()
                    .join(format!("{}.json", "mul"));
                compile_mul(
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
