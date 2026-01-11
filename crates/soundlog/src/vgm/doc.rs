use crate::chip;
use crate::meta::Gd3;
use crate::vgm::command::ChipId;
use crate::vgm::command::{ChipWriteSpec, VgmCommand};
use crate::vgm::header::VgmHeader;

#[derive(Debug, Clone, PartialEq, Default)]
/// A complete VGM document, consisting of a header, an ordered command
/// stream, and optional GD3 metadata.
///
/// Construct `VgmDocument` instances using `VgmBuilder`. Once assembled,
/// call `VgmDocument::to_bytes()` to obtain the serialized VGM file
/// bytes suitable for writing to disk.
pub struct VgmDocument {
    pub header: VgmHeader,
    pub commands: Vec<VgmCommand>,
    pub gd3: Option<Gd3>,
}

/// Builder for assembling a `VgmDocument`.
///
/// Use this builder to incrementally set header fields, register chip
/// clock frequencies, and append commands. Methods return `&mut Self` when
/// appropriate to allow chaining. Call `finalize()` to compute derived header
/// fields (for example `total_samples`) and obtain the completed
/// `VgmDocument`.
pub struct VgmBuilder {
    doc: VgmDocument,
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
            doc: VgmDocument::default(),
        }
    }

    /// Append a VGM command to the builder.
    ///
    /// Accepts any type convertible into `VgmCommand` (via `Into`).
    /// Returns `&mut Self` to allow method chaining.
    pub fn add_vgm_command<C>(&mut self, command: C) -> &mut Self
    where
        C: Into<VgmCommand>,
    {
        self.doc.commands.push(command.into());
        self
    }

    /// Append a chip write produced by a chip-specific spec.
    ///
    /// `chip_id` selects the chip instance (`ChipId::Primary` or `ChipId::Secondary`).
    /// `c` must implement `ChipWriteSpec`; the spec will push the appropriate
    /// `VgmCommand` into the builder's command stream. Returns `&mut Self`.
    pub fn add_chip_write<C, I>(&mut self, chip_id: I, c: C) -> &mut Self
    where
        I: Into<ChipId>,
        C: ChipWriteSpec,
    {
        c.write(chip_id.into(), &mut self.doc.commands);
        self
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
        I: Into<ChipId>,
    {
        let ch: chip::Chip = c.into();
        let instance: ChipId = chip_id.into();

        let clock = match instance {
            ChipId::Primary => master_clock,
            ChipId::Secondary => master_clock | 0x8000_0000u32,
        };

        match &ch {
            chip::Chip::Sn76489 => self.doc.header.sn76489_clock = clock,
            chip::Chip::Ym2413 => self.doc.header.ym2413_clock = clock,
            chip::Chip::Ym2612 => self.doc.header.ym2612_clock = clock,
            chip::Chip::Ym2151 => self.doc.header.ym2151_clock = clock,
            chip::Chip::SegaPcm => self.doc.header.sega_pcm_clock = clock,
            chip::Chip::Rf5c68 => self.doc.header.rf5c68_clock = clock,
            chip::Chip::Ym2203 => self.doc.header.ym2203_clock = clock,
            chip::Chip::Ym2608 => self.doc.header.ym2608_clock = clock,
            chip::Chip::Ym2610b => self.doc.header.ym2610b_clock = clock,
            chip::Chip::Ym3812 => self.doc.header.ym3812_clock = clock,
            chip::Chip::Ym3526 => self.doc.header.ym3526_clock = clock,
            chip::Chip::Y8950 => self.doc.header.y8950_clock = clock,
            chip::Chip::Ymf262 => self.doc.header.ymf262_clock = clock,
            chip::Chip::Ymf278b => self.doc.header.ymf278b_clock = clock,
            chip::Chip::Ymf271 => self.doc.header.ymf271_clock = clock,
            chip::Chip::Ymz280b => self.doc.header.ymz280b_clock = clock,
            chip::Chip::Rf5c164 => self.doc.header.rf5c164_clock = clock,
            chip::Chip::Pwm => self.doc.header.pwm_clock = clock,
            chip::Chip::Ay8910 => self.doc.header.ay8910_clock = clock,
            chip::Chip::GbDmg => self.doc.header.gb_dmg_clock = clock,
            chip::Chip::NesApu => self.doc.header.nes_apu_clock = clock,
            chip::Chip::MultiPcm => self.doc.header.multipcm_clock = clock,
            chip::Chip::Upd7759 => self.doc.header.upd7759_clock = clock,
            chip::Chip::Okim6258 => self.doc.header.okim6258_clock = clock,
            chip::Chip::Okim6295 => self.doc.header.okim6295_clock = clock,
            chip::Chip::K051649 => self.doc.header.k051649_clock = clock,
            chip::Chip::K054539 => self.doc.header.k054539_clock = clock,
            chip::Chip::Huc6280 => self.doc.header.huc6280_clock = clock,
            chip::Chip::C140 => self.doc.header.c140_clock = clock,
            chip::Chip::K053260 => self.doc.header.k053260_clock = clock,
            chip::Chip::Pokey => self.doc.header.pokey_clock = clock,
            chip::Chip::Qsound => self.doc.header.qsound_clock = clock,
            chip::Chip::Scsp => self.doc.header.scsp_clock = clock,
            chip::Chip::WonderSwan => self.doc.header.wonderswan_clock = clock,
            chip::Chip::Vsu => self.doc.header.vsu_clock = clock,
            chip::Chip::Saa1099 => self.doc.header.saa1099_clock = clock,
            chip::Chip::Es5503 => self.doc.header.es5503_clock = clock,
            chip::Chip::Es5506v8 => self.doc.header.es5506_clock = clock,
            chip::Chip::Es5506v16 => self.doc.header.es5506_clock = clock,
            chip::Chip::X1010 => self.doc.header.x1_010_clock = clock,
            chip::Chip::C352 => self.doc.header.c352_clock = clock,
            chip::Chip::Ga20 => self.doc.header.ga20_clock = clock,
            chip::Chip::Mikey => self.doc.header.mikey_clock = clock,
            _ => {}
        }
    }

    /// Finalize the builder and return the assembled `VgmDocument`.
    ///
    /// This computes derived header fields (for example `total_samples`) by
    /// scanning accumulated commands, and returns the complete document ready
    /// for serialization via `VgmDocument::to_bytes()`.
    pub fn finalize(mut self) -> VgmDocument {
        let total_sample: u32 = self
            .doc
            .commands
            .iter()
            .map(|cmd| match cmd {
                VgmCommand::WaitSamples(s) => s.0 as u32,
                VgmCommand::Wait735Samples(_) => 735,
                VgmCommand::Wait882Samples(_) => 882,
                VgmCommand::WaitNSample(s) => s.0 as u32,
                VgmCommand::YM2612Port0Address2AWriteAndWaitN(s) => s.0 as u32,
                _ => 0,
            })
            .sum::<u32>();
        self.doc.header.total_samples = total_sample;

        self.doc
    }
}
