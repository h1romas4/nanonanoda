#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum VgmChip {
    Ymf262,
    Ym2203,
}

#[derive(Debug, Clone, PartialEq)]
pub enum VgmCommand {
    WaitSamples(u32),
    Wait60Hz,
    Wait50Hz,
    Ymf262Write { port: u8, register: u8, value: u8 },
    Ym2203Write { port: u8, register: u8, value: u8 },
    EndOfData,
}

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Gd3 {
    pub track_name_en: Option<String>,
    pub track_name_jp: Option<String>,
    pub game_name_en: Option<String>,
    pub game_name_jp: Option<String>,
    pub system_name_en: Option<String>,
    pub system_name_jp: Option<String>,
    pub author_name_en: Option<String>,
    pub author_name_jp: Option<String>,
    pub release_date: Option<String>,
    pub creator: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VgmDocument {
    pub header: VgmHeader,
    pub commands: Vec<VgmCommand>,
    pub gd3: Option<Gd3>,
}

impl VgmDocument {
    pub fn new_empty() -> Self {
        VgmDocument {
            header: VgmHeader {
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
                okim6258_flags: [0; 4],
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
                reserved_e8_ef: [0; 8],
                reserved_f0_ff: [0; 16],
            },
            commands: Vec::new(),
            gd3: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct VgmBuilder {
    doc: VgmDocument,
}

impl Default for VgmBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl VgmBuilder {
    pub fn new() -> Self {
        VgmBuilder {
            doc: VgmDocument::new_empty(),
        }
    }

    pub fn set_gd3(&mut self, gd3: Gd3) {
        self.doc.gd3 = Some(gd3);
    }

    pub fn set_version(&mut self, v: u32) {
        self.doc.header.version = v;
    }

    pub fn set_sample_rate(&mut self, sr: u32) {
        self.doc.header.sample_rate = sr;
    }

    pub fn add_chip_clock(&mut self, chip: VgmChip, clock_hz: u32) {
        match chip {
            VgmChip::Ym2203 => self.doc.header.ym2203_clock = clock_hz,
            VgmChip::Ymf262 => self.doc.header.ymf262_clock = clock_hz,
        }
    }

    pub fn enable_dual_chip(&mut self, chip: VgmChip) {
        const DUAL_BIT: u32 = 0x4000_0000;
        match chip {
            VgmChip::Ym2203 => self.doc.header.ym2203_clock |= DUAL_BIT,
            VgmChip::Ymf262 => self.doc.header.ymf262_clock |= DUAL_BIT,
        }
    }

    pub fn wait_samples(&mut self, samples: u32) {
        self.doc.commands.push(VgmCommand::WaitSamples(samples));
    }

    pub fn wait_60hz(&mut self) {
        self.doc.commands.push(VgmCommand::Wait60Hz);
    }

    pub fn wait_50hz(&mut self) {
        self.doc.commands.push(VgmCommand::Wait50Hz);
    }

    pub fn ymf262_write(&mut self, port: u8, register: u8, value: u8) {
        self.doc.commands.push(VgmCommand::Ymf262Write {
            port,
            register,
            value,
        });
    }

    pub fn ym2203_write(&mut self, port: u8, register: u8, value: u8) {
        self.doc.commands.push(VgmCommand::Ym2203Write {
            port,
            register,
            value,
        });
    }

    pub fn end(&mut self) {
        self.doc.commands.push(VgmCommand::EndOfData);
    }

    pub fn build(self) -> VgmDocument {
        self.doc
    }
}

impl VgmDocument {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = vec![0; 0x100];

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

        let mut cmd_buf: Vec<u8> = Vec::new();
        let mut total_samples_u64: u64 = 0;
        for cmd in &self.commands {
            match cmd {
                VgmCommand::WaitSamples(n) => {
                    // Count samples and emit 0x61 chunks
                    total_samples_u64 = total_samples_u64.saturating_add(*n as u64);
                    let mut remaining = *n;
                    while remaining > 0 {
                        let this = if remaining > 0xFFFF {
                            0xFFFF_u32
                        } else {
                            remaining
                        } as u16;
                        cmd_buf.push(0x61);
                        cmd_buf.extend_from_slice(&this.to_le_bytes());
                        remaining = remaining.saturating_sub(this as u32);
                    }
                }
                VgmCommand::Wait60Hz => {
                    total_samples_u64 =
                        total_samples_u64.saturating_add(self.header.sample_rate as u64 / 60u64);
                    cmd_buf.push(0x62)
                }
                VgmCommand::Wait50Hz => {
                    total_samples_u64 =
                        total_samples_u64.saturating_add(self.header.sample_rate as u64 / 50u64);
                    cmd_buf.push(0x63_u8)
                }
                VgmCommand::EndOfData => cmd_buf.push(0x66u8),
                VgmCommand::Ymf262Write {
                    port,
                    register,
                    value,
                } => {
                    let base: u8 = if (port & 1) == 0 { 0x5E } else { 0x5F };
                    let opcode = if (port & 0x02) != 0 {
                        base.wrapping_add(0x50)
                    } else {
                        base
                    };
                    cmd_buf.push(opcode);
                    cmd_buf.push(*register);
                    cmd_buf.push(*value);
                }
                VgmCommand::Ym2203Write {
                    port,
                    register,
                    value,
                } => {
                    let base: u8 = 0x55;
                    let opcode = if (*port) != 0 {
                        base.wrapping_add(0x50)
                    } else {
                        base
                    };
                    cmd_buf.push(opcode);
                    cmd_buf.push(*register);
                    cmd_buf.push(*value);
                }
            }
        }

        let wrote_end_in_cmds = self
            .commands
            .iter()
            .any(|c| matches!(c, VgmCommand::EndOfData));

        let gd3_offset_val: u32 = if self.gd3.is_some() {
            (0x100u32)
                .wrapping_add(cmd_buf.len() as u32)
                .wrapping_sub(0x14)
        } else {
            0u32
        };

        // ident (0x00)
        write_slice(&mut buf, 0x00, &self.header.ident);
        // eof_offset (0x04) placeholder -> 0 for now
        write_u32(&mut buf, 0x04, 0);
        // version (0x08)
        write_u32(&mut buf, 0x08, self.header.version);
        // SN76489 clock (0x0C)
        write_u32(&mut buf, 0x0C, self.header.sn76489_clock);
        // YM2413 clock (0x10)
        write_u32(&mut buf, 0x10, self.header.ym2413_clock);
        // GD3 offset (0x14)
        write_u32(&mut buf, 0x14, gd3_offset_val);
        // total samples (0x18)
        write_u32(&mut buf, 0x18, self.header.total_samples);
        // loop offset (0x1C)
        write_u32(&mut buf, 0x1C, self.header.loop_offset);
        // loop samples (0x20)
        write_u32(&mut buf, 0x20, self.header.loop_samples);
        // sample rate (0x24)
        write_u32(&mut buf, 0x24, self.header.sample_rate);
        // SN FB (0x28) u16
        write_u16(&mut buf, 0x28, self.header.sn_fb);
        // SNW (0x2A) u8
        write_u8(&mut buf, 0x2A, self.header.snw);
        // SF (0x2B) u8
        write_u8(&mut buf, 0x2B, self.header.sf);
        // YM2612 clock (0x2C)
        write_u32(&mut buf, 0x2C, self.header.ym2612_clock);
        // YM2151 clock (0x30)
        write_u32(&mut buf, 0x30, self.header.ym2151_clock);
        // data offset (0x34)
        let data_offset_val: u32 = if self.header.data_offset != 0 {
            self.header.data_offset
        } else {
            0x100u32.wrapping_sub(0x34)
        };
        write_u32(&mut buf, 0x34, data_offset_val);
        // SegaPCM clock (0x38)
        write_u32(&mut buf, 0x38, self.header.sega_pcm_clock);
        // SPCM interface (0x3C)
        write_u32(&mut buf, 0x3C, self.header.spcm_interface);
        // RF5C68 (0x40)
        write_u32(&mut buf, 0x40, self.header.rf5c68_clock);
        // YM2203 (0x44)
        write_u32(&mut buf, 0x44, self.header.ym2203_clock);
        // YM2608 (0x48)
        write_u32(&mut buf, 0x48, self.header.ym2608_clock);
        // YM2610/B (0x4C)
        write_u32(&mut buf, 0x4C, self.header.ym2610b_clock);
        // YM3812 (0x50)
        write_u32(&mut buf, 0x50, self.header.ym3812_clock);
        // YM3526 (0x54)
        write_u32(&mut buf, 0x54, self.header.ym3526_clock);
        // Y8950 (0x58)
        write_u32(&mut buf, 0x58, self.header.y8950_clock);
        // YMF262 (0x5C)
        write_u32(&mut buf, 0x5C, self.header.ymf262_clock);
        // YMF278B (0x60)
        write_u32(&mut buf, 0x60, self.header.ymf278b_clock);
        // YMF271 (0x64)
        write_u32(&mut buf, 0x64, self.header.ymf271_clock);
        // YMZ280B (0x68)
        write_u32(&mut buf, 0x68, self.header.ymz280b_clock);
        // RF5C164 (0x6C)
        write_u32(&mut buf, 0x6C, self.header.rf5c164_clock);
        // PWM (0x70)
        write_u32(&mut buf, 0x70, self.header.pwm_clock);
        // AY8910 (0x74)
        write_u32(&mut buf, 0x74, self.header.ay8910_clock);
        // AY misc (0x78..0x7F)
        write_slice(&mut buf, 0x78, &self.header.ay_misc);
        // GB DMG (0x80)
        write_u32(&mut buf, 0x80, self.header.gb_dmg_clock);
        // NES APU (0x84)
        write_u32(&mut buf, 0x84, self.header.nes_apu_clock);
        // MultiPCM (0x88)
        write_u32(&mut buf, 0x88, self.header.multipcm_clock);
        // uPD7759 (0x8C)
        write_u32(&mut buf, 0x8C, self.header.upd7759_clock);
        // OKIM6258 (0x90)
        write_u32(&mut buf, 0x90, self.header.okim6258_clock);
        // OKIM6258 flags (0x94..0x97)
        write_slice(&mut buf, 0x94, &self.header.okim6258_flags);
        // OKIM6295 (0x98)
        write_u32(&mut buf, 0x98, self.header.okim6295_clock);
        // K051649 (0x9C)
        write_u32(&mut buf, 0x9C, self.header.k051649_clock);
        // K054539 (0xA0)
        write_u32(&mut buf, 0xA0, self.header.k054539_clock);
        // HuC6280 (0xA4)
        write_u32(&mut buf, 0xA4, self.header.huc6280_clock);
        // C140 (0xA8)
        write_u32(&mut buf, 0xA8, self.header.c140_clock);
        // K053260 (0xAC)
        write_u32(&mut buf, 0xAC, self.header.k053260_clock);
        // Pokey (0xB0)
        write_u32(&mut buf, 0xB0, self.header.pokey_clock);
        // QSound (0xB4)
        write_u32(&mut buf, 0xB4, self.header.qsound_clock);
        // SCSP (0xB8)
        write_u32(&mut buf, 0xB8, self.header.scsp_clock);
        // Extra header offset (0xBC)
        write_u32(&mut buf, 0xBC, self.header.extra_header_offset);
        // WonderSwan (0xC0)
        write_u32(&mut buf, 0xC0, self.header.wonderswan_clock);
        // VSU (0xC4)
        write_u32(&mut buf, 0xC4, self.header.vsu_clock);
        // SAA1099 (0xC8)
        write_u32(&mut buf, 0xC8, self.header.saa1099_clock);
        // ES5503 (0xCC)
        write_u32(&mut buf, 0xCC, self.header.es5503_clock);
        // ES5506 (0xD0)
        write_u32(&mut buf, 0xD0, self.header.es5506_clock);
        write_u16(&mut buf, 0xD4, self.header.es5506_channels);
        write_u8(&mut buf, 0xD6, self.header.es5506_cd);
        write_u8(&mut buf, 0xD7, self.header.es5506_reserved);
        // X1-010 (0xD8)
        write_u32(&mut buf, 0xD8, self.header.x1_010_clock);
        // C352 (0xDC)
        write_u32(&mut buf, 0xDC, self.header.c352_clock);
        // GA20 (0xE0)
        write_u32(&mut buf, 0xE0, self.header.ga20_clock);
        // Mikey (0xE4)
        write_u32(&mut buf, 0xE4, self.header.mikey_clock);
        // reserved (0xE8..0xEF)
        write_slice(&mut buf, 0xE8, &self.header.reserved_e8_ef);
        // reserved (0xF0..0xFF)
        write_slice(&mut buf, 0xF0, &self.header.reserved_f0_ff);

        buf.extend_from_slice(&cmd_buf);
        if !wrote_end_in_cmds {
            buf.push(0x66u8);
        }

        let total_samples: u32 = if total_samples_u64 > (u32::MAX as u64) {
            u32::MAX
        } else {
            total_samples_u64 as u32
        };
        write_u32(&mut buf, 0x18, total_samples);

        if let Some(gd3) = &self.gd3 {
            let gd3_start = buf.len() as u32;
            let gd3_offset_val = gd3_start.wrapping_sub(0x14u32);

            buf.extend_from_slice(b"Gd3 ");
            buf.extend_from_slice(&0x00000100u32.to_le_bytes()); // version 1.00
            buf.extend_from_slice(&0_u32.to_le_bytes()); // placeholder for length

            let fields: [&Option<String>; 11] = [
                &gd3.track_name_en,
                &gd3.track_name_jp,
                &gd3.game_name_en,
                &gd3.game_name_jp,
                &gd3.system_name_en,
                &gd3.system_name_jp,
                &gd3.author_name_en,
                &gd3.author_name_jp,
                &gd3.release_date,
                &gd3.creator,
                &gd3.notes,
            ];

            let mut gd3_data: Vec<u8> = Vec::new();
            for f in &fields {
                if let Some(s) = f {
                    for code in s.encode_utf16() {
                        gd3_data.extend_from_slice(&code.to_le_bytes());
                    }
                }
                gd3_data.extend_from_slice(&0u16.to_le_bytes());
            }

            let gd3_len = gd3_data.len() as u32;
            buf.extend_from_slice(&gd3_data);

            let len_pos = gd3_start as usize + 8;
            let len_bytes = gd3_len.to_le_bytes();
            buf[len_pos..len_pos + 4].copy_from_slice(&len_bytes);

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
