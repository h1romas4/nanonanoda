pub mod command;
mod header;
mod model;
mod parser;

// Re-export commonly used types at the `vgm` module root so callers
// can refer to `soundlog::vgm::VgmHeader` and `soundlog::vgm::VgmBuilder`
// rather than drilling into `vgm::header` or `vgm::model`.
pub use header::VgmHeader;
pub use model::{VgmBuilder, VgmDocument};
