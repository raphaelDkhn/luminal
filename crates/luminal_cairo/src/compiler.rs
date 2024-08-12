use crate::cairo_runner::CairoRunnerConfig;
use crate::ops::*;
use crate::CairoCompilerError;
use luminal::prelude::*;
use std::path::PathBuf;
use std::sync::Arc;

pub struct CairoCompiler {
    sierra_files_path: PathBuf,
    runner_config: CairoRunnerConfig,
}

impl CairoCompiler {
    pub fn new(sierra_files_path: PathBuf, config: CairoRunnerConfig) -> Self {
        Self {
            sierra_files_path,
            runner_config: config,
        }
    }

    fn get_sierra_file(&self, op_name: &str) -> PathBuf {
        self.sierra_files_path.join(format!("{}.json", op_name))
    }
}

impl Compiler for CairoCompiler {
    type Output = Result<(), CairoCompilerError>;

    fn compile<To: ToIdsMut>(&self, graph: &mut Graph, mut ids: To) -> Self::Output {
        for node in graph.node_indices().collect::<Vec<_>>() {
            let op = graph.node_weight(node).unwrap();

            if op.as_any().is::<Add>() {
                let sierra_file = self.get_sierra_file("add");
                compile_add(
                    graph,
                    node,
                    &mut ids,
                    sierra_file,
                    Arc::new(self.runner_config.clone()),
                )?;
            } else if op.as_any().is::<Mul>() {
                let sierra_file = self.get_sierra_file("mul");
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
