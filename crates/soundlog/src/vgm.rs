use crate::chip;
use crate::meta::Gd3;

const VGM_V171_HEADER_SIZE: u32 = 0x100;

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
    pub fn add_command<C>(&mut self, command: C) -> &mut Self
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
    pub fn add_chip<C, I>(&mut self, c: C, chip_id: I, master_clock: u32)
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

#[derive(Debug, Clone, PartialEq)]
/// VGM file header fields and utilities for serialization.
pub struct VgmHeader {
    pub ident: [u8; 4],
    pub eof_offset: u32,
    pub version: u32,
    pub sn76489_clock: u32,
    pub ym2413_clock: u32,
    pub gd3_offset: u32,
    pub total_samples: u32,
    pub loop_offset: u32,
    pub loop_samples: u32,
    pub sample_rate: u32,
    pub sn_fb: u16,
    pub snw: u8,
    pub sf: u8,
    pub ym2612_clock: u32,
    pub ym2151_clock: u32,
    pub data_offset: u32,
    pub sega_pcm_clock: u32,
    pub spcm_interface: u32,
    pub rf5c68_clock: u32,
    pub ym2203_clock: u32,
    pub ym2608_clock: u32,
    pub ym2610b_clock: u32,
    pub ym3812_clock: u32,
    pub ym3526_clock: u32,
    pub y8950_clock: u32,
    pub ymf262_clock: u32,
    pub ymf278b_clock: u32,
    pub ymf271_clock: u32,
    pub ymz280b_clock: u32,
    pub rf5c164_clock: u32,
    pub pwm_clock: u32,
    pub ay8910_clock: u32,
    pub ay_misc: [u8; 8],
    pub gb_dmg_clock: u32,
    pub nes_apu_clock: u32,
    pub multipcm_clock: u32,
    pub upd7759_clock: u32,
    pub okim6258_clock: u32,
    pub okim6258_flags: [u8; 4],
    pub okim6295_clock: u32,
    pub k051649_clock: u32,
    pub k054539_clock: u32,
    pub huc6280_clock: u32,
    pub c140_clock: u32,
    pub k053260_clock: u32,
    pub pokey_clock: u32,
    pub qsound_clock: u32,
    pub scsp_clock: u32,
    pub extra_header_offset: u32,
    pub wonderswan_clock: u32,
    pub vsu_clock: u32,
    pub saa1099_clock: u32,
    pub es5503_clock: u32,
    pub es5506_clock: u32,
    pub es5506_channels: u16,
    pub es5506_cd: u8,
    pub es5506_reserved: u8,
    pub x1_010_clock: u32,
    pub c352_clock: u32,
    pub ga20_clock: u32,
    pub mikey_clock: u32,
    pub reserved_e8_ef: [u8; 8],
    pub reserved_f0_ff: [u8; 16],
}

impl Default for VgmHeader {
    fn default() -> Self {
        VgmHeader {
            ident: *b"Vgm ",
            eof_offset: 0,
            version: 0x00000172, // 1.72
            sn76489_clock: 0,
            ym2413_clock: 0,
            gd3_offset: 0,
            total_samples: 0,
            loop_offset: 0,
            loop_samples: 0,
            sample_rate: 44100,
            sn_fb: 0,
            snw: 0,
            sf: 0,
            ym2612_clock: 0,
            ym2151_clock: 0,
            data_offset: 0,
            sega_pcm_clock: 0,
            spcm_interface: 0,
            rf5c68_clock: 0,
            ym2203_clock: 0,
            ym2608_clock: 0,
            ym2610b_clock: 0,
            ym3812_clock: 0,
            ym3526_clock: 0,
            y8950_clock: 0,
            ymf262_clock: 0,
            ymf278b_clock: 0,
            ymf271_clock: 0,
            ymz280b_clock: 0,
            rf5c164_clock: 0,
            pwm_clock: 0,
            ay8910_clock: 0,
            ay_misc: [0u8; 8],
            gb_dmg_clock: 0,
            nes_apu_clock: 0,
            multipcm_clock: 0,
            upd7759_clock: 0,
            okim6258_clock: 0,
            okim6258_flags: [0u8; 4],
            okim6295_clock: 0,
            k051649_clock: 0,
            k054539_clock: 0,
            huc6280_clock: 0,
            c140_clock: 0,
            k053260_clock: 0,
            pokey_clock: 0,
            qsound_clock: 0,
            scsp_clock: 0,
            extra_header_offset: 0,
            wonderswan_clock: 0,
            vsu_clock: 0,
            saa1099_clock: 0,
            es5503_clock: 0,
            es5506_clock: 0,
            es5506_channels: 0,
            es5506_cd: 0,
            es5506_reserved: 0,
            x1_010_clock: 0,
            c352_clock: 0,
            ga20_clock: 0,
            mikey_clock: 0,
            reserved_e8_ef: [0u8; 8],
            reserved_f0_ff: [0u8; 16],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ChipId {
    Primary = 0x0,
    Secondary = 0x1,
}

impl From<usize> for ChipId {
    fn from(v: usize) -> Self {
        match v {
            0 => ChipId::Primary,
            1 => ChipId::Secondary,
            _ => panic!("Invalid ChipId from usize: {}", v),
        }
    }
}

impl From<ChipId> for usize {
    fn from(id: ChipId) -> Self {
        id as usize
    }
}

#[derive(Debug, Clone, PartialEq)]
/// All supported VGM commands and per-chip write variants.
pub enum VgmCommand {
    AY8910StereoMask(Ay8910StereoMask),
    WaitSamples(WaitSamples),
    Wait735Samples(Wait735Samples),
    Wait882Samples(Wait882Samples),
    EndOfData(EndOfData),
    DataBlock(DataBlock),
    PcmRamWrite(PcmRamWrite),
    WaitNSample(WaitNSample),
    YM2612Port0Address2AWriteAndWaitN(Ym2612Port0Address2AWriteAndWaitN),
    SetupStreamControl(SetupStreamControl),
    SetStreamData(SetStreamData),
    SetStreamFrequency(SetStreamFrequency),
    StartStream(StartStream),
    StopStream(StopStream),
    StartStreamFastCall(StartStreamFastCall),
    SeekOffset(SeekOffset),
    Sn76489Write(ChipId, chip::PsgSpec),
    Ym2413Write(ChipId, chip::Ym2413Spec),
    Ym2612Write(ChipId, chip::Ym2612Spec),
    Ym2151Write(ChipId, chip::Ym2151Spec),
    SegaPcmWrite(ChipId, chip::SegaPcmSpec),
    Rf5c68Write(ChipId, chip::Rf5c68Spec),
    Ym2203Write(ChipId, chip::Ym2203Spec),
    Ym2608Write(ChipId, chip::Ym2608Spec),
    Ym2610bWrite(ChipId, chip::Ym2610Spec),
    Ym3812Write(ChipId, chip::Ym3812Spec),
    Ym3526Write(ChipId, chip::Ym3526Spec),
    Y8950Write(ChipId, chip::Y8950Spec),
    Ymf262Write(ChipId, chip::Ymf262Spec),
    Ymf278bWrite(ChipId, chip::Ymf278bSpec),
    Ymf271Write(ChipId, chip::Ymf271Spec),
    Scc1Write(ChipId, chip::Scc1Spec),
    Ymz280bWrite(ChipId, chip::Ymz280bSpec),
    Rf5c164Write(ChipId, chip::Rf5c164Spec),
    PwmWrite(ChipId, chip::PwmSpec),
    Ay8910Write(ChipId, chip::Ay8910Spec),
    GbDmgWrite(ChipId, chip::GbDmgSpec),
    NesApuWrite(ChipId, chip::NesApuSpec),
    MultiPcmWrite(ChipId, chip::MultiPcmSpec),
    Upd7759Write(ChipId, chip::Upd7759Spec),
    Okim6258Write(ChipId, chip::Okim6258Spec),
    Okim6295Write(ChipId, chip::Okim6295Spec),
    K051649Write(ChipId, chip::K051649Spec),
    K054539Write(ChipId, chip::K054539Spec),
    Huc6280Write(ChipId, chip::Huc6280Spec),
    C140Write(ChipId, chip::C140Spec),
    K053260Write(ChipId, chip::K053260Spec),
    PokeyWrite(ChipId, chip::PokeySpec),
    QsoundWrite(ChipId, chip::QsoundSpec),
    ScspWrite(ChipId, chip::ScspSpec),
    WonderSwanWrite(ChipId, chip::WonderSwanSpec),
    VsuWrite(ChipId, chip::VsuSpec),
    Saa1099Write(ChipId, chip::Saa1099Spec),
    Es5503Write(ChipId, chip::Es5503Spec),
    Es5506v8Write(ChipId, chip::Es5506v8Spec),
    Es5506v16Write(ChipId, chip::Es5506v16Spec),
    X1010Write(ChipId, chip::X1010Spec),
    C352Write(ChipId, chip::C352Spec),
    Ga20Write(ChipId, chip::Ga20Spec),
    MikeyWrite(ChipId, chip::MikeySpec),
    GameGearPsgWrite(ChipId, chip::GameGearPsgSpec),
}

pub trait WriteCommand {
    fn opcode(&self) -> u8;
    fn decode_vgm_bytes(&self, dest: &mut Vec<u8>);
}

#[derive(Debug, Clone, PartialEq)]
pub struct Ay8910StereoMask(pub u8);

#[derive(Debug, Clone, PartialEq)]
pub struct WaitSamples(pub u16);

#[derive(Debug, Clone, PartialEq)]
pub struct Wait735Samples;

#[derive(Debug, Clone, PartialEq)]
pub struct Wait882Samples;

#[derive(Debug, Clone, PartialEq)]
pub struct EndOfData;

#[derive(Debug, Clone, PartialEq)]
pub struct DataBlock {
    pub data_type: u8,
    pub size: u32,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PcmRamWrite {
    pub chip_type: u8,
    pub offset: u32,
    pub write_offset: u32,
    pub size_of_data: u32,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WaitNSample(pub u8);

#[derive(Debug, Clone, PartialEq)]
pub struct Ym2612Port0Address2AWriteAndWaitN(pub u8);

#[derive(Debug, Clone, PartialEq)]
pub struct SetupStreamControl {
    pub stream_number: u8,
    pub stream_type: u8,
    pub pan: u8,
    pub volume: u8,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SetStreamData {
    pub stream_number: u8,
    pub data_block_number: u8,
    pub loop_count: u8,
    pub playback_rate: u8,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SetStreamFrequency {
    pub stream_number: u8,
    pub frequency: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StartStream {
    pub stream_number: u8,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StopStream {
    pub stream_number: u8,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StartStreamFastCall {
    pub stream_number: u8,
    pub offset: u16,
    pub playback_rate: u8,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SeekOffset(pub u32);

impl WriteCommand for Ay8910StereoMask {
    fn opcode(&self) -> u8 {
        0x31
    }
    fn decode_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
        dest.push(self.0);
    }
}

impl WriteCommand for WaitSamples {
    fn opcode(&self) -> u8 {
        0x61
    }
    fn decode_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
        dest.push((self.0 & 0xFF) as u8);
        dest.push((self.0 >> 8) as u8);
    }
}

impl WriteCommand for Wait735Samples {
    fn opcode(&self) -> u8 {
        0x62
    }
    fn decode_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
    }
}

impl WriteCommand for Wait882Samples {
    fn opcode(&self) -> u8 {
        0x63
    }
    fn decode_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
    }
}

impl WriteCommand for EndOfData {
    fn opcode(&self) -> u8 {
        0x66
    }
    fn decode_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
    }
}

impl WriteCommand for DataBlock {
    fn opcode(&self) -> u8 {
        0x67
    }
    fn decode_vgm_bytes(&self, dest: &mut Vec<u8>) {
        // 0x67 0x66 tt ss ss ss ss data...
        dest.push(self.opcode());
        dest.push(0x66);
        dest.push(self.data_type);
        dest.extend_from_slice(&self.size.to_le_bytes());
        dest.extend_from_slice(&self.data);
    }
}

impl WriteCommand for PcmRamWrite {
    fn opcode(&self) -> u8 {
        0x68
    }
    fn decode_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
        dest.push(0x66);
        dest.push(self.chip_type);
        let o = self.offset & 0x00FF_FFFF;
        dest.push(((o >> 16) & 0xFF) as u8);
        dest.push(((o >> 8) & 0xFF) as u8);
        dest.push((o & 0xFF) as u8);
        let w = self.write_offset & 0x00FF_FFFF;
        dest.push(((w >> 16) & 0xFF) as u8);
        dest.push(((w >> 8) & 0xFF) as u8);
        dest.push((w & 0xFF) as u8);
        let s = self.size_of_data & 0x00FF_FFFF;
        dest.push(((s >> 16) & 0xFF) as u8);
        dest.push(((s >> 8) & 0xFF) as u8);
        dest.push((s & 0xFF) as u8);
        dest.extend_from_slice(&self.data);
    }
}

impl WriteCommand for WaitNSample {
    fn opcode(&self) -> u8 {
        0x70u8.wrapping_add(self.0.saturating_sub(1))
    }
    fn decode_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
    }
}

impl WriteCommand for Ym2612Port0Address2AWriteAndWaitN {
    fn opcode(&self) -> u8 {
        0x80u8.wrapping_add(self.0)
    }
    fn decode_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
    }
}

impl WriteCommand for SetupStreamControl {
    fn opcode(&self) -> u8 {
        0x90
    }
    fn decode_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
        dest.push(self.stream_number);
        dest.push(self.stream_type);
        dest.push(self.pan);
        dest.push(self.volume);
    }
}

impl WriteCommand for SetStreamData {
    fn opcode(&self) -> u8 {
        0x91
    }
    fn decode_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
        dest.push(self.stream_number);
        dest.push(self.data_block_number);
        dest.push(self.loop_count);
        dest.push(self.playback_rate);
    }
}

impl WriteCommand for SetStreamFrequency {
    fn opcode(&self) -> u8 {
        0x92
    }
    fn decode_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
        dest.push(self.stream_number);
        dest.extend_from_slice(&self.frequency.to_le_bytes());
    }
}

impl WriteCommand for StartStream {
    fn opcode(&self) -> u8 {
        0x93
    }
    fn decode_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
        dest.push(self.stream_number);
    }
}

impl WriteCommand for StopStream {
    fn opcode(&self) -> u8 {
        0x94
    }
    fn decode_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
        dest.push(self.stream_number);
    }
}

impl WriteCommand for StartStreamFastCall {
    fn opcode(&self) -> u8 {
        0x95
    }
    fn decode_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
        dest.push(self.stream_number);
        dest.push((self.offset >> 8) as u8);
        dest.push(self.offset as u8);
        dest.push(self.playback_rate);
    }
}

impl WriteCommand for SeekOffset {
    fn opcode(&self) -> u8 {
        0xE0
    }
    fn decode_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
        dest.extend_from_slice(&self.0.to_le_bytes());
    }
}

impl From<Ay8910StereoMask> for VgmCommand {
    fn from(s: Ay8910StereoMask) -> Self {
        VgmCommand::AY8910StereoMask(s)
    }
}

impl From<WaitSamples> for VgmCommand {
    fn from(s: WaitSamples) -> Self {
        VgmCommand::WaitSamples(s)
    }
}

impl From<Wait735Samples> for VgmCommand {
    fn from(s: Wait735Samples) -> Self {
        VgmCommand::Wait735Samples(s)
    }
}

impl From<Wait882Samples> for VgmCommand {
    fn from(s: Wait882Samples) -> Self {
        VgmCommand::Wait882Samples(s)
    }
}

impl From<EndOfData> for VgmCommand {
    fn from(s: EndOfData) -> Self {
        VgmCommand::EndOfData(s)
    }
}

impl From<DataBlock> for VgmCommand {
    fn from(s: DataBlock) -> Self {
        VgmCommand::DataBlock(s)
    }
}

impl From<PcmRamWrite> for VgmCommand {
    fn from(s: PcmRamWrite) -> Self {
        VgmCommand::PcmRamWrite(s)
    }
}

impl From<WaitNSample> for VgmCommand {
    fn from(s: WaitNSample) -> Self {
        VgmCommand::WaitNSample(s)
    }
}

impl From<Ym2612Port0Address2AWriteAndWaitN> for VgmCommand {
    fn from(s: Ym2612Port0Address2AWriteAndWaitN) -> Self {
        VgmCommand::YM2612Port0Address2AWriteAndWaitN(s)
    }
}

impl From<SetupStreamControl> for VgmCommand {
    fn from(s: SetupStreamControl) -> Self {
        VgmCommand::SetupStreamControl(s)
    }
}

impl From<SetStreamData> for VgmCommand {
    fn from(s: SetStreamData) -> Self {
        VgmCommand::SetStreamData(s)
    }
}

impl From<SetStreamFrequency> for VgmCommand {
    fn from(s: SetStreamFrequency) -> Self {
        VgmCommand::SetStreamFrequency(s)
    }
}

impl From<StartStream> for VgmCommand {
    fn from(s: StartStream) -> Self {
        VgmCommand::StartStream(s)
    }
}

impl From<StopStream> for VgmCommand {
    fn from(s: StopStream) -> Self {
        VgmCommand::StopStream(s)
    }
}

impl From<StartStreamFastCall> for VgmCommand {
    fn from(s: StartStreamFastCall) -> Self {
        VgmCommand::StartStreamFastCall(s)
    }
}

impl From<SeekOffset> for VgmCommand {
    fn from(s: SeekOffset) -> Self {
        VgmCommand::SeekOffset(s)
    }
}

pub trait ChipWriteSpec {
    fn write(self, chip_id: ChipId, cmds: &mut Vec<VgmCommand>);
}

impl ChipWriteSpec for chip::PsgSpec {
    fn write(self, chip_id: ChipId, cmds: &mut Vec<VgmCommand>) {
        cmds.push(VgmCommand::Sn76489Write(chip_id, self));
    }
}

impl ChipWriteSpec for chip::Ym2413Spec {
    fn write(self, chip_id: ChipId, cmds: &mut Vec<VgmCommand>) {
        cmds.push(VgmCommand::Ym2413Write(chip_id, self));
    }
}

impl ChipWriteSpec for chip::Ym2612Spec {
    fn write(self, chip_id: ChipId, cmds: &mut Vec<VgmCommand>) {
        cmds.push(VgmCommand::Ym2612Write(chip_id, self));
    }
}

impl ChipWriteSpec for chip::Ym2151Spec {
    fn write(self, chip_id: ChipId, cmds: &mut Vec<VgmCommand>) {
        cmds.push(VgmCommand::Ym2151Write(chip_id, self));
    }
}

impl ChipWriteSpec for chip::SegaPcmSpec {
    fn write(self, chip_id: ChipId, cmds: &mut Vec<VgmCommand>) {
        cmds.push(VgmCommand::SegaPcmWrite(chip_id, self));
    }
}

impl ChipWriteSpec for chip::Rf5c68Spec {
    fn write(self, chip_id: ChipId, cmds: &mut Vec<VgmCommand>) {
        cmds.push(VgmCommand::Rf5c68Write(chip_id, self));
    }
}

impl ChipWriteSpec for chip::Ym2203Spec {
    fn write(self, chip_id: ChipId, cmds: &mut Vec<VgmCommand>) {
        cmds.push(VgmCommand::Ym2203Write(chip_id, self));
    }
}

impl ChipWriteSpec for chip::Ym2608Spec {
    fn write(self, chip_id: ChipId, cmds: &mut Vec<VgmCommand>) {
        cmds.push(VgmCommand::Ym2608Write(chip_id, self));
    }
}

impl ChipWriteSpec for chip::Ym2610Spec {
    fn write(self, chip_id: ChipId, cmds: &mut Vec<VgmCommand>) {
        cmds.push(VgmCommand::Ym2610bWrite(chip_id, self));
    }
}

impl ChipWriteSpec for chip::Ym3812Spec {
    fn write(self, chip_id: ChipId, cmds: &mut Vec<VgmCommand>) {
        cmds.push(VgmCommand::Ym3812Write(chip_id, self));
    }
}

impl ChipWriteSpec for chip::Ym3526Spec {
    fn write(self, chip_id: ChipId, cmds: &mut Vec<VgmCommand>) {
        cmds.push(VgmCommand::Ym3526Write(chip_id, self));
    }
}

impl ChipWriteSpec for chip::Y8950Spec {
    fn write(self, chip_id: ChipId, cmds: &mut Vec<VgmCommand>) {
        cmds.push(VgmCommand::Y8950Write(chip_id, self));
    }
}

impl ChipWriteSpec for chip::Ymf262Spec {
    fn write(self, chip_id: ChipId, cmds: &mut Vec<VgmCommand>) {
        cmds.push(VgmCommand::Ymf262Write(chip_id, self));
    }
}

impl ChipWriteSpec for chip::Ymf278bSpec {
    fn write(self, chip_id: ChipId, cmds: &mut Vec<VgmCommand>) {
        cmds.push(VgmCommand::Ymf278bWrite(chip_id, self));
    }
}

impl ChipWriteSpec for chip::Ymf271Spec {
    fn write(self, chip_id: ChipId, cmds: &mut Vec<VgmCommand>) {
        cmds.push(VgmCommand::Ymf271Write(chip_id, self));
    }
}

impl ChipWriteSpec for chip::Scc1Spec {
    fn write(self, chip_id: ChipId, cmds: &mut Vec<VgmCommand>) {
        cmds.push(VgmCommand::Scc1Write(chip_id, self));
    }
}

impl ChipWriteSpec for chip::Ymz280bSpec {
    fn write(self, chip_id: ChipId, cmds: &mut Vec<VgmCommand>) {
        cmds.push(VgmCommand::Ymz280bWrite(chip_id, self));
    }
}

impl ChipWriteSpec for chip::Rf5c164Spec {
    fn write(self, chip_id: ChipId, cmds: &mut Vec<VgmCommand>) {
        cmds.push(VgmCommand::Rf5c164Write(chip_id, self));
    }
}

impl ChipWriteSpec for chip::PwmSpec {
    fn write(self, chip_id: ChipId, cmds: &mut Vec<VgmCommand>) {
        cmds.push(VgmCommand::PwmWrite(chip_id, self));
    }
}

impl ChipWriteSpec for chip::Ay8910Spec {
    fn write(self, chip_id: ChipId, cmds: &mut Vec<VgmCommand>) {
        cmds.push(VgmCommand::Ay8910Write(chip_id, self));
    }
}

impl ChipWriteSpec for chip::GbDmgSpec {
    fn write(self, chip_id: ChipId, cmds: &mut Vec<VgmCommand>) {
        cmds.push(VgmCommand::GbDmgWrite(chip_id, self));
    }
}

impl ChipWriteSpec for chip::NesApuSpec {
    fn write(self, chip_id: ChipId, cmds: &mut Vec<VgmCommand>) {
        cmds.push(VgmCommand::NesApuWrite(chip_id, self));
    }
}

impl ChipWriteSpec for chip::MultiPcmSpec {
    fn write(self, chip_id: ChipId, cmds: &mut Vec<VgmCommand>) {
        cmds.push(VgmCommand::MultiPcmWrite(chip_id, self));
    }
}

impl ChipWriteSpec for chip::Upd7759Spec {
    fn write(self, chip_id: ChipId, cmds: &mut Vec<VgmCommand>) {
        cmds.push(VgmCommand::Upd7759Write(chip_id, self));
    }
}

impl ChipWriteSpec for chip::Okim6258Spec {
    fn write(self, chip_id: ChipId, cmds: &mut Vec<VgmCommand>) {
        cmds.push(VgmCommand::Okim6258Write(chip_id, self));
    }
}

impl ChipWriteSpec for chip::Okim6295Spec {
    fn write(self, chip_id: ChipId, cmds: &mut Vec<VgmCommand>) {
        cmds.push(VgmCommand::Okim6295Write(chip_id, self));
    }
}

impl ChipWriteSpec for chip::K051649Spec {
    fn write(self, chip_id: ChipId, cmds: &mut Vec<VgmCommand>) {
        cmds.push(VgmCommand::K051649Write(chip_id, self));
    }
}

impl ChipWriteSpec for chip::K054539Spec {
    fn write(self, chip_id: ChipId, cmds: &mut Vec<VgmCommand>) {
        cmds.push(VgmCommand::K054539Write(chip_id, self));
    }
}

impl ChipWriteSpec for chip::Huc6280Spec {
    fn write(self, chip_id: ChipId, cmds: &mut Vec<VgmCommand>) {
        cmds.push(VgmCommand::Huc6280Write(chip_id, self));
    }
}

impl ChipWriteSpec for chip::C140Spec {
    fn write(self, chip_id: ChipId, cmds: &mut Vec<VgmCommand>) {
        cmds.push(VgmCommand::C140Write(chip_id, self));
    }
}

impl ChipWriteSpec for chip::K053260Spec {
    fn write(self, chip_id: ChipId, cmds: &mut Vec<VgmCommand>) {
        cmds.push(VgmCommand::K053260Write(chip_id, self));
    }
}

impl ChipWriteSpec for chip::PokeySpec {
    fn write(self, chip_id: ChipId, cmds: &mut Vec<VgmCommand>) {
        cmds.push(VgmCommand::PokeyWrite(chip_id, self));
    }
}

impl ChipWriteSpec for chip::QsoundSpec {
    fn write(self, chip_id: ChipId, cmds: &mut Vec<VgmCommand>) {
        cmds.push(VgmCommand::QsoundWrite(chip_id, self));
    }
}

impl ChipWriteSpec for chip::ScspSpec {
    fn write(self, chip_id: ChipId, cmds: &mut Vec<VgmCommand>) {
        cmds.push(VgmCommand::ScspWrite(chip_id, self));
    }
}

impl ChipWriteSpec for chip::WonderSwanSpec {
    fn write(self, chip_id: ChipId, cmds: &mut Vec<VgmCommand>) {
        cmds.push(VgmCommand::WonderSwanWrite(chip_id, self));
    }
}

impl ChipWriteSpec for chip::VsuSpec {
    fn write(self, chip_id: ChipId, cmds: &mut Vec<VgmCommand>) {
        cmds.push(VgmCommand::VsuWrite(chip_id, self));
    }
}

impl ChipWriteSpec for chip::Saa1099Spec {
    fn write(self, chip_id: ChipId, cmds: &mut Vec<VgmCommand>) {
        cmds.push(VgmCommand::Saa1099Write(chip_id, self));
    }
}

impl ChipWriteSpec for chip::Es5503Spec {
    fn write(self, chip_id: ChipId, cmds: &mut Vec<VgmCommand>) {
        cmds.push(VgmCommand::Es5503Write(chip_id, self));
    }
}

impl ChipWriteSpec for chip::Es5506v8Spec {
    fn write(self, chip_id: ChipId, cmds: &mut Vec<VgmCommand>) {
        cmds.push(VgmCommand::Es5506v8Write(chip_id, self));
    }
}

impl ChipWriteSpec for chip::Es5506v16Spec {
    fn write(self, chip_id: ChipId, cmds: &mut Vec<VgmCommand>) {
        cmds.push(VgmCommand::Es5506v16Write(chip_id, self));
    }
}

impl ChipWriteSpec for chip::X1010Spec {
    fn write(self, chip_id: ChipId, cmds: &mut Vec<VgmCommand>) {
        cmds.push(VgmCommand::X1010Write(chip_id, self));
    }
}

impl ChipWriteSpec for chip::C352Spec {
    fn write(self, chip_id: ChipId, cmds: &mut Vec<VgmCommand>) {
        cmds.push(VgmCommand::C352Write(chip_id, self));
    }
}

impl ChipWriteSpec for chip::Ga20Spec {
    fn write(self, chip_id: ChipId, cmds: &mut Vec<VgmCommand>) {
        cmds.push(VgmCommand::Ga20Write(chip_id, self));
    }
}

impl ChipWriteSpec for chip::MikeySpec {
    fn write(self, chip_id: ChipId, cmds: &mut Vec<VgmCommand>) {
        cmds.push(VgmCommand::MikeyWrite(chip_id, self));
    }
}

impl ChipWriteSpec for chip::GameGearPsgSpec {
    fn write(self, chip_id: ChipId, cmds: &mut Vec<VgmCommand>) {
        cmds.push(VgmCommand::GameGearPsgWrite(chip_id, self));
    }
}

impl WriteCommand for chip::PsgSpec {
    // PSG (SN76489/SN76496) write value dd
    fn opcode(&self) -> u8 {
        0x50
    }
    fn decode_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
        dest.push(self.value);
    }
}

impl WriteCommand for chip::Ym2413Spec {
    // YM2413, write value dd to register aa
    fn opcode(&self) -> u8 {
        0x51
    }
    fn decode_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
        dest.push(self.register);
        dest.push(self.value);
    }
}

impl WriteCommand for chip::Ym2612Spec {
    // YM2612 port 0, write value dd to register aa
    // YM2612 port 1, write value dd to register aa
    fn opcode(&self) -> u8 {
        if self.port == 0 { 0x52 } else { 0x53 }
    }
    fn decode_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
        dest.push(self.register);
        dest.push(self.value);
    }
}

impl WriteCommand for chip::Ym2151Spec {
    // YM2151, write value dd to register aa
    fn opcode(&self) -> u8 {
        0x54
    }
    fn decode_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
        dest.push(self.register);
        dest.push(self.value);
    }
}

impl WriteCommand for chip::SegaPcmSpec {
    // SegaPCM, write value dd to memory offset aabb
    fn opcode(&self) -> u8 {
        0xC0
    }
    fn decode_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
        dest.push((self.offset >> 8) as u8);
        dest.push(self.offset as u8);
        dest.push(self.value);
    }
}

impl WriteCommand for chip::Rf5c68Spec {
    // RF5C68, write value dd to memory offset aabb
    fn opcode(&self) -> u8 {
        0xC1
    }
    fn decode_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
        dest.push((self.offset >> 8) as u8);
        dest.push(self.offset as u8);
        dest.push(self.value);
    }
}

impl WriteCommand for chip::Ym2203Spec {
    // YM2203, write value dd to register aa
    fn opcode(&self) -> u8 {
        0x55
    }
    fn decode_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
        dest.push(self.register);
        dest.push(self.value);
    }
}

impl WriteCommand for chip::Ym2608Spec {
    // YM2608 port 0, write value dd to register aa
    // YM2608 port 1, write value dd to register aa
    fn opcode(&self) -> u8 {
        if self.port == 0 { 0x56 } else { 0x57 }
    }
    fn decode_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
        dest.push(self.register);
        dest.push(self.value);
    }
}

impl WriteCommand for chip::Ym2610Spec {
    // YM2610 port 0, write value dd to register aa
    // YM2610 port 1, write value dd to register aa
    fn opcode(&self) -> u8 {
        if self.port == 0 { 0x58 } else { 0x59 }
    }
    fn decode_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
        dest.push(self.register);
        dest.push(self.value);
    }
}

impl WriteCommand for chip::Ym3812Spec {
    // YM3812, write value dd to register aa
    fn opcode(&self) -> u8 {
        0x5A
    }
    fn decode_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
        dest.push(self.register);
        dest.push(self.value);
    }
}

impl WriteCommand for chip::Ym3526Spec {
    // YM3526, write value dd to register aa
    fn opcode(&self) -> u8 {
        0x5B
    }
    fn decode_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
        dest.push(self.register);
        dest.push(self.value);
    }
}

impl WriteCommand for chip::Y8950Spec {
    // Y8950, write value dd to register aa
    fn opcode(&self) -> u8 {
        0x5C
    }
    fn decode_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
        dest.push(self.register);
        dest.push(self.value);
    }
}

impl WriteCommand for chip::Ymf262Spec {
    // YMF262 port 0, write value dd to register aa
    // YMF262 port 1, write value dd to register aa
    fn opcode(&self) -> u8 {
        if self.port == 0 { 0x5E } else { 0x5F }
    }
    fn decode_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
        dest.push(self.register);
        dest.push(self.value);
    }
}

impl WriteCommand for chip::Ymf278bSpec {
    // YMF278B, port pp, write value dd to register aa
    fn opcode(&self) -> u8 {
        0xD0
    }
    fn decode_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
        dest.push(self.port);
        dest.push(self.register);
        dest.push(self.value);
    }
}

impl WriteCommand for chip::Ymf271Spec {
    // YMF271, port pp, write value dd to register aa
    fn opcode(&self) -> u8 {
        0xD1
    }
    fn decode_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
        dest.push(self.port);
        dest.push(self.register);
        dest.push(self.value);
    }
}

impl WriteCommand for chip::Scc1Spec {
    // SCC1, port pp, write value dd to register aa
    fn opcode(&self) -> u8 {
        0xD2
    }
    fn decode_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
        dest.push(self.port);
        dest.push(self.register);
        dest.push(self.value);
    }
}

impl WriteCommand for chip::Ymz280bSpec {
    // YMZ280B, write value dd to register aa
    fn opcode(&self) -> u8 {
        0x5D
    }
    fn decode_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
        dest.push(self.register);
        dest.push(self.value);
    }
}

impl WriteCommand for chip::Rf5c164Spec {
    // RF5C164, write value dd to register aa
    fn opcode(&self) -> u8 {
        0xB1
    }
    fn decode_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
        dest.push(self.register);
        dest.push(self.value);
    }
}

impl WriteCommand for chip::PwmSpec {
    // PWM, write value ddd to register a (d is MSB, dd is LSB)
    fn opcode(&self) -> u8 {
        0xB2
    }
    fn decode_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
        dest.push(self.register);
        let v = self.value & 0x00FF_FFFF;
        dest.push(((v >> 16) & 0xFF) as u8);
        dest.push(((v >> 8) & 0xFF) as u8);
        dest.push((v & 0xFF) as u8);
    }
}

impl WriteCommand for chip::Ay8910Spec {
    // AY8910, write value dd to register aa
    fn opcode(&self) -> u8 {
        0xA0
    }
    fn decode_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
        dest.push(self.register);
        dest.push(self.value);
    }
}

impl WriteCommand for chip::GbDmgSpec {
    // GameBoy DMG, write value dd to register aa
    fn opcode(&self) -> u8 {
        0xB3
    }
    fn decode_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
        dest.push(self.register);
        dest.push(self.value);
    }
}

impl WriteCommand for chip::NesApuSpec {
    // NES APU, write value dd to register aa
    fn opcode(&self) -> u8 {
        0xB4
    }
    fn decode_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
        dest.push(self.register);
        dest.push(self.value);
    }
}

impl WriteCommand for chip::MultiPcmSpec {
    // MultiPCM, write value dd to register aa
    fn opcode(&self) -> u8 {
        0xB5
    }
    fn decode_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
        dest.push(self.register);
        dest.push(self.value);
    }
}

impl WriteCommand for chip::Upd7759Spec {
    // uPD7759, write value dd to register aa
    fn opcode(&self) -> u8 {
        0xB6
    }
    fn decode_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
        dest.push(self.register);
        dest.push(self.value);
    }
}

impl WriteCommand for chip::Okim6258Spec {
    // OKIM6258, write value dd to register aa
    fn opcode(&self) -> u8 {
        0xB7
    }
    fn decode_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
        dest.push(self.register);
        dest.push(self.value);
    }
}

impl WriteCommand for chip::Okim6295Spec {
    // OKIM6295, write value dd to register aa
    fn opcode(&self) -> u8 {
        0xB8
    }
    fn decode_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
        dest.push(self.register);
        dest.push(self.value);
    }
}

impl WriteCommand for chip::K051649Spec {
    // TODO: K051649, write value dd to register ppaa
    fn opcode(&self) -> u8 {
        0x00
    }
    fn decode_vgm_bytes(&self, dest: &mut Vec<u8>) {
        let _ = dest;
        unimplemented!("chip::K051649Spec");
    }
}

impl WriteCommand for chip::K054539Spec {
    // K054539, write value dd to register ppaa
    fn opcode(&self) -> u8 {
        0xD3
    }
    fn decode_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
        dest.push((self.register >> 8) as u8);
        dest.push(self.register as u8);
        dest.push(self.value);
    }
}

impl WriteCommand for chip::Huc6280Spec {
    // HuC6280, write value dd to register aa
    fn opcode(&self) -> u8 {
        0xB9
    }
    fn decode_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
        dest.push(self.register);
        dest.push(self.value);
    }
}

impl WriteCommand for chip::C140Spec {
    // C140, write value dd to register ppaa
    fn opcode(&self) -> u8 {
        0xD4
    }
    fn decode_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
        dest.push((self.register >> 8) as u8);
        dest.push(self.register as u8);
        dest.push(self.value);
    }
}

impl WriteCommand for chip::K053260Spec {
    // K053260, write value dd to register aa
    fn opcode(&self) -> u8 {
        0xBA
    }
    fn decode_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
        dest.push(self.register);
        dest.push(self.value);
    }
}

impl WriteCommand for chip::PokeySpec {
    // Pokey, write value dd to register aa
    fn opcode(&self) -> u8 {
        0xBB
    }
    fn decode_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
        dest.push(self.register);
        dest.push(self.value);
    }
}

impl WriteCommand for chip::QsoundSpec {
    // QSound, write value mmll to register rr (mm - data MSB, ll - data LSB)
    fn opcode(&self) -> u8 {
        0xC4
    }
    fn decode_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
        dest.push(self.register);
        dest.push(((self.value >> 8) & 0xFF) as u8);
        dest.push((self.value & 0xFF) as u8);
    }
}

impl WriteCommand for chip::ScspSpec {
    // SCSP, write value dd to memory offset mmll (mm - offset MSB, ll - offset LSB)
    fn opcode(&self) -> u8 {
        0xC5
    }
    fn decode_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
        dest.push((self.offset >> 8) as u8);
        dest.push(self.offset as u8);
        dest.push(self.value);
    }
}

impl WriteCommand for chip::WonderSwanSpec {
    // WonderSwan, write value dd to memory offset mmll (mm - offset MSB, ll - offset LSB)
    fn opcode(&self) -> u8 {
        0xC6
    }
    fn decode_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
        dest.push((self.offset >> 8) as u8);
        dest.push(self.offset as u8);
        dest.push(self.value);
    }
}

impl WriteCommand for chip::VsuSpec {
    // VSU, write value dd to memory offset mmll (mm - offset MSB, ll - offset LSB)
    fn opcode(&self) -> u8 {
        0xC7
    }
    fn decode_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
        dest.push((self.offset >> 8) as u8);
        dest.push(self.offset as u8);
        dest.push(self.value);
    }
}

impl WriteCommand for chip::Saa1099Spec {
    // SAA1099, write value dd to register aa
    fn opcode(&self) -> u8 {
        0xBD
    }
    fn decode_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
        dest.push(self.register);
        dest.push(self.value);
    }
}

impl WriteCommand for chip::Es5503Spec {
    // ES5503, write value dd to register ppaa
    fn opcode(&self) -> u8 {
        0xD5
    }
    fn decode_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
        dest.push((self.register >> 8) as u8);
        dest.push(self.register as u8);
        dest.push(self.value);
    }
}

impl WriteCommand for chip::Es5506v8Spec {
    // ES5506, write value dd to register aa
    //  Note: This command writes 8-bit data.
    fn opcode(&self) -> u8 {
        0xBE
    }
    fn decode_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
        dest.push(self.register);
        dest.push(self.value);
    }
}

impl WriteCommand for chip::Es5506v16Spec {
    // ES5506, write value aadd to register pp
    //  Note: This command writes 16-bit data.
    fn opcode(&self) -> u8 {
        0xD6
    }
    fn decode_vgm_bytes(&self, dest: &mut Vec<u8>) {
        // TODO: Support 16-bit data write
        dest.push(self.opcode());
        dest.push(self.register);
        dest.push(((self.value >> 8) & 0xFF) as u8);
        dest.push((self.value & 0xFF) as u8);
    }
}

impl WriteCommand for chip::X1010Spec {
    // X1-010, write value dd to memory offset mmll (mm - offset MSB, ll - offset LSB)
    fn opcode(&self) -> u8 {
        0xC8
    }
    fn decode_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
        dest.push((self.offset >> 8) as u8);
        dest.push(self.offset as u8);
        dest.push(self.value);
    }
}

impl WriteCommand for chip::C352Spec {
    // C352, write value aadd to register mmll
    fn opcode(&self) -> u8 {
        0xE1
    }
    fn decode_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
        dest.push((self.register >> 8) as u8);
        dest.push(self.register as u8);
        dest.push(((self.value >> 8) & 0xFF) as u8);
        dest.push((self.value & 0xFF) as u8);
    }
}

impl WriteCommand for chip::Ga20Spec {
    // GA20, write value dd to register aa
    fn opcode(&self) -> u8 {
        0xBF
    }
    fn decode_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
        dest.push(self.register);
        dest.push(self.value);
    }
}

impl WriteCommand for chip::MikeySpec {
    // Mikey, write value dd to register aa
    fn opcode(&self) -> u8 {
        0x40
    }
    fn decode_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
        dest.push(self.register);
        dest.push(self.value);
    }
}

impl WriteCommand for chip::GameGearPsgSpec {
    // Game Gear PSG, write value dd
    fn opcode(&self) -> u8 {
        0x4F
    }
    fn decode_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
        dest.push(self.value);
    }
}

impl Default for VgmBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl VgmDocument {
    pub fn to_bytes(&self) -> Vec<u8> {
        fn adjust_opcode_for_chip_id(instance_id: ChipId, opcode: u8) -> u8 {
            match instance_id {
                ChipId::Primary => opcode,
                ChipId::Secondary => opcode.wrapping_add(0x50),
            }
        }

        fn emit_chip<C: WriteCommand + ?Sized>(id: ChipId, spec: &C, cmd_buf: &mut Vec<u8>) {
            let start = cmd_buf.len();
            spec.decode_vgm_bytes(cmd_buf);
            cmd_buf[start] = adjust_opcode_for_chip_id(id, cmd_buf[start]);
        }

        let mut cmd_buf: Vec<u8> = Vec::new();

        for cmd in &self.commands {
            match cmd {
                VgmCommand::AY8910StereoMask(s) => s.decode_vgm_bytes(&mut cmd_buf),
                VgmCommand::WaitSamples(s) => s.decode_vgm_bytes(&mut cmd_buf),
                VgmCommand::Wait735Samples(s) => s.decode_vgm_bytes(&mut cmd_buf),
                VgmCommand::Wait882Samples(s) => s.decode_vgm_bytes(&mut cmd_buf),
                VgmCommand::EndOfData(s) => s.decode_vgm_bytes(&mut cmd_buf),
                VgmCommand::DataBlock(s) => s.decode_vgm_bytes(&mut cmd_buf),
                VgmCommand::PcmRamWrite(s) => s.decode_vgm_bytes(&mut cmd_buf),
                VgmCommand::WaitNSample(s) => s.decode_vgm_bytes(&mut cmd_buf),
                VgmCommand::YM2612Port0Address2AWriteAndWaitN(s) => {
                    s.decode_vgm_bytes(&mut cmd_buf)
                }
                VgmCommand::SetupStreamControl(s) => s.decode_vgm_bytes(&mut cmd_buf),
                VgmCommand::SetStreamData(s) => s.decode_vgm_bytes(&mut cmd_buf),
                VgmCommand::SetStreamFrequency(s) => s.decode_vgm_bytes(&mut cmd_buf),
                VgmCommand::StartStream(s) => s.decode_vgm_bytes(&mut cmd_buf),
                VgmCommand::StopStream(s) => s.decode_vgm_bytes(&mut cmd_buf),
                VgmCommand::StartStreamFastCall(s) => s.decode_vgm_bytes(&mut cmd_buf),
                VgmCommand::SeekOffset(s) => s.decode_vgm_bytes(&mut cmd_buf),
                VgmCommand::Sn76489Write(id, s) => emit_chip(*id, s, &mut cmd_buf),
                VgmCommand::Ym2413Write(id, s) => emit_chip(*id, s, &mut cmd_buf),
                VgmCommand::Ym2612Write(id, s) => emit_chip(*id, s, &mut cmd_buf),
                VgmCommand::Ym2151Write(id, s) => emit_chip(*id, s, &mut cmd_buf),
                VgmCommand::SegaPcmWrite(id, s) => emit_chip(*id, s, &mut cmd_buf),
                VgmCommand::Rf5c68Write(id, s) => emit_chip(*id, s, &mut cmd_buf),
                VgmCommand::Ym2203Write(id, s) => emit_chip(*id, s, &mut cmd_buf),
                VgmCommand::Ym2608Write(id, s) => emit_chip(*id, s, &mut cmd_buf),
                VgmCommand::Ym2610bWrite(id, s) => emit_chip(*id, s, &mut cmd_buf),
                VgmCommand::Ym3812Write(id, s) => emit_chip(*id, s, &mut cmd_buf),
                VgmCommand::Ym3526Write(id, s) => emit_chip(*id, s, &mut cmd_buf),
                VgmCommand::Y8950Write(id, s) => emit_chip(*id, s, &mut cmd_buf),
                VgmCommand::Ymf262Write(id, s) => emit_chip(*id, s, &mut cmd_buf),
                VgmCommand::Ymf278bWrite(id, s) => emit_chip(*id, s, &mut cmd_buf),
                VgmCommand::Ymf271Write(id, s) => emit_chip(*id, s, &mut cmd_buf),
                VgmCommand::Scc1Write(id, s) => emit_chip(*id, s, &mut cmd_buf),
                VgmCommand::Ymz280bWrite(id, s) => emit_chip(*id, s, &mut cmd_buf),
                VgmCommand::Rf5c164Write(id, s) => emit_chip(*id, s, &mut cmd_buf),
                VgmCommand::PwmWrite(id, s) => emit_chip(*id, s, &mut cmd_buf),
                VgmCommand::Ay8910Write(id, s) => emit_chip(*id, s, &mut cmd_buf),
                VgmCommand::GbDmgWrite(id, s) => emit_chip(*id, s, &mut cmd_buf),
                VgmCommand::NesApuWrite(id, s) => emit_chip(*id, s, &mut cmd_buf),
                VgmCommand::MultiPcmWrite(id, s) => emit_chip(*id, s, &mut cmd_buf),
                VgmCommand::Upd7759Write(id, s) => emit_chip(*id, s, &mut cmd_buf),
                VgmCommand::Okim6258Write(id, s) => emit_chip(*id, s, &mut cmd_buf),
                VgmCommand::Okim6295Write(id, s) => emit_chip(*id, s, &mut cmd_buf),
                VgmCommand::K051649Write(id, s) => emit_chip(*id, s, &mut cmd_buf),
                VgmCommand::K054539Write(id, s) => emit_chip(*id, s, &mut cmd_buf),
                VgmCommand::Huc6280Write(id, s) => emit_chip(*id, s, &mut cmd_buf),
                VgmCommand::C140Write(id, s) => emit_chip(*id, s, &mut cmd_buf),
                VgmCommand::K053260Write(id, s) => emit_chip(*id, s, &mut cmd_buf),
                VgmCommand::PokeyWrite(id, s) => emit_chip(*id, s, &mut cmd_buf),
                VgmCommand::QsoundWrite(id, s) => emit_chip(*id, s, &mut cmd_buf),
                VgmCommand::ScspWrite(id, s) => emit_chip(*id, s, &mut cmd_buf),
                VgmCommand::WonderSwanWrite(id, s) => emit_chip(*id, s, &mut cmd_buf),
                VgmCommand::VsuWrite(id, s) => emit_chip(*id, s, &mut cmd_buf),
                VgmCommand::Saa1099Write(id, s) => emit_chip(*id, s, &mut cmd_buf),
                VgmCommand::Es5503Write(id, s) => emit_chip(*id, s, &mut cmd_buf),
                VgmCommand::Es5506v8Write(id, s) => emit_chip(*id, s, &mut cmd_buf),
                VgmCommand::Es5506v16Write(id, s) => emit_chip(*id, s, &mut cmd_buf),
                VgmCommand::X1010Write(id, s) => emit_chip(*id, s, &mut cmd_buf),
                VgmCommand::C352Write(id, s) => emit_chip(*id, s, &mut cmd_buf),
                VgmCommand::Ga20Write(id, s) => emit_chip(*id, s, &mut cmd_buf),
                VgmCommand::MikeyWrite(id, s) => emit_chip(*id, s, &mut cmd_buf),
                VgmCommand::GameGearPsgWrite(id, s) => emit_chip(*id, s, &mut cmd_buf),
            }
        }

        let wrote_end_in_cmds = self
            .commands
            .iter()
            .any(|c| matches!(c, VgmCommand::EndOfData(_)));

        // GD3 offset: if present, it will be placed after header+cmd_buf
        let gd3_offset: u32 = match &self.gd3 {
            Some(_) => VGM_V171_HEADER_SIZE
                .wrapping_add(cmd_buf.len() as u32)
                .wrapping_sub(0x14),
            None => 0,
        };
        // data offset (0x34)
        let data_offset: u32 = match self.header.data_offset {
            0 => VGM_V171_HEADER_SIZE.wrapping_sub(0x34),
            v => v,
        };

        // Build header bytes using VgmHeader::to_bytes
        let mut buf = self.header.to_bytes(gd3_offset, data_offset);

        buf.extend_from_slice(&cmd_buf);
        if !wrote_end_in_cmds {
            let end_spec = EndOfData;
            buf.push(end_spec.opcode());
        }

        // If GD3 metadata is present, append the full GD3 chunk and update
        // the header's GD3 offset field to point to its location.
        if let Some(gd3) = &self.gd3 {
            let gd3_start = buf.len() as u32;
            let gd3_offset_val = gd3_start.wrapping_sub(0x14u32);
            let gd3_bytes = gd3.to_bytes();
            buf.extend_from_slice(&gd3_bytes);
            let gd3_off_bytes = gd3_offset_val.to_le_bytes();
            buf[0x14..0x18].copy_from_slice(&gd3_off_bytes);
        }

        let file_size = buf.len() as u32;
        let eof_offset = file_size.wrapping_sub(4);
        let eof_bytes = eof_offset.to_le_bytes();
        buf[0x04..0x08].copy_from_slice(&eof_bytes);

        buf
    }
}

impl VgmHeader {
    pub fn to_bytes(&self, gd3_offset: u32, data_offset: u32) -> Vec<u8> {
        let mut buf: Vec<u8> = vec![0; VGM_V171_HEADER_SIZE as usize];

        fn write_u32(buf: &mut [u8], off: usize, v: u32) {
            let bytes = v.to_le_bytes();
            buf[off..off + 4].copy_from_slice(&bytes);
        }
        fn write_u16(buf: &mut [u8], off: usize, v: u16) {
            let bytes = v.to_le_bytes();
            buf[off..off + 2].copy_from_slice(&bytes);
        }
        fn write_u8(buf: &mut [u8], off: usize, v: u8) {
            buf[off] = v;
        }
        fn write_slice(buf: &mut [u8], off: usize, s: &[u8]) {
            buf[off..off + s.len()].copy_from_slice(s);
        }
        // ident (0x00)
        write_slice(&mut buf, 0x00, &self.ident);
        // eof_offset placeholder (0x04)
        write_u32(&mut buf, 0x04, 0);
        // version (0x08)
        write_u32(&mut buf, 0x08, self.version);
        // SN76489 clock (0x0C)
        write_u32(&mut buf, 0x0C, self.sn76489_clock);
        // YM2413 clock (0x10)
        write_u32(&mut buf, 0x10, self.ym2413_clock);
        // GD3 offset (0x14)
        write_u32(&mut buf, 0x14, gd3_offset);
        // total samples (0x18)
        write_u32(&mut buf, 0x18, self.total_samples);
        // loop offset (0x1C)
        write_u32(&mut buf, 0x1C, self.loop_offset);
        // loop samples (0x20)
        write_u32(&mut buf, 0x20, self.loop_samples);
        // sample rate (0x24)
        write_u32(&mut buf, 0x24, self.sample_rate);
        // SN FB (0x28) u16
        write_u16(&mut buf, 0x28, self.sn_fb);
        // SNW (0x2A) u8
        write_u8(&mut buf, 0x2A, self.snw);
        // SF (0x2B) u8
        write_u8(&mut buf, 0x2B, self.sf);
        // YM2612 clock (0x2C)
        write_u32(&mut buf, 0x2C, self.ym2612_clock);
        // YM2151 clock (0x30)
        write_u32(&mut buf, 0x30, self.ym2151_clock);
        // data offset (0x34)
        write_u32(&mut buf, 0x34, data_offset);
        // SegaPCM clock (0x38)
        write_u32(&mut buf, 0x38, self.sega_pcm_clock);
        // SPCM interface (0x3C)
        write_u32(&mut buf, 0x3C, self.spcm_interface);
        // RF5C68 (0x40)
        write_u32(&mut buf, 0x40, self.rf5c68_clock);
        // YM2203 (0x44)
        write_u32(&mut buf, 0x44, self.ym2203_clock);
        // YM2608 (0x48)
        write_u32(&mut buf, 0x48, self.ym2608_clock);
        // YM2610/B (0x4C)
        write_u32(&mut buf, 0x4C, self.ym2610b_clock);
        // YM3812 (0x50)
        write_u32(&mut buf, 0x50, self.ym3812_clock);
        // YM3526 (0x54)
        write_u32(&mut buf, 0x54, self.ym3526_clock);
        // Y8950 (0x58)
        write_u32(&mut buf, 0x58, self.y8950_clock);
        // YMF262 (0x5C)
        write_u32(&mut buf, 0x5C, self.ymf262_clock);
        // YMF278B (0x60)
        write_u32(&mut buf, 0x60, self.ymf278b_clock);
        // YMF271 (0x64)
        write_u32(&mut buf, 0x64, self.ymf271_clock);
        // YMZ280B (0x68)
        write_u32(&mut buf, 0x68, self.ymz280b_clock);
        // RF5C164 (0x6C)
        write_u32(&mut buf, 0x6C, self.rf5c164_clock);
        // PWM (0x70)
        write_u32(&mut buf, 0x70, self.pwm_clock);
        // AY8910 (0x74)
        write_u32(&mut buf, 0x74, self.ay8910_clock);
        // AY misc (0x78..0x7F)
        write_slice(&mut buf, 0x78, &self.ay_misc);
        // GB DMG (0x80)
        write_u32(&mut buf, 0x80, self.gb_dmg_clock);
        // NES APU (0x84)
        write_u32(&mut buf, 0x84, self.nes_apu_clock);
        // MultiPCM (0x88)
        write_u32(&mut buf, 0x88, self.multipcm_clock);
        // uPD7759 (0x8C)
        write_u32(&mut buf, 0x8C, self.upd7759_clock);
        // OKIM6258 (0x90)
        write_u32(&mut buf, 0x90, self.okim6258_clock);
        // OKIM6258 flags (0x94..0x97)
        write_slice(&mut buf, 0x94, &self.okim6258_flags);
        // OKIM6295 (0x98)
        write_u32(&mut buf, 0x98, self.okim6295_clock);
        // K051649 (0x9C)
        write_u32(&mut buf, 0x9C, self.k051649_clock);
        // K054539 (0xA0)
        write_u32(&mut buf, 0xA0, self.k054539_clock);
        // HuC6280 (0xA4)
        write_u32(&mut buf, 0xA4, self.huc6280_clock);
        // C140 (0xA8)
        write_u32(&mut buf, 0xA8, self.c140_clock);
        // K053260 (0xAC)
        write_u32(&mut buf, 0xAC, self.k053260_clock);
        // Pokey (0xB0)
        write_u32(&mut buf, 0xB0, self.pokey_clock);
        // QSound (0xB4)
        write_u32(&mut buf, 0xB4, self.qsound_clock);
        // SCSP (0xB8)
        write_u32(&mut buf, 0xB8, self.scsp_clock);
        // Extra header offset (0xBC)
        write_u32(&mut buf, 0xBC, self.extra_header_offset);
        // WonderSwan (0xC0)
        write_u32(&mut buf, 0xC0, self.wonderswan_clock);
        // VSU (0xC4)
        write_u32(&mut buf, 0xC4, self.vsu_clock);
        // SAA1099 (0xC8)
        write_u32(&mut buf, 0xC8, self.saa1099_clock);
        // ES5503 (0xCC)
        write_u32(&mut buf, 0xCC, self.es5503_clock);
        // ES5506 (0xD0)
        write_u32(&mut buf, 0xD0, self.es5506_clock);
        write_u16(&mut buf, 0xD4, self.es5506_channels);
        write_u8(&mut buf, 0xD6, self.es5506_cd);
        write_u8(&mut buf, 0xD7, self.es5506_reserved);
        // X1-010 (0xD8)
        write_u32(&mut buf, 0xD8, self.x1_010_clock);
        // C352 (0xDC)
        write_u32(&mut buf, 0xDC, self.c352_clock);
        // GA20 (0xE0)
        write_u32(&mut buf, 0xE0, self.ga20_clock);
        // Mikey (0xE4)
        write_u32(&mut buf, 0xE4, self.mikey_clock);
        // reserved (0xE8..0xEF)
        write_slice(&mut buf, 0xE8, &self.reserved_e8_ef);
        // reserved (0xF0..0xFF)
        write_slice(&mut buf, 0xF0, &self.reserved_f0_ff);
        // truncate header buffer sized to `0x34 + data_offset`.
        let header_size = 0x34u32.wrapping_add(data_offset) as usize;
        if header_size < buf.len() {
            buf.truncate(header_size);
        }

        buf
    }
}
