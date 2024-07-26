use luminal::prelude::*;

mod compiler;
mod ops;
mod sierra_runner;

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
}
