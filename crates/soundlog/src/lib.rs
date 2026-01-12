mod binutil;
pub mod chip;
pub mod meta;
mod tests;
pub mod vgm;
mod xgm;

pub use binutil::ParseError;
pub use vgm::VgmHeader;
pub use vgm::command::{
    Ay8910StereoMask, ChipId, DataBlock, EndOfData, PcmRamWrite, SeekOffset, SetStreamFrequency,
    SetupStreamControl, StartStream, StartStreamFastCall, StopStream, VgmCommand, Wait735Samples,
    Wait882Samples, WaitNSample, WaitSamples, Ym2612Port0Address2AWriteAndWaitN,
};
pub use vgm::{VgmBuilder, VgmDocument};
