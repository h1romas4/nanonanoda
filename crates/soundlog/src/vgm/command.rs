use crate::binutil::{
    ParseError, read_i32_le_at, read_slice, read_u8_at, read_u24_be_at, read_u32_le_at,
};
use crate::chip;
use crate::vgm::doc::{VgmBuilder, VgmDocument};
use crate::vgm::header::VGM_V171_HEADER_SIZE;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

pub trait CommandSpec {
    fn opcode(&self) -> u8;
    fn to_vgm_bytes(&self, dest: &mut Vec<u8>);
    fn parse(bytes: &[u8], offset: usize, opcode: u8) -> Result<(Self, usize), ParseError>
    where
        Self: Sized;
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
    pub read_offset: u32,
    pub write_offset: u32,
    pub size: u32,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WaitNSample(pub u8);

#[derive(Debug, Clone, PartialEq)]
pub struct Ym2612Port0Address2AWriteAndWaitN(pub u8);

#[derive(Debug, Clone, PartialEq)]
pub struct SetupStreamControl {
    pub stream_id: u8,
    pub chip_type: u8,
    pub write_port: u8,
    pub write_command: u8,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SetStreamData {
    pub stream_id: u8,
    pub data_bank_id: u8,
    pub step_size: u8,
    pub step_base: u8,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SetStreamFrequency {
    pub stream_id: u8,
    pub frequency: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StartStream {
    pub stream_id: u8,
    pub data_start_offset: i32,
    pub length_mode: u8,
    pub data_length: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StopStream {
    pub stream_id: u8,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StartStreamFastCall {
    pub stream_id: u8,
    pub block_id: u16,
    pub flags: u8,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SeekOffset(pub u32);

impl CommandSpec for Ay8910StereoMask {
    fn opcode(&self) -> u8 {
        0x31
    }
    fn to_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
        dest.push(self.0);
    }
    fn parse(bytes: &[u8], offset: usize, _opcode: u8) -> Result<(Self, usize), ParseError>
    where
        Self: Sized,
    {
        let v = read_u8_at(bytes, offset)?;
        Ok((Ay8910StereoMask(v), 1))
    }
}

impl CommandSpec for WaitSamples {
    fn opcode(&self) -> u8 {
        0x61
    }
    fn to_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
        dest.push((self.0 & 0xFF) as u8);
        dest.push((self.0 >> 8) as u8);
    }
    fn parse(bytes: &[u8], offset: usize, _opcode: u8) -> Result<(Self, usize), ParseError>
    where
        Self: Sized,
    {
        let lo = read_u8_at(bytes, offset)?;
        let hi = read_u8_at(bytes, offset + 1)?;
        let val = ((hi as u16) << 8) | (lo as u16);
        Ok((WaitSamples(val), 2))
    }
}

impl CommandSpec for Wait735Samples {
    fn opcode(&self) -> u8 {
        0x62
    }
    fn to_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
    }
    fn parse(_bytes: &[u8], _offset: usize, _opcode: u8) -> Result<(Self, usize), ParseError>
    where
        Self: Sized,
    {
        Ok((Wait735Samples, 0))
    }
}

impl CommandSpec for Wait882Samples {
    fn opcode(&self) -> u8 {
        0x63
    }
    fn to_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
    }
    fn parse(_bytes: &[u8], _offset: usize, _opcode: u8) -> Result<(Self, usize), ParseError>
    where
        Self: Sized,
    {
        Ok((Wait882Samples, 0))
    }
}

impl CommandSpec for EndOfData {
    fn opcode(&self) -> u8 {
        0x66
    }
    fn to_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
    }
    fn parse(_bytes: &[u8], _offset: usize, _opcode: u8) -> Result<(Self, usize), ParseError>
    where
        Self: Sized,
    {
        Ok((EndOfData, 0))
    }
}

impl CommandSpec for DataBlock {
    fn opcode(&self) -> u8 {
        0x67
    }
    fn to_vgm_bytes(&self, dest: &mut Vec<u8>) {
        // 0x67 0x66 tt ss ss ss ss data...
        dest.push(self.opcode());
        dest.push(0x66);
        dest.push(self.data_type);
        dest.extend_from_slice(&self.size.to_le_bytes());
        dest.extend_from_slice(&self.data);
    }
    fn parse(bytes: &[u8], offset: usize, _opcode: u8) -> Result<(Self, usize), ParseError>
    where
        Self: Sized,
    {
        let data_type = read_u8_at(bytes, offset)?;
        let size = read_u32_le_at(bytes, offset + 1)?;
        let data_slice = read_slice(bytes, offset + 5, size as usize)?;
        Ok((
            DataBlock {
                data_type,
                size,
                data: data_slice.to_vec(),
            },
            1 + 4 + size as usize,
        ))
    }
}

impl CommandSpec for PcmRamWrite {
    fn opcode(&self) -> u8 {
        0x68
    }
    fn to_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
        dest.push(0x66);
        dest.push(self.chip_type);
        let o = self.read_offset & 0x00FF_FFFF;
        dest.push(((o >> 16) & 0xFF) as u8);
        dest.push(((o >> 8) & 0xFF) as u8);
        dest.push((o & 0xFF) as u8);
        let w = self.write_offset & 0x00FF_FFFF;
        dest.push(((w >> 16) & 0xFF) as u8);
        dest.push(((w >> 8) & 0xFF) as u8);
        dest.push((w & 0xFF) as u8);
        let s = self.size & 0x00FF_FFFF;
        dest.push(((s >> 16) & 0xFF) as u8);
        dest.push(((s >> 8) & 0xFF) as u8);
        dest.push((s & 0xFF) as u8);
        dest.extend_from_slice(&self.data);
    }
    fn parse(bytes: &[u8], offset: usize, _opcode: u8) -> Result<(Self, usize), ParseError>
    where
        Self: Sized,
    {
        let chip_type = read_u8_at(bytes, offset)?;
        let read_off = read_u24_be_at(bytes, offset + 1)?;
        let write_off = read_u24_be_at(bytes, offset + 4)?;
        let size = read_u24_be_at(bytes, offset + 7)?;
        let data_slice = read_slice(bytes, offset + 10, size as usize)?;
        Ok((
            PcmRamWrite {
                chip_type,
                read_offset: read_off,
                write_offset: write_off,
                size,
                data: data_slice.to_vec(),
            },
            1 + 3 + 3 + 3 + size as usize,
        ))
    }
}

impl CommandSpec for WaitNSample {
    fn opcode(&self) -> u8 {
        0x70u8.wrapping_add(self.0.saturating_sub(1))
    }
    fn to_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
    }
    fn parse(_bytes: &[u8], _off: usize, opcode: u8) -> Result<(Self, usize), ParseError>
    where
        Self: Sized,
    {
        Ok((WaitNSample(opcode - 0x70 + 1), 0))
    }
}

impl CommandSpec for Ym2612Port0Address2AWriteAndWaitN {
    fn opcode(&self) -> u8 {
        0x80u8.wrapping_add(self.0)
    }
    fn to_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
    }
    fn parse(_bytes: &[u8], _off: usize, opcode: u8) -> Result<(Self, usize), ParseError>
    where
        Self: Sized,
    {
        Ok((
            Ym2612Port0Address2AWriteAndWaitN(opcode.wrapping_sub(0x80)),
            0,
        ))
    }
}

impl CommandSpec for SetupStreamControl {
    fn opcode(&self) -> u8 {
        0x90
    }
    fn to_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
        dest.push(self.stream_id);
        dest.push(self.chip_type);
        dest.push(self.write_port);
        dest.push(self.write_command);
    }
    fn parse(bytes: &[u8], off: usize, _opcode: u8) -> Result<(Self, usize), ParseError>
    where
        Self: Sized,
    {
        let stream_id = read_u8_at(bytes, off)?;
        let chip_type = read_u8_at(bytes, off + 1)?;
        let write_port = read_u8_at(bytes, off + 2)?;
        let write_command = read_u8_at(bytes, off + 3)?;
        Ok((
            SetupStreamControl {
                stream_id,
                chip_type,
                write_port,
                write_command,
            },
            4,
        ))
    }
}

impl CommandSpec for SetStreamData {
    fn opcode(&self) -> u8 {
        0x91
    }
    fn to_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
        dest.push(self.stream_id);
        dest.push(self.data_bank_id);
        dest.push(self.step_size);
        dest.push(self.step_base);
    }
    fn parse(bytes: &[u8], off: usize, _opcode: u8) -> Result<(Self, usize), ParseError>
    where
        Self: Sized,
    {
        let stream_id = read_u8_at(bytes, off)?;
        let data_bank_id = read_u8_at(bytes, off + 1)?;
        let step_size = read_u8_at(bytes, off + 2)?;
        let step_base = read_u8_at(bytes, off + 3)?;
        Ok((
            SetStreamData {
                stream_id,
                data_bank_id,
                step_size,
                step_base,
            },
            4,
        ))
    }
}

impl CommandSpec for SetStreamFrequency {
    fn opcode(&self) -> u8 {
        0x92
    }
    fn to_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
        dest.push(self.stream_id);
        dest.extend_from_slice(&self.frequency.to_le_bytes());
    }
    fn parse(bytes: &[u8], off: usize, _opcode: u8) -> Result<(Self, usize), ParseError>
    where
        Self: Sized,
    {
        let stream_id = read_u8_at(bytes, off)?;
        let freq = read_u32_le_at(bytes, off + 1)?;
        Ok((
            SetStreamFrequency {
                stream_id,
                frequency: freq,
            },
            1 + 4,
        ))
    }
}

impl CommandSpec for StartStream {
    fn opcode(&self) -> u8 {
        0x93
    }
    fn to_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
        dest.push(self.stream_id);
        dest.extend_from_slice(&self.data_start_offset.to_le_bytes());
        dest.push(self.length_mode);
        dest.extend_from_slice(&self.data_length.to_le_bytes());
    }
    fn parse(bytes: &[u8], off: usize, _opcode: u8) -> Result<(Self, usize), ParseError>
    where
        Self: Sized,
    {
        let stream_id = read_u8_at(bytes, off)?;
        let start_offset = read_i32_le_at(bytes, off + 1)?;
        let length_mode = read_u8_at(bytes, off + 5)?;
        let data_length = read_u32_le_at(bytes, off + 6)?;
        Ok((
            StartStream {
                stream_id,
                data_start_offset: start_offset,
                length_mode,
                data_length,
            },
            1 + 4 + 1 + 4,
        ))
    }
}

impl CommandSpec for StopStream {
    fn opcode(&self) -> u8 {
        0x94
    }
    fn to_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
        dest.push(self.stream_id);
    }
    fn parse(bytes: &[u8], off: usize, _opcode: u8) -> Result<(Self, usize), ParseError>
    where
        Self: Sized,
    {
        let stream_id = read_u8_at(bytes, off)?;
        Ok((StopStream { stream_id }, 1))
    }
}

impl CommandSpec for StartStreamFastCall {
    fn opcode(&self) -> u8 {
        0x95
    }
    fn to_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
        dest.push(self.stream_id);
        dest.push((self.block_id >> 8) as u8);
        dest.push(self.block_id as u8);
        dest.push(self.flags);
    }
    fn parse(bytes: &[u8], off: usize, _opcode: u8) -> Result<(Self, usize), ParseError>
    where
        Self: Sized,
    {
        let stream_id = read_u8_at(bytes, off)?;
        let b0 = read_u8_at(bytes, off + 1)?;
        let b1 = read_u8_at(bytes, off + 2)?;
        let block_id = ((b0 as u16) << 8) | (b1 as u16);
        let flags = read_u8_at(bytes, off + 3)?;
        Ok((
            StartStreamFastCall {
                stream_id,
                block_id,
                flags,
            },
            4,
        ))
    }
}

impl CommandSpec for SeekOffset {
    fn opcode(&self) -> u8 {
        0xE0
    }
    fn to_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
        dest.extend_from_slice(&self.0.to_le_bytes());
    }
    fn parse(bytes: &[u8], off: usize, _opcode: u8) -> Result<(Self, usize), ParseError>
    where
        Self: Sized,
    {
        let v = read_u32_le_at(bytes, off)?;
        Ok((SeekOffset(v), 4))
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

impl CommandSpec for chip::PsgSpec {
    // PSG (SN76489/SN76496) write value dd
    fn opcode(&self) -> u8 {
        0x50
    }
    fn to_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
        dest.push(self.value);
    }
    fn parse(bytes: &[u8], offset: usize, _opcode: u8) -> Result<(Self, usize), ParseError> {
        let val = read_u8_at(bytes, offset)?;
        Ok((chip::PsgSpec { value: val }, 1))
    }
}

impl CommandSpec for chip::Ym2413Spec {
    // YM2413, write value dd to register aa
    fn opcode(&self) -> u8 {
        0x51
    }
    fn to_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
        dest.push(self.register);
        dest.push(self.value);
    }
    fn parse(bytes: &[u8], off: usize, _opcode: u8) -> Result<(Self, usize), ParseError> {
        let reg = read_u8_at(bytes, off)?;
        let val = read_u8_at(bytes, off + 1)?;
        Ok((
            chip::Ym2413Spec {
                register: reg,
                value: val,
            },
            2,
        ))
    }
}

impl CommandSpec for chip::Ym2612Spec {
    // YM2612 port 0, write value dd to register aa
    // YM2612 port 1, write value dd to register aa
    fn opcode(&self) -> u8 {
        if self.port == 0 { 0x52 } else { 0x53 }
    }
    fn to_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
        dest.push(self.register);
        dest.push(self.value);
    }
    fn parse(bytes: &[u8], off: usize, opcode: u8) -> Result<(Self, usize), ParseError>
    where
        Self: Sized,
    {
        let port = if opcode == 0x52 { 0 } else { 1 };
        let reg = read_u8_at(bytes, off)?;
        let val = read_u8_at(bytes, off + 1)?;
        Ok((
            chip::Ym2612Spec {
                port,
                register: reg,
                value: val,
            },
            2,
        ))
    }
}

impl CommandSpec for chip::Ym2151Spec {
    // YM2151, write value dd to register aa
    fn opcode(&self) -> u8 {
        0x54u8
    }
    fn to_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
        dest.push(self.register);
        dest.push(self.value);
    }
    fn parse(bytes: &[u8], off: usize, _opcode: u8) -> Result<(Self, usize), ParseError> {
        let reg = read_u8_at(bytes, off)?;
        let val = read_u8_at(bytes, off + 1)?;
        Ok((
            chip::Ym2151Spec {
                register: reg,
                value: val,
            },
            2,
        ))
    }
}

impl CommandSpec for chip::SegaPcmSpec {
    // SegaPCM, write value dd to memory offset aabb
    fn opcode(&self) -> u8 {
        0xC0
    }
    fn to_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
        dest.push((self.offset >> 8) as u8);
        dest.push(self.offset as u8);
        dest.push(self.value);
    }
    fn parse(bytes: &[u8], off: usize, _opcode: u8) -> Result<(Self, usize), ParseError> {
        let off_hi = read_u8_at(bytes, off)?;
        let off_lo = read_u8_at(bytes, off + 1)?;
        let offv = ((off_hi as u16) << 8) | (off_lo as u16);
        let val = read_u8_at(bytes, off + 2)?;
        Ok((
            chip::SegaPcmSpec {
                offset: offv,
                value: val,
            },
            3,
        ))
    }
}

impl CommandSpec for chip::Rf5c68Spec {
    // RF5C68, write value dd to memory offset aabb
    fn opcode(&self) -> u8 {
        0xC1
    }
    fn to_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
        dest.push((self.offset >> 8) as u8);
        dest.push(self.offset as u8);
        dest.push(self.value);
    }
    fn parse(bytes: &[u8], off: usize, _opcode: u8) -> Result<(Self, usize), ParseError> {
        let off_hi = read_u8_at(bytes, off)?;
        let off_lo = read_u8_at(bytes, off + 1)?;
        let offv = ((off_hi as u16) << 8) | (off_lo as u16);
        let val = read_u8_at(bytes, off + 2)?;
        Ok((
            chip::Rf5c68Spec {
                offset: offv,
                value: val,
            },
            3,
        ))
    }
}

impl CommandSpec for chip::Ym2203Spec {
    // YM2203, write value dd to register aa
    fn opcode(&self) -> u8 {
        0x55
    }
    fn to_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
        dest.push(self.register);
        dest.push(self.value);
    }
    fn parse(bytes: &[u8], off: usize, _opcode: u8) -> Result<(Self, usize), ParseError> {
        let reg = read_u8_at(bytes, off)?;
        let val = read_u8_at(bytes, off + 1)?;
        Ok((
            chip::Ym2203Spec {
                register: reg,
                value: val,
            },
            2,
        ))
    }
}

impl CommandSpec for chip::Ym2608Spec {
    // YM2608 port 0, write value dd to register aa
    // YM2608 port 1, write value dd to register aa
    fn opcode(&self) -> u8 {
        if self.port == 0 { 0x56 } else { 0x57 }
    }
    fn to_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
        dest.push(self.register);
        dest.push(self.value);
    }
    fn parse(bytes: &[u8], off: usize, opcode: u8) -> Result<(Self, usize), ParseError> {
        let port = if opcode == 0x56 { 0 } else { 1 };
        let reg = read_u8_at(bytes, off)?;
        let val = read_u8_at(bytes, off + 1)?;
        Ok((
            chip::Ym2608Spec {
                port,
                register: reg,
                value: val,
            },
            2,
        ))
    }
}

impl CommandSpec for chip::Ym2610Spec {
    // YM2610 port 0, write value dd to register aa
    // YM2610 port 1, write value dd to register aa
    fn opcode(&self) -> u8 {
        if self.port == 0 { 0x58 } else { 0x59 }
    }
    fn to_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
        dest.push(self.register);
        dest.push(self.value);
    }
    fn parse(bytes: &[u8], off: usize, opcode: u8) -> Result<(Self, usize), ParseError> {
        let port = if opcode == 0x58 { 0 } else { 1 };
        let reg = read_u8_at(bytes, off)?;
        let val = read_u8_at(bytes, off + 1)?;
        Ok((
            chip::Ym2610Spec {
                port,
                register: reg,
                value: val,
            },
            2,
        ))
    }
}

impl CommandSpec for chip::Ym3812Spec {
    // YM3812, write value dd to register aa
    fn opcode(&self) -> u8 {
        0x5A
    }
    fn to_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
        dest.push(self.register);
        dest.push(self.value);
    }
    fn parse(bytes: &[u8], off: usize, _opcode: u8) -> Result<(Self, usize), ParseError> {
        let reg = read_u8_at(bytes, off)?;
        let val = read_u8_at(bytes, off + 1)?;
        Ok((
            chip::Ym3812Spec {
                register: reg,
                value: val,
            },
            2,
        ))
    }
}

impl CommandSpec for chip::Ym3526Spec {
    // YM3526, write value dd to register aa
    fn opcode(&self) -> u8 {
        0x5B
    }
    fn to_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
        dest.push(self.register);
        dest.push(self.value);
    }
    fn parse(bytes: &[u8], off: usize, _opcode: u8) -> Result<(Self, usize), ParseError> {
        let reg = read_u8_at(bytes, off)?;
        let val = read_u8_at(bytes, off + 1)?;
        Ok((
            chip::Ym3526Spec {
                register: reg,
                value: val,
            },
            2,
        ))
    }
}

impl CommandSpec for chip::Y8950Spec {
    // Y8950, write value dd to register aa
    fn opcode(&self) -> u8 {
        0x5C
    }
    fn to_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
        dest.push(self.register);
        dest.push(self.value);
    }
    fn parse(bytes: &[u8], off: usize, _opcode: u8) -> Result<(Self, usize), ParseError> {
        let reg = read_u8_at(bytes, off)?;
        let val = read_u8_at(bytes, off + 1)?;
        Ok((
            chip::Y8950Spec {
                register: reg,
                value: val,
            },
            2,
        ))
    }
}

impl CommandSpec for chip::Ymf262Spec {
    // YMF262 port 0, write value dd to register aa
    // YMF262 port 1, write value dd to register aa
    fn opcode(&self) -> u8 {
        if self.port == 0 { 0x5E } else { 0x5F }
    }
    fn to_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
        dest.push(self.register);
        dest.push(self.value);
    }
    fn parse(bytes: &[u8], off: usize, opcode: u8) -> Result<(Self, usize), ParseError> {
        let port = if opcode == 0x5E { 0 } else { 1 };
        let reg = read_u8_at(bytes, off)?;
        let val = read_u8_at(bytes, off + 1)?;
        Ok((
            chip::Ymf262Spec {
                port,
                register: reg,
                value: val,
            },
            2,
        ))
    }
}

impl CommandSpec for chip::Ymf278bSpec {
    // YMF278B, port pp, write value dd to register aa
    fn opcode(&self) -> u8 {
        0xD0
    }
    fn to_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
        dest.push(self.port);
        dest.push(self.register);
        dest.push(self.value);
    }
    fn parse(bytes: &[u8], off: usize, _opcode: u8) -> Result<(Self, usize), ParseError> {
        let port = read_u8_at(bytes, off)?;
        let reg = read_u8_at(bytes, off + 1)?;
        let val = read_u8_at(bytes, off + 2)?;
        Ok((
            chip::Ymf278bSpec {
                port,
                register: reg,
                value: val,
            },
            3,
        ))
    }
}

impl CommandSpec for chip::Ymf271Spec {
    // YMF271, port pp, write value dd to register aa
    fn opcode(&self) -> u8 {
        0xD1
    }
    fn to_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
        dest.push(self.port);
        dest.push(self.register);
        dest.push(self.value);
    }
    fn parse(bytes: &[u8], off: usize, _opcode: u8) -> Result<(Self, usize), ParseError> {
        let port = read_u8_at(bytes, off)?;
        let reg = read_u8_at(bytes, off + 1)?;
        let val = read_u8_at(bytes, off + 2)?;
        Ok((
            chip::Ymf271Spec {
                port,
                register: reg,
                value: val,
            },
            3,
        ))
    }
}

impl CommandSpec for chip::Scc1Spec {
    // SCC1, port pp, write value dd to register aa
    fn opcode(&self) -> u8 {
        0xD2
    }
    fn to_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
        dest.push(self.port);
        dest.push(self.register);
        dest.push(self.value);
    }
    fn parse(bytes: &[u8], off: usize, _opcode: u8) -> Result<(Self, usize), ParseError> {
        let port = read_u8_at(bytes, off)?;
        let reg = read_u8_at(bytes, off + 1)?;
        let val = read_u8_at(bytes, off + 2)?;
        Ok((
            chip::Scc1Spec {
                port,
                register: reg,
                value: val,
            },
            3,
        ))
    }
}

impl CommandSpec for chip::Ymz280bSpec {
    // YMZ280B, write value dd to register aa
    fn opcode(&self) -> u8 {
        0x5D
    }
    fn to_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
        dest.push(self.register);
        dest.push(self.value);
    }
    fn parse(bytes: &[u8], off: usize, _opcode: u8) -> Result<(Self, usize), ParseError> {
        let reg = read_u8_at(bytes, off)?;
        let val = read_u8_at(bytes, off + 1)?;
        Ok((
            chip::Ymz280bSpec {
                register: reg,
                value: val,
            },
            2,
        ))
    }
}

impl CommandSpec for chip::Rf5c164Spec {
    // RF5C164, write value dd to register aa
    fn opcode(&self) -> u8 {
        0xB1
    }
    fn to_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
        dest.push(self.register);
        dest.push(self.value);
    }
    fn parse(bytes: &[u8], off: usize, _opcode: u8) -> Result<(Self, usize), ParseError> {
        let reg = read_u8_at(bytes, off)?;
        let val = read_u8_at(bytes, off + 1)?;
        Ok((
            chip::Rf5c164Spec {
                register: reg,
                value: val,
            },
            2,
        ))
    }
}

impl CommandSpec for chip::PwmSpec {
    // PWM, write value ddd to register a (d is MSB, dd is LSB)
    fn opcode(&self) -> u8 {
        0xB2
    }
    fn to_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
        dest.push(self.register);
        let v = self.value & 0x00FF_FFFF;
        dest.push(((v >> 16) & 0xFF) as u8);
        dest.push(((v >> 8) & 0xFF) as u8);
        dest.push((v & 0xFF) as u8);
    }
    fn parse(bytes: &[u8], off: usize, _opcode: u8) -> Result<(Self, usize), ParseError> {
        let reg = read_u8_at(bytes, off)?;
        let v = read_u24_be_at(bytes, off + 1)?;
        Ok((
            chip::PwmSpec {
                register: reg,
                value: v,
            },
            4,
        ))
    }
}

impl CommandSpec for chip::Ay8910Spec {
    // AY8910, write value dd to register aa
    fn opcode(&self) -> u8 {
        0xA0
    }
    fn to_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
        dest.push(self.register);
        dest.push(self.value);
    }
    fn parse(bytes: &[u8], off: usize, _opcode: u8) -> Result<(Self, usize), ParseError> {
        let reg = read_u8_at(bytes, off)?;
        let val = read_u8_at(bytes, off + 1)?;
        Ok((
            chip::Ay8910Spec {
                register: reg,
                value: val,
            },
            2,
        ))
    }
}

impl CommandSpec for chip::GbDmgSpec {
    // GameBoy DMG, write value dd to register aa
    fn opcode(&self) -> u8 {
        0xB3
    }
    fn to_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
        dest.push(self.register);
        dest.push(self.value);
    }
    fn parse(bytes: &[u8], off: usize, _opcode: u8) -> Result<(Self, usize), ParseError> {
        let reg = read_u8_at(bytes, off)?;
        let val = read_u8_at(bytes, off + 1)?;
        Ok((
            chip::GbDmgSpec {
                register: reg,
                value: val,
            },
            2,
        ))
    }
}

impl CommandSpec for chip::NesApuSpec {
    // NES APU, write value dd to register aa
    fn opcode(&self) -> u8 {
        0xB4
    }
    fn to_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
        dest.push(self.register);
        dest.push(self.value);
    }
    fn parse(bytes: &[u8], off: usize, _opcode: u8) -> Result<(Self, usize), ParseError> {
        let reg = read_u8_at(bytes, off)?;
        let val = read_u8_at(bytes, off + 1)?;
        Ok((
            chip::NesApuSpec {
                register: reg,
                value: val,
            },
            2,
        ))
    }
}

impl CommandSpec for chip::MultiPcmSpec {
    // MultiPCM, write value dd to register aa
    fn opcode(&self) -> u8 {
        0xB5
    }
    fn to_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
        dest.push(self.register);
        dest.push(self.value);
    }
    fn parse(bytes: &[u8], off: usize, _opcode: u8) -> Result<(Self, usize), ParseError> {
        let reg = read_u8_at(bytes, off)?;
        let val = read_u8_at(bytes, off + 1)?;
        Ok((
            chip::MultiPcmSpec {
                register: reg,
                value: val,
            },
            2,
        ))
    }
}

impl CommandSpec for chip::Upd7759Spec {
    // uPD7759, write value dd to register aa
    fn opcode(&self) -> u8 {
        0xB6
    }
    fn to_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
        dest.push(self.register);
        dest.push(self.value);
    }
    fn parse(bytes: &[u8], off: usize, _opcode: u8) -> Result<(Self, usize), ParseError> {
        let reg = read_u8_at(bytes, off)?;
        let val = read_u8_at(bytes, off + 1)?;
        Ok((
            chip::Upd7759Spec {
                register: reg,
                value: val,
            },
            2,
        ))
    }
}

impl CommandSpec for chip::Okim6258Spec {
    // OKIM6258, write value dd to register aa
    fn opcode(&self) -> u8 {
        0xB7
    }
    fn to_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
        dest.push(self.register);
        dest.push(self.value);
    }
    fn parse(bytes: &[u8], off: usize, _opcode: u8) -> Result<(Self, usize), ParseError> {
        let reg = read_u8_at(bytes, off)?;
        let val = read_u8_at(bytes, off + 1)?;
        Ok((
            chip::Okim6258Spec {
                register: reg,
                value: val,
            },
            2,
        ))
    }
}

impl CommandSpec for chip::Okim6295Spec {
    // OKIM6295, write value dd to register aa
    fn opcode(&self) -> u8 {
        0xB8
    }
    fn to_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
        dest.push(self.register);
        dest.push(self.value);
    }
    fn parse(bytes: &[u8], off: usize, _opcode: u8) -> Result<(Self, usize), ParseError> {
        let reg = read_u8_at(bytes, off)?;
        let val = read_u8_at(bytes, off + 1)?;
        Ok((
            chip::Okim6295Spec {
                register: reg,
                value: val,
            },
            2,
        ))
    }
}

impl CommandSpec for chip::K051649Spec {
    // TODO: K051649, write value dd to register ppaa
    fn opcode(&self) -> u8 {
        0x00
    }
    fn to_vgm_bytes(&self, dest: &mut Vec<u8>) {
        let _ = dest;
        unimplemented!("chip::K051649Spec");
    }
    fn parse(_bytes: &[u8], _off: usize, _opcode: u8) -> Result<(Self, usize), ParseError>
    where
        Self: Sized,
    {
        Err(ParseError::Other("parse not implemented".into()))
    }
}

impl CommandSpec for chip::K054539Spec {
    // K054539, write value dd to register ppaa
    fn opcode(&self) -> u8 {
        0xD3
    }
    fn to_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
        dest.push((self.register >> 8) as u8);
        dest.push(self.register as u8);
        dest.push(self.value);
    }
    fn parse(bytes: &[u8], off: usize, _opcode: u8) -> Result<(Self, usize), ParseError>
    where
        Self: Sized,
    {
        let hi = read_u8_at(bytes, off)?;
        let lo = read_u8_at(bytes, off + 1)?;
        let reg = ((hi as u16) << 8) | (lo as u16);
        let val = read_u8_at(bytes, off + 2)?;
        Ok((
            chip::K054539Spec {
                register: reg,
                value: val,
            },
            3,
        ))
    }
}

impl CommandSpec for chip::Huc6280Spec {
    // HuC6280, write value dd to register aa
    fn opcode(&self) -> u8 {
        0xB9
    }
    fn to_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
        dest.push(self.register);
        dest.push(self.value);
    }
    fn parse(bytes: &[u8], off: usize, _opcode: u8) -> Result<(Self, usize), ParseError>
    where
        Self: Sized,
    {
        let reg = read_u8_at(bytes, off)?;
        let val = read_u8_at(bytes, off + 1)?;
        Ok((
            chip::Huc6280Spec {
                register: reg,
                value: val,
            },
            2,
        ))
    }
}

impl CommandSpec for chip::C140Spec {
    // C140, write value dd to register ppaa
    fn opcode(&self) -> u8 {
        0xD4
    }
    fn to_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
        dest.push((self.register >> 8) as u8);
        dest.push(self.register as u8);
        dest.push(self.value);
    }
    fn parse(bytes: &[u8], off: usize, _opcode: u8) -> Result<(Self, usize), ParseError>
    where
        Self: Sized,
    {
        let hi = read_u8_at(bytes, off)?;
        let lo = read_u8_at(bytes, off + 1)?;
        let reg = ((hi as u16) << 8) | (lo as u16);
        let val = read_u8_at(bytes, off + 2)?;
        Ok((
            chip::C140Spec {
                register: reg,
                value: val,
            },
            3,
        ))
    }
}

impl CommandSpec for chip::K053260Spec {
    // K053260, write value dd to register aa
    fn opcode(&self) -> u8 {
        0xBA
    }
    fn to_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
        dest.push(self.register);
        dest.push(self.value);
    }
    fn parse(bytes: &[u8], off: usize, _opcode: u8) -> Result<(Self, usize), ParseError>
    where
        Self: Sized,
    {
        let reg = read_u8_at(bytes, off)?;
        let val = read_u8_at(bytes, off + 1)?;
        Ok((
            chip::K053260Spec {
                register: reg,
                value: val,
            },
            2,
        ))
    }
}

impl CommandSpec for chip::PokeySpec {
    // Pokey, write value dd to register aa
    fn opcode(&self) -> u8 {
        0xBB
    }
    fn to_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
        dest.push(self.register);
        dest.push(self.value);
    }
    fn parse(bytes: &[u8], off: usize, _opcode: u8) -> Result<(Self, usize), ParseError>
    where
        Self: Sized,
    {
        let reg = read_u8_at(bytes, off)?;
        let val = read_u8_at(bytes, off + 1)?;
        Ok((
            chip::PokeySpec {
                register: reg,
                value: val,
            },
            2,
        ))
    }
}

impl CommandSpec for chip::QsoundSpec {
    // QSound, write value mmll to register rr (mm - data MSB, ll - data LSB)
    fn opcode(&self) -> u8 {
        0xC4
    }
    fn to_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
        dest.push(self.register);
        dest.push(((self.value >> 8) & 0xFF) as u8);
        dest.push((self.value & 0xFF) as u8);
    }
    fn parse(bytes: &[u8], off: usize, _opcode: u8) -> Result<(Self, usize), ParseError>
    where
        Self: Sized,
    {
        let reg = read_u8_at(bytes, off)?;
        let hi = read_u8_at(bytes, off + 1)?;
        let lo = read_u8_at(bytes, off + 2)?;
        let val = ((hi as u16) << 8) | (lo as u16);
        Ok((
            chip::QsoundSpec {
                register: reg,
                value: val,
            },
            3,
        ))
    }
}

impl CommandSpec for chip::ScspSpec {
    // SCSP, write value dd to memory offset mmll (mm - offset MSB, ll - offset LSB)
    fn opcode(&self) -> u8 {
        0xC5
    }
    fn to_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
        dest.push((self.offset >> 8) as u8);
        dest.push(self.offset as u8);
        dest.push(self.value);
    }
    fn parse(bytes: &[u8], off: usize, _opcode: u8) -> Result<(Self, usize), ParseError>
    where
        Self: Sized,
    {
        let hi = read_u8_at(bytes, off)?;
        let lo = read_u8_at(bytes, off + 1)?;
        let offv = ((hi as u16) << 8) | (lo as u16);
        let val = read_u8_at(bytes, off + 2)?;
        Ok((
            chip::ScspSpec {
                offset: offv,
                value: val,
            },
            3,
        ))
    }
}

impl CommandSpec for chip::WonderSwanSpec {
    // WonderSwan, write value dd to memory offset mmll (mm - offset MSB, ll - offset LSB)
    fn opcode(&self) -> u8 {
        0xC6
    }
    fn to_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
        dest.push((self.offset >> 8) as u8);
        dest.push(self.offset as u8);
        dest.push(self.value);
    }
    fn parse(bytes: &[u8], off: usize, _opcode: u8) -> Result<(Self, usize), ParseError>
    where
        Self: Sized,
    {
        let hi = read_u8_at(bytes, off)?;
        let lo = read_u8_at(bytes, off + 1)?;
        let offv = ((hi as u16) << 8) | (lo as u16);
        let val = read_u8_at(bytes, off + 2)?;
        Ok((
            chip::WonderSwanSpec {
                offset: offv,
                value: val,
            },
            3,
        ))
    }
}

impl CommandSpec for chip::VsuSpec {
    // VSU, write value dd to memory offset mmll (mm - offset MSB, ll - offset LSB)
    fn opcode(&self) -> u8 {
        0xC7
    }
    fn to_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
        dest.push((self.offset >> 8) as u8);
        dest.push(self.offset as u8);
        dest.push(self.value);
    }
    fn parse(bytes: &[u8], off: usize, _opcode: u8) -> Result<(Self, usize), ParseError>
    where
        Self: Sized,
    {
        let hi = read_u8_at(bytes, off)?;
        let lo = read_u8_at(bytes, off + 1)?;
        let offv = ((hi as u16) << 8) | (lo as u16);
        let val = read_u8_at(bytes, off + 2)?;
        Ok((
            chip::VsuSpec {
                offset: offv,
                value: val,
            },
            3,
        ))
    }
}

impl CommandSpec for chip::Saa1099Spec {
    // SAA1099, write value dd to register aa
    fn opcode(&self) -> u8 {
        0xBD
    }
    fn to_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
        dest.push(self.register);
        dest.push(self.value);
    }
    fn parse(bytes: &[u8], off: usize, _opcode: u8) -> Result<(Self, usize), ParseError>
    where
        Self: Sized,
    {
        let reg = read_u8_at(bytes, off)?;
        let val = read_u8_at(bytes, off + 1)?;
        Ok((
            chip::Saa1099Spec {
                register: reg,
                value: val,
            },
            2,
        ))
    }
}

impl CommandSpec for chip::Es5503Spec {
    // ES5503, write value dd to register ppaa
    fn opcode(&self) -> u8 {
        0xD5
    }
    fn to_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
        dest.push((self.register >> 8) as u8);
        dest.push(self.register as u8);
        dest.push(self.value);
    }
    fn parse(bytes: &[u8], off: usize, _opcode: u8) -> Result<(Self, usize), ParseError>
    where
        Self: Sized,
    {
        let hi = read_u8_at(bytes, off)?;
        let lo = read_u8_at(bytes, off + 1)?;
        let reg = ((hi as u16) << 8) | (lo as u16);
        let val = read_u8_at(bytes, off + 2)?;
        Ok((
            chip::Es5503Spec {
                register: reg,
                value: val,
            },
            3,
        ))
    }
}

impl CommandSpec for chip::Es5506v8Spec {
    // ES5506, write value dd to register aa
    //  Note: This command writes 8-bit data.
    fn opcode(&self) -> u8 {
        0xBE
    }
    fn to_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
        dest.push(self.register);
        dest.push(self.value);
    }
    fn parse(bytes: &[u8], off: usize, _opcode: u8) -> Result<(Self, usize), ParseError>
    where
        Self: Sized,
    {
        let reg = read_u8_at(bytes, off)?;
        let val = read_u8_at(bytes, off + 1)?;
        Ok((
            chip::Es5506v8Spec {
                register: reg,
                value: val,
            },
            2,
        ))
    }
}

impl CommandSpec for chip::Es5506v16Spec {
    // ES5506, write value aadd to register pp
    //  Note: This command writes 16-bit data.
    fn opcode(&self) -> u8 {
        0xD6
    }
    fn to_vgm_bytes(&self, dest: &mut Vec<u8>) {
        // TODO: Support 16-bit data write
        dest.push(self.opcode());
        dest.push(self.register);
        dest.push(((self.value >> 8) & 0xFF) as u8);
        dest.push((self.value & 0xFF) as u8);
    }
    fn parse(bytes: &[u8], off: usize, _opcode: u8) -> Result<(Self, usize), ParseError>
    where
        Self: Sized,
    {
        let reg = read_u8_at(bytes, off)?;
        let hi = read_u8_at(bytes, off + 1)?;
        let lo = read_u8_at(bytes, off + 2)?;
        let val = ((hi as u16) << 8) | (lo as u16);
        Ok((
            chip::Es5506v16Spec {
                register: reg,
                value: val,
            },
            3,
        ))
    }
}

impl CommandSpec for chip::X1010Spec {
    // X1-010, write value dd to memory offset mmll (mm - offset MSB, ll - offset LSB)
    fn opcode(&self) -> u8 {
        0xC8
    }
    fn to_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
        dest.push((self.offset >> 8) as u8);
        dest.push(self.offset as u8);
        dest.push(self.value);
    }
    fn parse(bytes: &[u8], off: usize, _opcode: u8) -> Result<(Self, usize), ParseError>
    where
        Self: Sized,
    {
        let hi = read_u8_at(bytes, off)?;
        let lo = read_u8_at(bytes, off + 1)?;
        let offv = ((hi as u16) << 8) | (lo as u16);
        let val = read_u8_at(bytes, off + 2)?;
        Ok((
            chip::X1010Spec {
                offset: offv,
                value: val,
            },
            3,
        ))
    }
}

impl CommandSpec for chip::C352Spec {
    // C352, write value aadd to register mmll
    fn opcode(&self) -> u8 {
        0xE1
    }
    fn to_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
        dest.push((self.register >> 8) as u8);
        dest.push(self.register as u8);
        dest.push(((self.value >> 8) & 0xFF) as u8);
        dest.push((self.value & 0xFF) as u8);
    }
    fn parse(bytes: &[u8], off: usize, _opcode: u8) -> Result<(Self, usize), ParseError>
    where
        Self: Sized,
    {
        let hi = read_u8_at(bytes, off)?;
        let lo = read_u8_at(bytes, off + 1)?;
        let reg = ((hi as u16) << 8) | (lo as u16);
        let vhi = read_u8_at(bytes, off + 2)?;
        let vlo = read_u8_at(bytes, off + 3)?;
        let val = ((vhi as u16) << 8) | (vlo as u16);
        Ok((
            chip::C352Spec {
                register: reg,
                value: val,
            },
            4,
        ))
    }
}

impl CommandSpec for chip::Ga20Spec {
    // GA20, write value dd to register aa
    fn opcode(&self) -> u8 {
        0xBF
    }
    fn to_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
        dest.push(self.register);
        dest.push(self.value);
    }
    fn parse(bytes: &[u8], off: usize, _opcode: u8) -> Result<(Self, usize), ParseError>
    where
        Self: Sized,
    {
        let reg = read_u8_at(bytes, off)?;
        let val = read_u8_at(bytes, off + 1)?;
        Ok((
            chip::Ga20Spec {
                register: reg,
                value: val,
            },
            2,
        ))
    }
}

impl CommandSpec for chip::MikeySpec {
    // Mikey, write value dd to register aa
    fn opcode(&self) -> u8 {
        0x40
    }
    fn to_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
        dest.push(self.register);
        dest.push(self.value);
    }
    fn parse(bytes: &[u8], off: usize, _opcode: u8) -> Result<(Self, usize), ParseError>
    where
        Self: Sized,
    {
        let reg = read_u8_at(bytes, off)?;
        let val = read_u8_at(bytes, off + 1)?;
        Ok((
            chip::MikeySpec {
                register: reg,
                value: val,
            },
            2,
        ))
    }
}

impl CommandSpec for chip::GameGearPsgSpec {
    // Game Gear PSG, write value dd
    fn opcode(&self) -> u8 {
        0x4F
    }
    fn to_vgm_bytes(&self, dest: &mut Vec<u8>) {
        dest.push(self.opcode());
        dest.push(self.value);
    }
    fn parse(bytes: &[u8], off: usize, _opcode: u8) -> Result<(Self, usize), ParseError>
    where
        Self: Sized,
    {
        let v = read_u8_at(bytes, off)?;
        Ok((chip::GameGearPsgSpec { value: v }, 1))
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

        fn emit_chip<C: CommandSpec + ?Sized>(id: ChipId, spec: &C, cmd_buf: &mut Vec<u8>) {
            let start = cmd_buf.len();
            spec.to_vgm_bytes(cmd_buf);
            cmd_buf[start] = adjust_opcode_for_chip_id(id, cmd_buf[start]);
        }

        let mut cmd_buf: Vec<u8> = Vec::new();

        for cmd in &self.commands {
            match cmd {
                VgmCommand::AY8910StereoMask(s) => s.to_vgm_bytes(&mut cmd_buf),
                VgmCommand::WaitSamples(s) => s.to_vgm_bytes(&mut cmd_buf),
                VgmCommand::Wait735Samples(s) => s.to_vgm_bytes(&mut cmd_buf),
                VgmCommand::Wait882Samples(s) => s.to_vgm_bytes(&mut cmd_buf),
                VgmCommand::EndOfData(s) => s.to_vgm_bytes(&mut cmd_buf),
                VgmCommand::DataBlock(s) => s.to_vgm_bytes(&mut cmd_buf),
                VgmCommand::PcmRamWrite(s) => s.to_vgm_bytes(&mut cmd_buf),
                VgmCommand::WaitNSample(s) => s.to_vgm_bytes(&mut cmd_buf),
                VgmCommand::YM2612Port0Address2AWriteAndWaitN(s) => s.to_vgm_bytes(&mut cmd_buf),
                VgmCommand::SetupStreamControl(s) => s.to_vgm_bytes(&mut cmd_buf),
                VgmCommand::SetStreamData(s) => s.to_vgm_bytes(&mut cmd_buf),
                VgmCommand::SetStreamFrequency(s) => s.to_vgm_bytes(&mut cmd_buf),
                VgmCommand::StartStream(s) => s.to_vgm_bytes(&mut cmd_buf),
                VgmCommand::StopStream(s) => s.to_vgm_bytes(&mut cmd_buf),
                VgmCommand::StartStreamFastCall(s) => s.to_vgm_bytes(&mut cmd_buf),
                VgmCommand::SeekOffset(s) => s.to_vgm_bytes(&mut cmd_buf),
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

// Parse error and byte-read helpers live in `binutils.rs`.
fn parse_chip_write(
    base: u8,
    instance: ChipId,
    bytes: &[u8],
    off: usize,
) -> Result<(VgmCommand, usize), ParseError> {
    // return a VgmCommand containing the chip write with `instance` and
    // number of bytes consumed from `off`.
    match base {
        0x50 => {
            let (spec, n) = chip::PsgSpec::parse(bytes, off, base)?;
            Ok((VgmCommand::Sn76489Write(instance, spec), n))
        }
        0x51 => {
            let (spec, n) = chip::Ym2413Spec::parse(bytes, off, base)?;
            Ok((VgmCommand::Ym2413Write(instance, spec), n))
        }
        0x52 | 0x53 => {
            let (spec, n) = chip::Ym2612Spec::parse(bytes, off, base)?;
            Ok((VgmCommand::Ym2612Write(instance, spec), n))
        }
        0x54 => {
            let (spec, n) = chip::Ym2151Spec::parse(bytes, off, base)?;
            Ok((VgmCommand::Ym2151Write(instance, spec), n))
        }
        0x55 => {
            let (spec, n) = chip::Ym2203Spec::parse(bytes, off, base)?;
            Ok((VgmCommand::Ym2203Write(instance, spec), n))
        }
        0x56 | 0x57 => {
            let (spec, n) = chip::Ym2608Spec::parse(bytes, off, base)?;
            Ok((VgmCommand::Ym2608Write(instance, spec), n))
        }
        0x58 | 0x59 => {
            let (spec, n) = chip::Ym2610Spec::parse(bytes, off, base)?;
            Ok((VgmCommand::Ym2610bWrite(instance, spec), n))
        }
        0x5A => {
            let (spec, n) = chip::Ym3812Spec::parse(bytes, off, base)?;
            Ok((VgmCommand::Ym3812Write(instance, spec), n))
        }
        0x5B => {
            let (spec, n) = chip::Ym3526Spec::parse(bytes, off, base)?;
            Ok((VgmCommand::Ym3526Write(instance, spec), n))
        }
        0x5C => {
            let (spec, n) = chip::Y8950Spec::parse(bytes, off, base)?;
            Ok((VgmCommand::Y8950Write(instance, spec), n))
        }
        0x5D => {
            let (spec, n) = chip::Ymz280bSpec::parse(bytes, off, base)?;
            Ok((VgmCommand::Ymz280bWrite(instance, spec), n))
        }
        0x5E | 0x5F => {
            let (spec, n) = chip::Ymf262Spec::parse(bytes, off, base)?;
            Ok((VgmCommand::Ymf262Write(instance, spec), n))
        }
        0xC0 => {
            let (spec, n) = chip::SegaPcmSpec::parse(bytes, off, base)?;
            Ok((VgmCommand::SegaPcmWrite(instance, spec), n))
        }
        0xC1 => {
            let (spec, n) = chip::Rf5c68Spec::parse(bytes, off, base)?;
            Ok((VgmCommand::Rf5c68Write(instance, spec), n))
        }
        0xB2 => {
            let (spec, n) = chip::PwmSpec::parse(bytes, off, base)?;
            Ok((VgmCommand::PwmWrite(instance, spec), n))
        }
        0xA0 => {
            let (spec, n) = chip::Ay8910Spec::parse(bytes, off, base)?;
            Ok((VgmCommand::Ay8910Write(instance, spec), n))
        }
        0xB3 => {
            let (spec, n) = chip::GbDmgSpec::parse(bytes, off, base)?;
            Ok((VgmCommand::GbDmgWrite(instance, spec), n))
        }
        0xB4 => {
            let (spec, n) = chip::NesApuSpec::parse(bytes, off, base)?;
            Ok((VgmCommand::NesApuWrite(instance, spec), n))
        }
        0xB5 => {
            let (spec, n) = chip::MultiPcmSpec::parse(bytes, off, base)?;
            Ok((VgmCommand::MultiPcmWrite(instance, spec), n))
        }
        0xB6 => {
            let (spec, n) = chip::Upd7759Spec::parse(bytes, off, base)?;
            Ok((VgmCommand::Upd7759Write(instance, spec), n))
        }
        0xB7 => {
            let (spec, n) = chip::Okim6258Spec::parse(bytes, off, base)?;
            Ok((VgmCommand::Okim6258Write(instance, spec), n))
        }
        0xB8 => {
            let (spec, n) = chip::Okim6295Spec::parse(bytes, off, base)?;
            Ok((VgmCommand::Okim6295Write(instance, spec), n))
        }
        0xD0 => {
            let (spec, n) = chip::Ymf278bSpec::parse(bytes, off, base)?;
            Ok((VgmCommand::Ymf278bWrite(instance, spec), n))
        }
        0xD1 => {
            let (spec, n) = chip::Ymf271Spec::parse(bytes, off, base)?;
            Ok((VgmCommand::Ymf271Write(instance, spec), n))
        }
        0xD2 => {
            let (spec, n) = chip::Scc1Spec::parse(bytes, off, base)?;
            Ok((VgmCommand::Scc1Write(instance, spec), n))
        }
        0xD3 => {
            let (spec, n) = chip::K054539Spec::parse(bytes, off, base)?;
            Ok((VgmCommand::K054539Write(instance, spec), n))
        }
        0xD4 => {
            let (spec, n) = chip::C140Spec::parse(bytes, off, base)?;
            Ok((VgmCommand::C140Write(instance, spec), n))
        }
        0xD5 => {
            let (spec, n) = chip::Es5503Spec::parse(bytes, off, base)?;
            Ok((VgmCommand::Es5503Write(instance, spec), n))
        }
        0xBE => {
            let (spec, n) = chip::Es5506v8Spec::parse(bytes, off, base)?;
            Ok((VgmCommand::Es5506v8Write(instance, spec), n))
        }
        0xD6 => {
            let (spec, n) = chip::Es5506v16Spec::parse(bytes, off, base)?;
            Ok((VgmCommand::Es5506v16Write(instance, spec), n))
        }
        0xC4 => {
            let (spec, n) = chip::QsoundSpec::parse(bytes, off, base)?;
            Ok((VgmCommand::QsoundWrite(instance, spec), n))
        }
        0xC5 => {
            let (spec, n) = chip::ScspSpec::parse(bytes, off, base)?;
            Ok((VgmCommand::ScspWrite(instance, spec), n))
        }
        0xC6 => {
            let (spec, n) = chip::WonderSwanSpec::parse(bytes, off, base)?;
            Ok((VgmCommand::WonderSwanWrite(instance, spec), n))
        }
        0xC7 => {
            let (spec, n) = chip::VsuSpec::parse(bytes, off, base)?;
            Ok((VgmCommand::VsuWrite(instance, spec), n))
        }
        0xC8 => {
            let (spec, n) = chip::X1010Spec::parse(bytes, off, base)?;
            Ok((VgmCommand::X1010Write(instance, spec), n))
        }
        0xE1 => {
            let (spec, n) = chip::C352Spec::parse(bytes, off, base)?;
            Ok((VgmCommand::C352Write(instance, spec), n))
        }
        0xBF => {
            let (spec, n) = chip::Ga20Spec::parse(bytes, off, base)?;
            Ok((VgmCommand::Ga20Write(instance, spec), n))
        }
        0x40 => {
            let (spec, n) = chip::MikeySpec::parse(bytes, off, base)?;
            Ok((VgmCommand::MikeyWrite(instance, spec), n))
        }
        0x4F => {
            let (spec, n) = chip::GameGearPsgSpec::parse(bytes, off, base)?;
            Ok((VgmCommand::GameGearPsgWrite(instance, spec), n))
        }
        _ => Err(ParseError::Other(format!(
            "unknown chip base opcode {:#X}",
            base
        ))),
    }
}

impl VgmCommand {
    /// Parse a single VGM command starting at `off` in `bytes`.
    /// Returns the parsed `VgmCommand` and the number of bytes consumed.
    pub fn from_bytes(bytes: &[u8], off: usize) -> Result<(VgmCommand, usize), ParseError> {
        let opcode = read_u8_at(bytes, off)?;
        let mut cur = off + 1;
        match opcode {
            0x31 => {
                let (v, n) = Ay8910StereoMask::parse(bytes, cur, opcode)?;
                Ok((VgmCommand::AY8910StereoMask(v), 1 + n))
            }
            0x61 => {
                let (v, n) = WaitSamples::parse(bytes, cur, opcode)?;
                Ok((VgmCommand::WaitSamples(v), 1 + n))
            }
            0x62 => {
                let (v, n) = Wait735Samples::parse(bytes, cur, opcode)?;
                Ok((VgmCommand::Wait735Samples(v), 1 + n))
            }
            0x63 => {
                let (v, n) = Wait882Samples::parse(bytes, cur, opcode)?;
                Ok((VgmCommand::Wait882Samples(v), 1 + n))
            }
            0x66 => {
                let (v, n) = EndOfData::parse(bytes, cur, opcode)?;
                Ok((VgmCommand::EndOfData(v), 1 + n))
            }
            0x67 => {
                // expect 0x66 next, then hand off to DataBlock::parse which expects
                // to be called at the byte after the marker.
                let marker = read_u8_at(bytes, cur)?;
                cur += 1;
                if marker != 0x66 {
                    return Err(ParseError::Other("invalid data block marker".into()));
                }
                let (db, n) = DataBlock::parse(bytes, cur, opcode)?;
                cur += n;
                Ok((VgmCommand::DataBlock(db), cur - off))
            }
            0x68 => {
                // expect 0x66 then chip_type then 3*24-bit fields + data
                let marker = read_u8_at(bytes, cur)?;
                cur += 1;
                if marker != 0x66 {
                    return Err(ParseError::Other("invalid pcm ram write marker".into()));
                }
                let (pr, n) = PcmRamWrite::parse(bytes, cur, opcode)?;
                cur += n;
                Ok((VgmCommand::PcmRamWrite(pr), cur - off))
            }
            0x70..=0x7F => {
                let (v, n) = WaitNSample::parse(bytes, cur, opcode)?;
                Ok((VgmCommand::WaitNSample(v), 1 + n))
            }
            0x80..=0x8F => {
                let (v, n) = Ym2612Port0Address2AWriteAndWaitN::parse(bytes, cur, opcode)?;
                Ok((VgmCommand::YM2612Port0Address2AWriteAndWaitN(v), 1 + n))
            }
            0x90 => {
                let (v, n) = SetupStreamControl::parse(bytes, cur, opcode)?;
                Ok((VgmCommand::SetupStreamControl(v), 1 + n))
            }
            0x91 => {
                let (v, n) = SetStreamData::parse(bytes, cur, opcode)?;
                Ok((VgmCommand::SetStreamData(v), 1 + n))
            }
            0x92 => {
                let (v, n) = SetStreamFrequency::parse(bytes, cur, opcode)?;
                Ok((VgmCommand::SetStreamFrequency(v), 1 + n))
            }
            0x93 => {
                let (v, n) = StartStream::parse(bytes, cur, opcode)?;
                Ok((VgmCommand::StartStream(v), 1 + n))
            }
            0x94 => {
                let (v, n) = StopStream::parse(bytes, cur, opcode)?;
                Ok((VgmCommand::StopStream(v), 1 + n))
            }
            0x95 => {
                let (v, n) = StartStreamFastCall::parse(bytes, cur, opcode)?;
                Ok((VgmCommand::StartStreamFastCall(v), 1 + n))
            }
            0xE0 => {
                let (v, n) = SeekOffset::parse(bytes, cur, opcode)?;
                Ok((VgmCommand::SeekOffset(v), 1 + n))
            }
            other => {
                // Try to parse as a chip write (primary or secondary instance).
                for &instance in &[ChipId::Primary, ChipId::Secondary] {
                    let base = match instance {
                        ChipId::Primary => other,
                        ChipId::Secondary => other.wrapping_sub(0x50),
                    };
                    match parse_chip_write(base, instance, bytes, cur) {
                        Ok((cmd, cons)) => return Ok((cmd, 1 + cons)),
                        Err(ParseError::Other(_)) => continue,
                        Err(e) => return Err(e),
                    }
                }

                Err(ParseError::Other(format!("unknown opcode {:#X}", other)))
            }
        }
    }
}
