use std::{
    fs,
    io::{self, Write},
    path::PathBuf,
};

use bincode::enc::write::Writer;
use cairo1_run::{cairo_run_program, error::Error, Cairo1RunConfig, FuncArg};
use cairo_lang_sierra::program::{Program, VersionedProgram};
use cairo_vm::{
    air_public_input::PublicInputError,
    types::layout_name::LayoutName,
    vm::runners::cairo_runner::CairoRunner as CairoVMRunner,
    vm::{errors::trace_errors::TraceError, vm_core::VirtualMachine},
};
use luminal::prelude::*;
use tracing::info;

use crate::CairoCompilerError;

struct FileWriter {
    buf_writer: io::BufWriter<std::fs::File>,
    bytes_written: usize,
}

impl Writer for FileWriter {
    fn write(&mut self, bytes: &[u8]) -> Result<(), bincode::error::EncodeError> {
        self.buf_writer
            .write_all(bytes)
            .map_err(|e| bincode::error::EncodeError::Io {
                inner: e,
                index: self.bytes_written,
            })?;

        self.bytes_written += bytes.len();

        Ok(())
    }
}

impl FileWriter {
    fn new(buf_writer: io::BufWriter<std::fs::File>) -> Self {
        Self {
            buf_writer,
            bytes_written: 0,
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        self.buf_writer.flush()
    }
}

pub struct CairoRunnerConfig {
    pub trace_file: Option<PathBuf>,
    pub memory_file: Option<PathBuf>,
    pub layout: String,
    pub proof_mode: bool,
    pub air_public_input: Option<PathBuf>,
    pub air_private_input: Option<PathBuf>,
    pub cairo_pie_output: Option<PathBuf>,
    pub append_return_values: bool,
}

pub struct CairoRunner {
    config: CairoRunnerConfig,
}

impl CairoRunner {
    pub fn new(config: CairoRunnerConfig) -> Self {
        Self { config }
    }

    pub fn run(
        &self,
        sierra_file: PathBuf,
        inputs: Vec<Tensor>,
    ) -> Result<Tensor, CairoCompilerError> {
        // load program
        let program = self.load_sierra_file(sierra_file)?;

        // Set up cairo runner config.
        let config = &self.config;
        let cairo_run_config = Cairo1RunConfig {
            args: todo!(),
            serialize_output: true,
            trace_enabled: config.trace_file.is_some() || config.air_public_input.is_some(),
            relocate_mem: config.memory_file.is_some() || config.air_public_input.is_some(),
            layout: LayoutName::all_cairo,
            proof_mode: config.proof_mode,
            finalize_builtins: config.air_private_input.is_some()
                || config.cairo_pie_output.is_some(),
            append_return_values: config.append_return_values,
        };

        // Run the program
        let (runner, _, serialized_output) = cairo_run_program(&program, cairo_run_config)?;

        // Generate output files (trace, memory, cairopie files)
        self.generate_output_files(&runner)?;

        todo!()
    }

    fn load_sierra_file(&self, file_path: PathBuf) -> Result<Program, CairoCompilerError> {
        let content = fs::read(&file_path).map_err(|e| {
            CairoCompilerError::SierraLoadError(format!("Failed to read file: {:?}", e))
        })?;

        let versioned_program =
            serde_json::from_slice::<VersionedProgram>(&content).map_err(|e| {
                CairoCompilerError::SierraLoadError(format!("Failed to deserialize file: {:?}", e))
            })?;

        let program = versioned_program
            .into_v1()
            .map_err(|_| CairoCompilerError::SierraLoadError("Version conversion failed".into()))?
            .program;

        let file_name = file_path
            .file_name()
            .ok_or_else(|| CairoCompilerError::SierraLoadError("Failed to get file name".into()))?
            .to_str()
            .ok_or_else(|| {
                CairoCompilerError::SierraLoadError("Failed to convert file name to string".into())
            })?
            .to_string();

        info!("📄 Loaded program: {}", file_name);

        Ok(program)
    }

    fn generate_output_files(&self, runner: &CairoVMRunner) -> Result<(), CairoCompilerError> {
        let config = &self.config;

        if let (Some(file_path), Some(trace_file), Some(memory_file)) = (
            config.air_private_input.clone(),
            config.trace_file.clone(),
            config.memory_file.clone(),
        ) {
            // Get absolute paths of trace_file & memory_file
            let trace_path = trace_file
                .as_path()
                .canonicalize()
                .unwrap_or(trace_file.clone())
                .to_string_lossy()
                .to_string();
            let memory_path = memory_file
                .as_path()
                .canonicalize()
                .unwrap_or(memory_file.clone())
                .to_string_lossy()
                .to_string();

            let json = runner
                .get_air_private_input()
                .to_serializable(trace_path, memory_path)
                .serialize_json()
                .map_err(PublicInputError::Serde)?;
            std::fs::write(file_path, json)?;
        }

        if let Some(ref file_path) = config.cairo_pie_output {
            runner.get_cairo_pie()?.write_zip_file(file_path)?
        }

        if let Some(trace_path) = &config.trace_file {
            let relocated_trace = runner
                .relocated_trace
                .clone()
                .ok_or(Error::Trace(TraceError::TraceNotRelocated))?;
            let trace_file = std::fs::File::create(trace_path)?;
            let mut trace_writer =
                FileWriter::new(io::BufWriter::with_capacity(3 * 1024 * 1024, trace_file));

            cairo_vm::cairo_run::write_encoded_trace(&relocated_trace, &mut trace_writer)?;
            trace_writer.flush()?;
        }
        if let Some(memory_path) = &config.memory_file {
            let memory_file = std::fs::File::create(memory_path)?;
            let mut memory_writer =
                FileWriter::new(io::BufWriter::with_capacity(5 * 1024 * 1024, memory_file));

            cairo_vm::cairo_run::write_encoded_memory(
                &runner.relocated_memory,
                &mut memory_writer,
            )?;
            memory_writer.flush()?;
        }

        Ok(())
    }
}
