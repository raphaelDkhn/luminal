use std::path::PathBuf;

use cairo_lang_sierra::program::Program;
use cairo_vm::vm::{runners::cairo_runner::CairoRunner, vm_core::VirtualMachine};
use luminal::prelude::*;

use crate::CairoCompilerError;

pub struct SierraRunner {
    vm: VirtualMachine,
}

impl SierraRunner {
    pub fn new() -> Self {
        Self {
            vm: VirtualMachine::new(true),
        }
    }

    pub fn run_sierra(
        &self,
        sierra_file: PathBuf,
        inputs: Vec<Tensor>,
    ) -> Result<Tensor, CairoCompilerError> {
        todo!()
    }

    fn load_sierra_file(&self, file_path: PathBuf) -> Result<Program, CairoCompilerError> {
        todo!()
    }
}
