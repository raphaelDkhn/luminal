use crate::cairo_runner::{CairoRunner, CairoRunnerConfig};
use crate::ops::*;
use crate::CairoCompilerError;
use luminal::prelude::*;
use std::path::PathBuf;

pub struct CairoCompiler {
    cairo_runner: CairoRunner,
    sierra_files_path: PathBuf,
}

impl CairoCompiler {
    pub fn new(sierra_files_path: PathBuf, config: CairoRunnerConfig) -> Self {
        Self {
            cairo_runner: CairoRunner::new(config),
            sierra_files_path,
        }
    }

    fn get_sierra_file(&self, op_name: &str) -> PathBuf {
        self.sierra_files_path.join(format!("{}.json", op_name))
    }
}

impl Compiler for CairoCompiler {
    type Output = Result<(), CairoCompilerError>;

    fn compile<To: ToIdsMut>(&self, graph: &mut Graph, _: To) -> Self::Output {
        for node in graph.node_indices().collect::<Vec<_>>() {
            let op = graph.node_weight(node).unwrap();

            if op.as_any().is::<Add>() {
                let sierra_file = self.get_sierra_file("add");
                compile_add(graph, node, &self.cairo_runner, sierra_file)?;
            } else if op.as_any().is::<Mul>() {
                let sierra_file = self.get_sierra_file("mul");
                compile_mul(graph, node, &self.cairo_runner, sierra_file)?;
            }
        }

        Ok(())
    }
}
