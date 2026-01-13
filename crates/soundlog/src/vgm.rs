pub mod command;
mod document;
mod header;
mod parser;

// Re-export commonly used types at the `vgm` module root so callers
// can refer to `soundlog::vgm::VgmHeader` and `soundlog::vgm::VgmBuilder`
// rather than drilling into `vgm::header` or `vgm::model`.
pub use document::{VgmBuilder, VgmDocument};
pub use header::VgmHeader;
