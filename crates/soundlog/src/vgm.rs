pub mod command;
mod document;
mod header;
mod parser;

pub use document::{VgmBuilder, VgmDocument};
pub use header::{VgmExtraHeader, VgmHeader};
