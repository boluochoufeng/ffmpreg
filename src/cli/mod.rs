pub mod args;
pub mod pipeline;

pub use args::Args;
pub use pipeline::{BatchPipeline, Pipeline, is_batch_pattern, is_directory};
