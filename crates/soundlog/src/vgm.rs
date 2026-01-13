pub mod command;
<<<<<<< HEAD
mod header;
mod model;
=======
mod document;
mod header;
>>>>>>> feature-refvgm
mod parser;

// Re-export commonly used types at the `vgm` module root so callers
// can refer to `soundlog::vgm::VgmHeader` and `soundlog::vgm::VgmBuilder`
// rather than drilling into `vgm::header` or `vgm::model`.
<<<<<<< HEAD
pub use header::VgmHeader;
pub use model::{VgmBuilder, VgmDocument};
=======
pub use document::{VgmBuilder, VgmDocument};
pub use header::VgmHeader;
>>>>>>> feature-refvgm
