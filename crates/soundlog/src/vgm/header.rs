use crate::binutil::{
    ParseError, read_slice, read_u8_at, read_u16_le_at, read_u32_le_at, write_slice, write_u8,
    write_u16, write_u32,
};

pub const VGM_V171_HEADER_SIZE: u32 = 0x100;

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
    pub fn to_bytes(&self, gd3_offset: u32, data_offset: u32) -> Vec<u8> {
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
        let header_size = 0x34u32.wrapping_add(data_offset) as usize;
        if header_size < buf.len() {
            buf.truncate(header_size);
        }
        buf
    }

    /// Parse a VGM header from `bytes`.
    ///
    /// Returns the parsed `VgmHeader` and the number of bytes consumed
    /// for the header (the header size). This function performs strict
    /// checks for the minimal header size and the `Vgm ` ident.
    pub fn from_bytes(bytes: &[u8]) -> Result<(VgmHeader, usize), ParseError> {
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
            return Err(ParseError::UnexpectedEof);
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
}
