use crate::binutil::{ParseError, read_slice, read_u8_at, read_u16_le_at, read_u32_le_at};
use crate::chip;
use crate::meta::parse_gd3;
use crate::vgm::command::{
    Ay8910StereoMask, CommandSpec, DataBlock, EndOfData, Instance, PcmRamWrite, ReservedU8,
    ReservedU16, ReservedU24, ReservedU32, SeekOffset, SetStreamData, SetStreamFrequency,
    SetupStreamControl, StartStream, StartStreamFastCall, StopStream, VgmCommand, Wait735Samples,
    Wait882Samples, WaitNSample, WaitSamples, Ym2612Port0Address2AWriteAndWaitN,
};
use crate::vgm::document::VgmDocument;
use crate::vgm::header::{VGM_V171_HEADER_SIZE, VgmExtraHeader, VgmHeader};

/// Parse a complete VGM file from a byte slice into a `VgmDocument`.
///
/// High-level parsing steps:
/// 1. Parse the VGM header with `parse_vgm_header`, which returns the
///    parsed `VgmHeader` and the header size in bytes.
/// 2. Iterate commands starting immediately after the header and decode
///    each command using `parse_vgm_command`. Each command parse returns
///    a `(VgmCommand, consumed_bytes)` pair; consumed bytes include the
///    opcode and payload.
/// 3. If the header declares a non-zero `gd3_offset`, attempt to parse
///    the GD3 metadata using `crate::meta::parse_gd3` and attach it to
///    the resulting `VgmDocument::gd3` field. GD3 parsing errors are
///    ignored here (the document will contain `None` on failure).
///
/// Returns `Ok(VgmDocument)` on success or a `ParseError` if header or
/// any command parsing fails.
pub(crate) fn parse_vgm(bytes: &[u8]) -> Result<VgmDocument, ParseError> {
    let (header, mut off) = parse_vgm_header(bytes)?;

    let mut commands: Vec<VgmCommand> = Vec::new();

    while off < bytes.len() {
        let (cmd, cons) = parse_vgm_command(bytes, off)?;
        commands.push(cmd.clone());
        off = off.wrapping_add(cons);

        if let VgmCommand::EndOfData(_) = commands.last().unwrap() {
            break;
        }
    }

    // Attach GD3 metadata if present (gd3_offset is stored as gd3_start - 0x14).
    let gd3 = if header.gd3_offset != 0 {
        let gd3_start = header.gd3_offset.wrapping_add(0x14) as usize;
        // If the computed start is outside the buffer, treat it as an out-of-range offset.
        if gd3_start >= bytes.len() {
            return Err(ParseError::OffsetOutOfRange(gd3_start));
        }
        // Attempt to parse GD3 and propagate any parse error to the caller.
        match parse_gd3(&bytes[gd3_start..]) {
            Ok(g) => Some(g),
            Err(e) => return Err(e),
        }
    } else {
        None
    };

    // Attach extra header if present (extra_header_offset stored at 0xBC in main header).
    let extra_header = if header.extra_header_offset != 0 {
        let start = header.extra_header_offset.wrapping_add(0xBC) as usize;
        // If the computed start is outside the buffer, treat it as an out-of-range offset.
        if start >= bytes.len() {
            return Err(ParseError::OffsetOutOfRange(start));
        }
        // Parse the extra header and propagate any parse error to the caller.
        match parse_vgm_extra_header(bytes, start) {
            Ok((eh, _)) => Some(eh),
            Err(e) => return Err(e),
        }
    } else {
        None
    };

    Ok(VgmDocument {
        header,
        commands,
        gd3,
        extra_header,
    })
}

/// Parse a VGM header located at the start of `bytes`.
///
/// This performs strict validation of the header: verifies the 4-byte
/// ident (`"Vgm "`), reads the version and the `data_offset` field,
/// and uses the legacy fallback when `data_offset` is zero
/// (interpreted as `VGM_V171_HEADER_SIZE - 0x34`). The full header
/// size is computed as `0x34 + data_offset`. The function ensures that
/// the provided slice contains the complete header before reading
/// extended fields.
///
/// On success returns `(VgmHeader, header_size)`, where `header_size`
/// is the number of bytes consumed by the header. On failure returns a
/// `ParseError` (for example `HeaderTooShort`, `InvalidIdent`, or
/// `UnexpectedEof`).
pub(crate) fn parse_vgm_header(bytes: &[u8]) -> Result<(VgmHeader, usize), ParseError> {
    // Require at least the 0x34 bytes present to read the data_offset
    if bytes.len() < 0x34 {
        return Err(ParseError::HeaderTooShort);
    }

    let ident_slice = read_slice(bytes, 0x00, 4)?;
    if ident_slice != b"Vgm " {
        let mut id: [u8; 4] = [0; 4];
        id.copy_from_slice(ident_slice);
        return Err(ParseError::InvalidIdent(id));
    }

    let version = read_u32_le_at(bytes, 0x08)?;

    // Read raw data_offset field (at 0x34). Writer treats 0 as
    // the legacy default; mirror that interpretation here.
    let data_offset_raw = read_u32_le_at(bytes, 0x34)?;
    let data_offset = if data_offset_raw == 0 {
        VGM_V171_HEADER_SIZE - 0x34
    } else {
        data_offset_raw
    };

    let header_size = 0x34usize.wrapping_add(data_offset as usize);
    if bytes.len() < header_size {
        return Err(ParseError::OffsetOutOfRange(header_size));
    }

    let mut h = VgmHeader::default();
    // Core fields always present
    h.ident.copy_from_slice(&bytes[0x00..0x04]);
    h.eof_offset = read_u32_le_at(bytes, 0x04)?;
    h.version = version;
    h.sn76489_clock = read_u32_le_at(bytes, 0x0C)?;
    h.ym2413_clock = read_u32_le_at(bytes, 0x10)?;
    h.gd3_offset = read_u32_le_at(bytes, 0x14)?;
    h.total_samples = read_u32_le_at(bytes, 0x18)?;
    h.loop_offset = read_u32_le_at(bytes, 0x1C)?;
    h.loop_samples = read_u32_le_at(bytes, 0x20)?;
    h.sample_rate = read_u32_le_at(bytes, 0x24)?;
    h.sn_fb = read_u16_le_at(bytes, 0x28)?;
    h.snw = read_u8_at(bytes, 0x2A)?;
    h.sf = read_u8_at(bytes, 0x2B)?;
    h.ym2612_clock = read_u32_le_at(bytes, 0x2C)?;
    h.ym2151_clock = read_u32_le_at(bytes, 0x30)?;
    h.data_offset = data_offset_raw;

    // Following fields are part of the extended header region.
    h.sega_pcm_clock = read_u32_le_at(bytes, 0x38)?;
    h.spcm_interface = read_u32_le_at(bytes, 0x3C)?;
    h.rf5c68_clock = read_u32_le_at(bytes, 0x40)?;
    h.ym2203_clock = read_u32_le_at(bytes, 0x44)?;
    h.ym2608_clock = read_u32_le_at(bytes, 0x48)?;
    h.ym2610b_clock = read_u32_le_at(bytes, 0x4C)?;
    h.ym3812_clock = read_u32_le_at(bytes, 0x50)?;
    h.ym3526_clock = read_u32_le_at(bytes, 0x54)?;
    h.y8950_clock = read_u32_le_at(bytes, 0x58)?;
    h.ymf262_clock = read_u32_le_at(bytes, 0x5C)?;
    h.ymf278b_clock = read_u32_le_at(bytes, 0x60)?;
    h.ymf271_clock = read_u32_le_at(bytes, 0x64)?;
    h.ymz280b_clock = read_u32_le_at(bytes, 0x68)?;
    h.rf5c164_clock = read_u32_le_at(bytes, 0x6C)?;
    h.pwm_clock = read_u32_le_at(bytes, 0x70)?;
    h.ay8910_clock = read_u32_le_at(bytes, 0x74)?;
    let ay_misc_slice = read_slice(bytes, 0x78, 8)?;
    h.ay_misc.copy_from_slice(ay_misc_slice);
    h.gb_dmg_clock = read_u32_le_at(bytes, 0x80)?;
    h.nes_apu_clock = read_u32_le_at(bytes, 0x84)?;
    h.multipcm_clock = read_u32_le_at(bytes, 0x88)?;
    h.upd7759_clock = read_u32_le_at(bytes, 0x8C)?;
    h.okim6258_clock = read_u32_le_at(bytes, 0x90)?;
    let ok_flags = read_slice(bytes, 0x94, 4)?;
    h.okim6258_flags.copy_from_slice(ok_flags);
    h.okim6295_clock = read_u32_le_at(bytes, 0x98)?;
    h.k051649_clock = read_u32_le_at(bytes, 0x9C)?;
    h.k054539_clock = read_u32_le_at(bytes, 0xA0)?;
    h.huc6280_clock = read_u32_le_at(bytes, 0xA4)?;
    h.c140_clock = read_u32_le_at(bytes, 0xA8)?;
    h.k053260_clock = read_u32_le_at(bytes, 0xAC)?;
    h.pokey_clock = read_u32_le_at(bytes, 0xB0)?;
    h.qsound_clock = read_u32_le_at(bytes, 0xB4)?;
    h.scsp_clock = read_u32_le_at(bytes, 0xB8)?;
    h.extra_header_offset = read_u32_le_at(bytes, 0xBC)?;
    h.wonderswan_clock = read_u32_le_at(bytes, 0xC0)?;
    h.vsu_clock = read_u32_le_at(bytes, 0xC4)?;
    h.saa1099_clock = read_u32_le_at(bytes, 0xC8)?;
    h.es5503_clock = read_u32_le_at(bytes, 0xCC)?;
    h.es5506_clock = read_u32_le_at(bytes, 0xD0)?;
    h.es5506_channels = read_u16_le_at(bytes, 0xD4)?;
    h.es5506_cd = read_u8_at(bytes, 0xD6)?;
    h.es5506_reserved = read_u8_at(bytes, 0xD7)?;
    h.x1_010_clock = read_u32_le_at(bytes, 0xD8)?;
    h.c352_clock = read_u32_le_at(bytes, 0xDC)?;
    h.ga20_clock = read_u32_le_at(bytes, 0xE0)?;
    h.mikey_clock = read_u32_le_at(bytes, 0xE4)?;
    let res_e8 = read_slice(bytes, 0xE8, 8)?;
    h.reserved_e8_ef.copy_from_slice(res_e8);
    let res_f0 = read_slice(bytes, 0xF0, 16)?;
    h.reserved_f0_ff.copy_from_slice(res_f0);

    Ok((h, header_size))
}

/// Parse a VGM extra-header (v1.70+) located at `offset` within `bytes`.
///
/// The extra header format:
/// - u32 LE header_size (including this field)
/// - u32 LE offset to chip-clock block (relative to start of extra header, 0 = none)
/// - u32 LE offset to chip-volume block (relative to start of extra header, 0 = none)
/// - optional chip-clock block: 1 byte count, then count * (1 byte chip_id + 4 byte LE clock)
/// - optional chip-volume block: 1 byte count, then count * (1 byte chip_id + 1 byte flags + 2 byte LE volume)
pub(crate) fn parse_vgm_extra_header(
    bytes: &[u8],
    offset: usize,
) -> Result<(VgmExtraHeader, usize), ParseError> {
    // Read the three header fields (12 bytes)
    let header_size = read_u32_le_at(bytes, offset)?;
    let chip_clock_offset = read_u32_le_at(bytes, offset + 4)?;
    let chip_vol_offset = read_u32_le_at(bytes, offset + 8)?;

    let mut extra = VgmExtraHeader {
        header_size,
        chip_clock_offset,
        chip_vol_offset,
        chip_clocks: Vec::new(),
        chip_volumes: Vec::new(),
    };

    // Parse chip clocks block if present (offset is relative to extra header start)
    if chip_clock_offset != 0 {
        let cc_base = offset.wrapping_add(chip_clock_offset as usize);
        // first byte is entry count
        let count = read_u8_at(bytes, cc_base)?;
        let mut cur = cc_base + 1;
        for _ in 0..count {
            let chip_id = read_u8_at(bytes, cur)?;
            let clock = read_u32_le_at(bytes, cur + 1)?;
            extra.chip_clocks.push((chip_id, clock));
            cur = cur.wrapping_add(5);
        }
    }

    // Parse chip volumes block if present (offset is relative to extra header start)
    if chip_vol_offset != 0 {
        let cv_base = offset.wrapping_add(chip_vol_offset as usize);
        // first byte is entry count
        let count = read_u8_at(bytes, cv_base)?;
        let mut cur = cv_base + 1;
        for _ in 0..count {
            let chip_id = read_u8_at(bytes, cur)?;
            let flags = read_u8_at(bytes, cur + 1)?;
            let volume = read_u16_le_at(bytes, cur + 2)?;
            extra.chip_volumes.push((chip_id, flags, volume));
            cur = cur.wrapping_add(4);
        }
    }

    Ok((extra, header_size as usize))
}

/// Parse a single VGM command beginning at `off` within `bytes`.
///
/// The function reads the opcode byte at `off`, dispatches to the
/// appropriate per-command parser for commands with payload, and
/// returns the decoded `VgmCommand` together with the total number of
/// bytes consumed (including the opcode byte). If the opcode is not a
/// recognized non-chip command, the parser will try to interpret it as
/// a chip write for the primary instance and then for the secondary
/// instance (secondary opcodes are the primary opcode + 0x50) by
/// delegating to `parse_chip_write`.
///
/// Returns `Ok((VgmCommand, consumed_bytes))` on success or a
/// `ParseError` on failure.
pub(crate) fn parse_vgm_command(
    bytes: &[u8],
    off: usize,
) -> Result<(VgmCommand, usize), ParseError> {
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
            for &instance in &[Instance::Primary, Instance::Secondary] {
                let opcode = match instance {
                    Instance::Primary => other,
                    Instance::Secondary => other.wrapping_sub(0x50),
                };
                match parse_chip_write(opcode, instance, bytes, cur) {
                    Ok((cmd, cons)) => return Ok((cmd, 1 + cons)),
                    Err(ParseError::Other(_)) => continue,
                    Err(e) => return Err(e),
                }
            }

            // If no chip write matched, try reserved opcode ranges as a fallback.
            match parse_reserved_write(other, bytes, cur) {
                Ok((cmd, cons)) => return Ok((cmd, 1 + cons)),
                Err(ParseError::Other(_)) => {}
                Err(e) => return Err(e),
            }

            Err(ParseError::Other(format!("unknown opcode {:#X}", other)))
        }
    }
}

/// Parse a chip write payload and return the corresponding
/// `VgmCommand` plus the number of bytes consumed by the chip-specific
/// payload parser.
///
/// The `opcode` parameter is the base opcode value for the primary
/// instance (the caller is responsible for passing the correctly
/// adjusted base for secondary instances if required). `instance`
/// indicates whether the command targets the primary or secondary
/// chip instance and is encoded into the returned `VgmCommand`.
///
/// `bytes` and `offset` indicate the source buffer and the start of
/// the chip-specific payload (the per-chip `CommandSpec::parse`
/// implementations expect `offset` to point at the payload bytes,
/// not the opcode). This function dispatches to the appropriate
/// `<chip::XxxSpec as CommandSpec>::parse` implementation and wraps the
/// resulting spec into the matching `VgmCommand` variant.
pub(crate) fn parse_chip_write(
    opcode: u8,
    instance: Instance,
    bytes: &[u8],
    offset: usize,
) -> Result<(VgmCommand, usize), ParseError> {
    match opcode {
        0x50 => {
            let (spec, n) = <chip::PsgSpec as CommandSpec>::parse(bytes, offset, opcode)?;
            Ok((VgmCommand::Sn76489Write(instance, spec), n))
        }
        0x51 => {
            let (spec, n) = <chip::Ym2413Spec as CommandSpec>::parse(bytes, offset, opcode)?;
            Ok((VgmCommand::Ym2413Write(instance, spec), n))
        }
        0x52 | 0x53 => {
            let (spec, n) = <chip::Ym2612Spec as CommandSpec>::parse(bytes, offset, opcode)?;
            Ok((VgmCommand::Ym2612Write(instance, spec), n))
        }
        0x54 => {
            let (spec, n) = <chip::Ym2151Spec as CommandSpec>::parse(bytes, offset, opcode)?;
            Ok((VgmCommand::Ym2151Write(instance, spec), n))
        }
        0x55 => {
            let (spec, n) = <chip::Ym2203Spec as CommandSpec>::parse(bytes, offset, opcode)?;
            Ok((VgmCommand::Ym2203Write(instance, spec), n))
        }
        0x56 | 0x57 => {
            let (spec, n) = <chip::Ym2608Spec as CommandSpec>::parse(bytes, offset, opcode)?;
            Ok((VgmCommand::Ym2608Write(instance, spec), n))
        }
        0x58 | 0x59 => {
            let (spec, n) = <chip::Ym2610Spec as CommandSpec>::parse(bytes, offset, opcode)?;
            Ok((VgmCommand::Ym2610bWrite(instance, spec), n))
        }
        0x5A => {
            let (spec, n) = <chip::Ym3812Spec as CommandSpec>::parse(bytes, offset, opcode)?;
            Ok((VgmCommand::Ym3812Write(instance, spec), n))
        }
        0x5B => {
            let (spec, n) = <chip::Ym3526Spec as CommandSpec>::parse(bytes, offset, opcode)?;
            Ok((VgmCommand::Ym3526Write(instance, spec), n))
        }
        0x5C => {
            let (spec, n) = <chip::Y8950Spec as CommandSpec>::parse(bytes, offset, opcode)?;
            Ok((VgmCommand::Y8950Write(instance, spec), n))
        }
        0x5D => {
            let (spec, n) = <chip::Ymz280bSpec as CommandSpec>::parse(bytes, offset, opcode)?;
            Ok((VgmCommand::Ymz280bWrite(instance, spec), n))
        }
        0x5E | 0x5F => {
            let (spec, n) = <chip::Ymf262Spec as CommandSpec>::parse(bytes, offset, opcode)?;
            Ok((VgmCommand::Ymf262Write(instance, spec), n))
        }
        0xC0 => {
            let (spec, n) = <chip::SegaPcmSpec as CommandSpec>::parse(bytes, offset, opcode)?;
            Ok((VgmCommand::SegaPcmWrite(instance, spec), n))
        }
        0xC1 => {
            let (spec, n) = <chip::Rf5c68Spec as CommandSpec>::parse(bytes, offset, opcode)?;
            Ok((VgmCommand::Rf5c68Write(instance, spec), n))
        }
        0xB2 => {
            let (spec, n) = <chip::PwmSpec as CommandSpec>::parse(bytes, offset, opcode)?;
            Ok((VgmCommand::PwmWrite(instance, spec), n))
        }
        0xA0 => {
            let (spec, n) = <chip::Ay8910Spec as CommandSpec>::parse(bytes, offset, opcode)?;
            Ok((VgmCommand::Ay8910Write(instance, spec), n))
        }
        0xB3 => {
            let (spec, n) = <chip::GbDmgSpec as CommandSpec>::parse(bytes, offset, opcode)?;
            Ok((VgmCommand::GbDmgWrite(instance, spec), n))
        }
        0xB4 => {
            let (spec, n) = <chip::NesApuSpec as CommandSpec>::parse(bytes, offset, opcode)?;
            Ok((VgmCommand::NesApuWrite(instance, spec), n))
        }
        0xB5 => {
            let (spec, n) = <chip::MultiPcmSpec as CommandSpec>::parse(bytes, offset, opcode)?;
            Ok((VgmCommand::MultiPcmWrite(instance, spec), n))
        }
        0xB6 => {
            let (spec, n) = <chip::Upd7759Spec as CommandSpec>::parse(bytes, offset, opcode)?;
            Ok((VgmCommand::Upd7759Write(instance, spec), n))
        }
        0xB7 => {
            let (spec, n) = <chip::Okim6258Spec as CommandSpec>::parse(bytes, offset, opcode)?;
            Ok((VgmCommand::Okim6258Write(instance, spec), n))
        }
        0xB8 => {
            let (spec, n) = <chip::Okim6295Spec as CommandSpec>::parse(bytes, offset, opcode)?;
            Ok((VgmCommand::Okim6295Write(instance, spec), n))
        }
        0xD0 => {
            let (spec, n) = <chip::Ymf278bSpec as CommandSpec>::parse(bytes, offset, opcode)?;
            Ok((VgmCommand::Ymf278bWrite(instance, spec), n))
        }
        0xD1 => {
            let (spec, n) = <chip::Ymf271Spec as CommandSpec>::parse(bytes, offset, opcode)?;
            Ok((VgmCommand::Ymf271Write(instance, spec), n))
        }
        0xD2 => {
            let (spec, n) = <chip::Scc1Spec as CommandSpec>::parse(bytes, offset, opcode)?;
            Ok((VgmCommand::Scc1Write(instance, spec), n))
        }
        0xD3 => {
            let (spec, n) = <chip::K054539Spec as CommandSpec>::parse(bytes, offset, opcode)?;
            Ok((VgmCommand::K054539Write(instance, spec), n))
        }
        0xD4 => {
            let (spec, n) = <chip::C140Spec as CommandSpec>::parse(bytes, offset, opcode)?;
            Ok((VgmCommand::C140Write(instance, spec), n))
        }
        0xD5 => {
            let (spec, n) = <chip::Es5503Spec as CommandSpec>::parse(bytes, offset, opcode)?;
            Ok((VgmCommand::Es5503Write(instance, spec), n))
        }
        0xBE => {
            let (spec, n) = <chip::Es5506U8Spec as CommandSpec>::parse(bytes, offset, opcode)?;
            Ok((VgmCommand::Es5506BEWrite(instance, spec), n))
        }
        0xD6 => {
            let (spec, n) = <chip::Es5506U16Spec as CommandSpec>::parse(bytes, offset, opcode)?;
            Ok((VgmCommand::Es5506D6Write(instance, spec), n))
        }
        0xC4 => {
            let (spec, n) = <chip::QsoundSpec as CommandSpec>::parse(bytes, offset, opcode)?;
            Ok((VgmCommand::QsoundWrite(instance, spec), n))
        }
        0xC5 => {
            let (spec, n) = <chip::ScspSpec as CommandSpec>::parse(bytes, offset, opcode)?;
            Ok((VgmCommand::ScspWrite(instance, spec), n))
        }
        0xC6 => {
            let (spec, n) = <chip::WonderSwanSpec as CommandSpec>::parse(bytes, offset, opcode)?;
            Ok((VgmCommand::WonderSwanWrite(instance, spec), n))
        }
        0xC7 => {
            let (spec, n) = <chip::VsuSpec as CommandSpec>::parse(bytes, offset, opcode)?;
            Ok((VgmCommand::VsuWrite(instance, spec), n))
        }
        0xC8 => {
            let (spec, n) = <chip::X1010Spec as CommandSpec>::parse(bytes, offset, opcode)?;
            Ok((VgmCommand::X1010Write(instance, spec), n))
        }
        0xE1 => {
            let (spec, n) = <chip::C352Spec as CommandSpec>::parse(bytes, offset, opcode)?;
            Ok((VgmCommand::C352Write(instance, spec), n))
        }
        0xBF => {
            let (spec, n) = <chip::Ga20Spec as CommandSpec>::parse(bytes, offset, opcode)?;
            Ok((VgmCommand::Ga20Write(instance, spec), n))
        }
        0x40 => {
            let (spec, n) = <chip::MikeySpec as CommandSpec>::parse(bytes, offset, opcode)?;
            Ok((VgmCommand::MikeyWrite(instance, spec), n))
        }
        0x4F => {
            let (spec, n) = <chip::GameGearPsgSpec as CommandSpec>::parse(bytes, offset, opcode)?;
            Ok((VgmCommand::GameGearPsgWrite(instance, spec), n))
        }
        _ => Err(ParseError::Other(format!(
            "unknown chip base opcode {:#X}",
            opcode
        ))),
    }
}

/// Parse reserved (non-chip) VGM write opcodes.
///
/// This mirrors the structure of `parse_chip_write` but handles the
/// reserved opcode ranges that map to `ReservedU8`, `ReservedU16`,
/// `ReservedU24`, and `ReservedU32` command specs. The `opcode`
/// parameter is the opcode byte as seen in the VGM stream (the parser
/// expects the caller to have consumed the opcode byte already and
/// `offset` points at the first payload byte).
pub(crate) fn parse_reserved_write(
    opcode: u8,
    bytes: &[u8],
    offset: usize,
) -> Result<(VgmCommand, usize), ParseError> {
    match opcode {
        // ReservedU8: 0x30..=0x3F
        0x30..=0x3F => {
            let (spec, n) = ReservedU8::parse(bytes, offset, opcode)?;
            Ok((VgmCommand::ReservedU8Write(spec), n))
        }

        // ReservedU16: 0x41..=0x4E
        0x41..=0x4E => {
            let (spec, n) = ReservedU16::parse(bytes, offset, opcode)?;
            Ok((VgmCommand::ReservedU16Write(spec), n))
        }

        // ReservedU24: 0xC9..=0xCF and 0xD7..=0xDF
        0xC9..=0xCF | 0xD7..=0xDF => {
            let (spec, n) = ReservedU24::parse(bytes, offset, opcode)?;
            Ok((VgmCommand::ReservedU24Write(spec), n))
        }

        // ReservedU32: 0xE2..=0xFF
        0xE2..=0xFF => {
            let (spec, n) = ReservedU32::parse(bytes, offset, opcode)?;
            Ok((VgmCommand::ReservedU32Write(spec), n))
        }

        _ => Err(ParseError::Other(format!(
            "unknown reserved opcode {:#X}",
            opcode
        ))),
    }
}
