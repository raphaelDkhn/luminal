use cairo_vm::air_public_input::PublicInputError;
use luminal::prelude::*;

mod constants;
mod compiler;
mod ops;
mod cairo_runner;
mod fixed_point;

#[cfg(test)]
mod tests;

#[derive(thiserror::Error, Debug)]
pub enum CairoCompilerError {
    #[error("Failed to load Sierra file: {0}")]
    SierraLoadError(String),
    #[error("Failed to run Sierra program: {0}")]
    SierraRunError(String),
    #[error("Unsupported operation: {0}")]
    UnsupportedOperation(String),
    #[error("Missing tensor for node: {0:?}")]
    MissingTensor(NodeIndex),
    #[error("Runtime error: {0}")]
    RuntimeError(String),
    #[error("Public input error: {0}")]
    PublicInputError(String),
    #[error("IO error: {0}")]
    IoError(String),
    #[error("VM runner error: {0}")]
    VmRunnerError(String),
    #[error("Trace encoding error: {0}")]
    TraceEncodingError(String),
    #[error("Cairo output deserialization error: {0}")]
    DeserializationError(String),

}

impl From<cairo1_run::error::Error> for CairoCompilerError {
    fn from(err: cairo1_run::error::Error) -> Self {
        CairoCompilerError::RuntimeError(err.to_string())
    }
}

impl From<PublicInputError> for CairoCompilerError {
    fn from(err: PublicInputError) -> Self {
        CairoCompilerError::PublicInputError(err.to_string())
    }
}

impl From<std::io::Error> for CairoCompilerError {
    fn from(err: std::io::Error) -> Self {
        CairoCompilerError::IoError(err.to_string())
    }
}

impl From<cairo_vm::vm::errors::runner_errors::RunnerError> for CairoCompilerError {
    fn from(err: cairo_vm::vm::errors::runner_errors::RunnerError) -> Self {
        CairoCompilerError::VmRunnerError(err.to_string())
    }
}

impl From<cairo_vm::cairo_run::EncodeTraceError> for CairoCompilerError {
    fn from(err: cairo_vm::cairo_run::EncodeTraceError) -> Self {
        CairoCompilerError::TraceEncodingError(err.to_string())
    }
}
