use crate::binutil::{write_slice, write_u8, write_u16, write_u32};
use crate::chip;
use crate::vgm::command::Instance;
use crate::vgm::parser;
use std::convert::TryFrom;

pub(crate) const VGM_V171_HEADER_SIZE: u32 = 0x100;

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

impl VgmHeader {
    pub(crate) fn to_bytes(&self, gd3_offset: u32, data_offset: u32) -> Vec<u8> {
        let mut buf: Vec<u8> = vec![0; VGM_V171_HEADER_SIZE as usize];
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
        let header_size = 0x34_u32.wrapping_add(data_offset) as usize;
        if header_size < buf.len() {
            buf.truncate(header_size);
        }
        buf
    }

    /// Set the stored clock field for a chip `ch` at the given `instance`.
    /// For secondary instances the high bit is set on the stored value
    /// following VGM header convention.
    pub fn set_chip_clock(&mut self, ch: chip::Chip, instance: Instance, master_clock: u32) {
        let clock = match instance {
            Instance::Primary => master_clock,
            Instance::Secondary => master_clock | 0x8000_0000_u32,
        };

        match &ch {
            chip::Chip::Sn76489 => self.sn76489_clock = clock,
            chip::Chip::Ym2413 => self.ym2413_clock = clock,
            chip::Chip::Ym2612 => self.ym2612_clock = clock,
            chip::Chip::Ym2151 => self.ym2151_clock = clock,
            chip::Chip::SegaPcm => self.sega_pcm_clock = clock,
            chip::Chip::Rf5c68 => self.rf5c68_clock = clock,
            chip::Chip::Ym2203 => self.ym2203_clock = clock,
            chip::Chip::Ym2608 => self.ym2608_clock = clock,
            chip::Chip::Ym2610b => self.ym2610b_clock = clock,
            chip::Chip::Ym3812 => self.ym3812_clock = clock,
            chip::Chip::Ym3526 => self.ym3526_clock = clock,
            chip::Chip::Y8950 => self.y8950_clock = clock,
            chip::Chip::Ymf262 => self.ymf262_clock = clock,
            chip::Chip::Ymf278b => self.ymf278b_clock = clock,
            chip::Chip::Ymf271 => self.ymf271_clock = clock,
            chip::Chip::Ymz280b => self.ymz280b_clock = clock,
            chip::Chip::Rf5c164 => self.rf5c164_clock = clock,
            chip::Chip::Pwm => self.pwm_clock = clock,
            chip::Chip::Ay8910 => self.ay8910_clock = clock,
            chip::Chip::GbDmg => self.gb_dmg_clock = clock,
            chip::Chip::NesApu => self.nes_apu_clock = clock,
            chip::Chip::MultiPcm => self.multipcm_clock = clock,
            chip::Chip::Upd7759 => self.upd7759_clock = clock,
            chip::Chip::Okim6258 => self.okim6258_clock = clock,
            chip::Chip::Okim6295 => self.okim6295_clock = clock,
            chip::Chip::K051649 => self.k051649_clock = clock,
            chip::Chip::K054539 => self.k054539_clock = clock,
            chip::Chip::Huc6280 => self.huc6280_clock = clock,
            chip::Chip::C140 => self.c140_clock = clock,
            chip::Chip::K053260 => self.k053260_clock = clock,
            chip::Chip::Pokey => self.pokey_clock = clock,
            chip::Chip::Qsound => self.qsound_clock = clock,
            chip::Chip::Scsp => self.scsp_clock = clock,
            chip::Chip::WonderSwan => self.wonderswan_clock = clock,
            chip::Chip::Vsu => self.vsu_clock = clock,
            chip::Chip::Saa1099 => self.saa1099_clock = clock,
            chip::Chip::Es5503 => self.es5503_clock = clock,
            chip::Chip::Es5506U8 => self.es5506_clock = clock,
            chip::Chip::Es5506U16 => self.es5506_clock = clock,
            chip::Chip::X1010 => self.x1_010_clock = clock,
            chip::Chip::C352 => self.c352_clock = clock,
            chip::Chip::Ga20 => self.ga20_clock = clock,
            chip::Chip::Mikey => self.mikey_clock = clock,
            _ => {}
        }
    }
}

/// Attempt to convert a raw VGM byte slice into a `VgmHeader`.
impl TryFrom<&[u8]> for VgmHeader {
    type Error = crate::binutil::ParseError;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        parser::parse_vgm_header(bytes).map(|(h, _)| h)
    }
}

/// Extra header introduced in VGM v1.70.
///
/// Format summary (see VGM specification):
/// - 32-bit LE header size (including this 4-byte size field)
/// - 32-bit LE offset to chip-clock block (relative to start of extra header, 0 = none)
/// - 32-bit LE offset to chip-volume block (relative to start of extra header, 0 = none)
/// - additional data follows at offsets above
#[derive(Debug, Clone, PartialEq)]
pub struct VgmExtraHeader {
    /// Full extra header size (as stored on-disk)
    pub header_size: u32,
    /// Offset (relative to start of extra header) to chip clock list (0 if absent)
    pub chip_clock_offset: u32,
    /// Offset (relative to start of extra header) to chip volume list (0 if absent)
    pub chip_vol_offset: u32,

    /// Parsed chip clock entries: (chip_id, clock)
    /// chip_id is the 1-byte ID following the main header's chip order.
    pub chip_clocks: Vec<(u8, u32)>,

    /// Parsed chip volume entries: (chip_id_with_flags, flags, volume)
    /// Each entry in the on-disk format is: 1 byte chip id, 1 byte flags, 2 bytes volume (LE).
    pub chip_volumes: Vec<(u8, u8, u16)>,
}

impl VgmExtraHeader {
    /// Serialize the extra header into bytes using the VGM extra-header format.
    ///
    /// The produced buffer contains the 12-byte header (size, offsets) followed
    /// by optional chip-clock and chip-volume blocks when present. Offsets in
    /// the header are expressed relative to the start of this extra header.
    pub fn to_bytes(&self) -> Vec<u8> {
        // Start with a 12-byte placeholder for header_size, chip_clock_offset, chip_vol_offset
        let mut buf: Vec<u8> = vec![0u8; 12];

        // chip_clock block
        let chip_clock_offset: u32 = if !self.chip_clocks.is_empty() {
            let off = buf.len() as u32;
            // count (1 byte)
            buf.push(self.chip_clocks.len() as u8);
            // entries: chip_id (1 byte) + clock (4 bytes LE)
            for (chip_id, clock) in &self.chip_clocks {
                buf.push(*chip_id);
                buf.extend_from_slice(&clock.to_le_bytes());
            }
            off
        } else {
            0u32
        };

        // chip_volume block
        let chip_vol_offset: u32 = if !self.chip_volumes.is_empty() {
            let off = buf.len() as u32;
            // count (1 byte)
            buf.push(self.chip_volumes.len() as u8);
            // entries: chip_id (1 byte) + flags (1 byte) + volume (2 bytes LE)
            for (chip_id, flags, volume) in &self.chip_volumes {
                buf.push(*chip_id);
                buf.push(*flags);
                buf.extend_from_slice(&volume.to_le_bytes());
            }
            off
        } else {
            0u32
        };

        // Now fill in the 3 header fields
        let header_size = buf.len() as u32;
        buf[0..4].copy_from_slice(&header_size.to_le_bytes());
        buf[4..8].copy_from_slice(&chip_clock_offset.to_le_bytes());
        buf[8..12].copy_from_slice(&chip_vol_offset.to_le_bytes());

        buf
    }
}
