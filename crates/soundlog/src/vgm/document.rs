use crate::chip;
use crate::meta::Gd3;
use crate::vgm::command::Instance;
use crate::vgm::command::VgmCommand;
use crate::vgm::header::{VgmExtraHeader, VgmHeader};
use crate::vgm::parser;
use std::convert::TryFrom;

#[derive(Debug, Clone, PartialEq, Default)]
/// A complete VGM document, consisting of a header, an ordered command
/// stream, and optional GD3 metadata and an optional extra header.
///
/// Construct `VgmDocument` instances using `VgmBuilder`. Once assembled,
/// call `VgmDocument::to_bytes()` to obtain the serialized VGM file
/// bytes suitable for writing to disk.
pub struct VgmDocument {
    pub header: VgmHeader,
    pub commands: Vec<VgmCommand>,
    pub gd3: Option<Gd3>,
    pub extra_header: Option<VgmExtraHeader>,
}

/// Builder for assembling a `VgmDocument`.
///
/// Use this builder to incrementally set header fields, register chip
/// clock frequencies, append commands, and specify a loop point. Methods
/// return `&mut Self` when appropriate to allow chaining. Call
/// `finalize()` to compute derived header fields (for example
/// `total_samples` and `loop_offset`) and obtain the completed
/// `VgmDocument`.
pub struct VgmBuilder {
    document: VgmDocument,
    loop_index: Option<usize>,
}

/// Implementation of `VgmBuilder` methods.
///
/// This `impl` block provides constructors and fluent APIs for building
/// `VgmDocument` instances: adding commands, registering chips, and finalizing
/// the assembled document for serialization.
impl VgmBuilder {
    /// Create a new `VgmBuilder` with a default, empty `VgmDocument`.
    ///
    /// The returned builder is ready to have header fields and commands
    /// appended via the other builder methods.
    pub fn new() -> Self {
        VgmBuilder {
            document: VgmDocument::default(),
            loop_index: None,
        }
    }

    /// Register a chip in the VGM header with its master clock frequency.
    ///
    /// `c` is convertible to `chip::Chip`. `chip_id` selects which instance
    /// (primary/secondary) the clock applies to. `master_clock` is the chip's
    /// base clock frequency in Hz. For secondary instances the high bit is set
    /// on the stored clock field as per the VGM header convention.
    pub fn register_chip<C, I>(&mut self, c: C, chip_id: I, master_clock: u32)
    where
        C: Into<chip::Chip>,
        I: Into<Instance>,
    {
        let ch: chip::Chip = c.into();
        let instance: Instance = chip_id.into();

        self.document
            .header
            .set_chip_clock(ch, instance, master_clock);
    }

    /// Set the loop point by command index.
    ///
    /// `idx` is an index into `doc.commands` indicating the command at which
    /// playback should loop. The header's `loop_offset` will be computed in
    /// `finalize()` as a relative offset from 0x1C.
    pub fn set_loop_offset(&mut self, document_index: usize) -> &mut Self {
        self.loop_index = Some(document_index);
        self
    }

    /// Append a VGM command to the builder.
    ///
    /// Accepts any type convertible into `VgmCommand` (via `Into`).
    /// Returns `&mut Self` to allow method chaining.
    pub fn add_vgm_command<C>(&mut self, command: C) -> &mut Self
    where
        C: Into<VgmCommand>,
    {
        self.document.commands.push(command.into());
        self
    }

    /// Append a chip write produced by a chip-specific spec.
    ///
    /// `chip_id` selects the chip instance (`ChipId::Primary` or `ChipId::Secondary`).
    /// `c` must implement `ChipWriteSpec`; the spec will push the appropriate
    /// `VgmCommand` into the builder's command stream. Returns `&mut Self`.
    pub fn add_chip_write<C, I>(&mut self, chip_id: I, spec: C) -> &mut Self
    where
        I: Into<Instance>,
        (Instance, C): Into<VgmCommand>,
    {
        self.document.commands.push((chip_id.into(), spec).into());
        self
    }

    /// Set GD3 metadata for the document under construction.
    ///
    /// This stores the provided `Gd3` into the builder's internal
    /// `VgmDocument` so it will be present on the finalized document.
    pub fn set_gd3(&mut self, gd3: Gd3) -> &mut Self {
        self.document.gd3 = Some(gd3);
        self
    }

    /// Set the extra-header for the document under construction.
    ///
    /// This stores the provided `VgmExtraHeader` into the builder's internal
    /// `VgmDocument` so it will be included when the document is serialized.
    pub fn set_extra_header(&mut self, extra: VgmExtraHeader) -> &mut Self {
        self.document.extra_header = Some(extra);
        self
    }

    /// Finalize the builder and return the assembled `VgmDocument`.
    ///
    /// This computes derived header fields (for example `total_samples` and
    /// `loop_offset`) by scanning accumulated commands. If a loop index has
    /// been set via `set_loop_offset()`, the corresponding command's byte
    /// offset is computed and stored (relative to 0x1C) in the header. The
    /// method returns the complete document ready for serialization via
    /// `VgmDocument::to_bytes()`.
    pub fn finalize(mut self) -> VgmDocument {
        // compute total samples
        let total_sample = self.document.total_samples();
        self.document.header.total_samples = total_sample;

        // compute data_offset the same way as VgmDocument::to_bytes
        let data_offset: u32 = match self.document.header.data_offset {
            0 => crate::vgm::header::VGM_V171_HEADER_SIZE.wrapping_sub(0x34),
            v => v,
        };

        // If an extra_header has been attached, and the header does not
        // already contain a stored extra_header_offset, set it so that the
        // extra header will be placed immediately after the main header in
        // the serialized output. The offset is stored relative to 0xBC.
        //
        // Additionally, compute the serialized length of the extra header and
        // update the header's `data_offset` so the on-disk header correctly
        // reflects the final header size that includes the extra header.
        if self.document.extra_header.is_some() && self.document.header.extra_header_offset == 0 {
            // compute the header length as it will be before inserting the extra header
            let header_len = self.document.header.to_bytes(0, data_offset).len() as u32;
            let extra_offset = header_len.wrapping_sub(0xBC_u32);
            self.document.header.extra_header_offset = extra_offset;

            // If we have the extra header available, compute its serialized
            // size and update the header.data_offset so the final header size
            // (0x34 + data_offset) will cover the inserted extra header.
            if let Some(eh) = &self.document.extra_header {
                let extra_bytes_len = eh.to_bytes().len() as u32;
                let new_header_len = header_len.wrapping_add(extra_bytes_len);
                let new_data_offset = new_header_len.wrapping_sub(0x34_u32);
                self.document.header.data_offset = new_data_offset;
            }
        }

        // If loop index is set, compute byte offset to that command and
        // set header.loop_offset = absolute_offset - 0x1C per VGM spec.
        if let Some(index) = self.loop_index
            && index < self.document.commands.len()
        {
            // header length depends on data_offset and may include an extra-header now
            let header_len = self.document.header.to_bytes(0, data_offset).len() as u32;

            // Use command_offsets_and_lengths to obtain the offset of the command
            // at `index` without serializing the full command stream.
            let offsets = self.document.command_offsets_and_lengths();
            if index < offsets.len() {
                let (cmd_offset, _cmd_len) = offsets[index];
                let loop_abs = header_len.wrapping_add(cmd_offset as u32);
                // store relative offset from 0x1C per spec
                self.document.header.loop_offset = loop_abs.wrapping_sub(0x1C);
            }
        }

        self.document
    }
}

/// Conversion from `VgmDocument` to `VgmBuilder`.
impl From<VgmDocument> for VgmBuilder {
    fn from(document: VgmDocument) -> Self {
        VgmBuilder {
            document,
            loop_index: None,
        }
    }
}

/// Default implementation for `VgmBuilder`.
impl Default for VgmBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl VgmDocument {
    /// Compute the total samples represented by the command stream.
    ///
    /// Returns the summed sample counts from wait and special wait-like
    /// commands; does not modify the document.
    pub fn total_samples(&self) -> u32 {
        self.commands
            .iter()
            .map(|cmd| match cmd {
                VgmCommand::WaitSamples(s) => s.0 as u32,
                VgmCommand::Wait735Samples(_) => 735,
                VgmCommand::Wait882Samples(_) => 882,
                VgmCommand::WaitNSample(s) => s.0 as u32,
                VgmCommand::YM2612Port0Address2AWriteAndWaitN(s) => s.0 as u32,
                _ => 0,
            })
            .sum::<u32>()
    }
}

/// Attempt to convert a raw VGM byte slice into a `VgmDocument`.
///
/// This is a fallible conversion that delegates to `parser::parse_vgm` and
/// returns a `crate::binutil::ParseError` on failure.
///
/// Use `VgmDocument::try_from(bytes)` or `parser::parse_vgm(bytes)` when
/// you need to handle parse errors explicitly.
impl TryFrom<&[u8]> for VgmDocument {
    type Error = crate::binutil::ParseError;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        parser::parse_vgm(bytes)
    }
}

/// Convert a `VgmDocument` into its serialized VGM bytes.
impl From<VgmDocument> for Vec<u8> {
    fn from(document: VgmDocument) -> Vec<u8> {
        document.to_bytes()
    }
}

/// Convert a borrowed `VgmDocument` into serialized bytes.
impl From<&VgmDocument> for Vec<u8> {
    fn from(document: &VgmDocument) -> Vec<u8> {
        document.to_bytes()
    }
}

impl VgmDocument {
    /// Return an iterator over `VgmCommand` references.
    pub fn iter(&self) -> std::slice::Iter<'_, VgmCommand> {
        self.commands.iter()
    }

    /// Return a mutable iterator over `VgmCommand` references.
    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, VgmCommand> {
        self.commands.iter_mut()
    }

    /// Iterate over commands together with their (offset, length) when
    /// serialized into VGM bytes.
    ///
    /// The iterator yields tuples `(offset, length, &VgmCommand)` where
    /// `offset` is the byte offset of the command relative to the start of
    /// the command stream (i.e. immediately after the header), and
    /// `length` is the serialized length in bytes. This does not serialize
    /// the full document; lengths are computed via
    /// `command_to_vgm_bytes` for each command.
    pub fn iter_with_offsets(&self) -> impl Iterator<Item = (usize, usize, &VgmCommand)> + '_ {
        let offs = self.command_offsets_and_lengths();
        offs.into_iter()
            .zip(self.commands.iter())
            .map(|((o, l), cmd)| (o, l, cmd))
    }
}

/// Consume the document and iterate its commands by value.
impl IntoIterator for VgmDocument {
    type Item = VgmCommand;
    type IntoIter = std::vec::IntoIter<VgmCommand>;

    fn into_iter(self) -> Self::IntoIter {
        self.commands.into_iter()
    }
}

/// Iterate over commands by reference: `for c in &doc { ... }`.
impl<'a> IntoIterator for &'a VgmDocument {
    type Item = &'a VgmCommand;
    type IntoIter = std::slice::Iter<'a, VgmCommand>;

    fn into_iter(self) -> Self::IntoIter {
        self.commands.iter()
    }
}

/// Iterate over commands by mutable reference: `for c in &mut doc { ... }`.
impl<'a> IntoIterator for &'a mut VgmDocument {
    type Item = &'a mut VgmCommand;
    type IntoIter = std::slice::IterMut<'a, VgmCommand>;

    fn into_iter(self) -> Self::IntoIter {
        self.commands.iter_mut()
    }
}
