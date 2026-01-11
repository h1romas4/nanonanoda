use soundlog::chip::*;
use soundlog::vgm::command::{
    Ay8910StereoMask, ChipId, CommandSpec, DataBlock, PcmRamWrite, SeekOffset, SetStreamFrequency,
    SetupStreamControl, StartStream, StartStreamFastCall, StopStream, VgmCommand, WaitNSample,
    WaitSamples, Ym2612Port0Address2AWriteAndWaitN,
};
use soundlog::{VgmBuilder, VgmDocument, VgmHeader};

#[test]
fn build_minimal_vgmdocument() {
    // Build an empty/default VGM document using the builder API.
    let doc: VgmDocument = VgmBuilder::new().finalize();
    // Header defaults are set and commands are empty.
    assert_eq!(doc.commands.len(), 0);
    assert_eq!(doc.header, VgmHeader::default());
}

// Per-Spec decode_vgm_bytes tests (統合)
#[test]
fn spec_decode_vgm_bytes_all() {
    // PSG
    {
        let s = PsgSpec { value: 0xAA };
        let mut buf = Vec::new();
        s.to_vgm_bytes(&mut buf);
        assert_eq!(buf, vec![0x50, 0xAA]);
    }

    // YM2413
    {
        let s = Ym2413Spec {
            register: 0x10,
            value: 0x22,
        };
        let mut buf = Vec::new();
        s.to_vgm_bytes(&mut buf);
        assert_eq!(buf, vec![0x51, 0x10, 0x22]);
    }

    // YM2612 ports
    {
        let s0 = Ym2612Spec {
            port: 0,
            register: 0x2A,
            value: 0x55,
        };
        let s1 = Ym2612Spec {
            port: 1,
            register: 0x2A,
            value: 0x66,
        };
        let mut b0 = Vec::new();
        let mut b1 = Vec::new();
        s0.to_vgm_bytes(&mut b0);
        s1.to_vgm_bytes(&mut b1);
        assert_eq!(b0, vec![0x52, 0x2A, 0x55]);
        assert_eq!(b1, vec![0x53, 0x2A, 0x66]);
    }

    // YM2151
    {
        let s = Ym2151Spec {
            register: 0x01,
            value: 0x02,
        };
        let mut buf = Vec::new();
        s.to_vgm_bytes(&mut buf);
        assert_eq!(buf, vec![0x54, 0x01, 0x02]);
    }

    // Sega PCM
    {
        let s = SegaPcmSpec {
            offset: 0x1234,
            value: 0x77,
        };
        let mut buf = Vec::new();
        s.to_vgm_bytes(&mut buf);
        assert_eq!(buf, vec![0xC0, 0x12, 0x34, 0x77]);
    }

    // RF5C68
    {
        let s = Rf5c68Spec {
            offset: 0x0201,
            value: 0x88,
        };
        let mut buf = Vec::new();
        s.to_vgm_bytes(&mut buf);
        assert_eq!(buf, vec![0xC1, 0x02, 0x01, 0x88]);
    }

    // YM2203
    {
        let s = Ym2203Spec {
            register: 0x05,
            value: 0x06,
        };
        let mut buf = Vec::new();
        s.to_vgm_bytes(&mut buf);
        assert_eq!(buf, vec![0x55, 0x05, 0x06]);
    }

    // YM2608 ports
    {
        let s0 = Ym2608Spec {
            port: 0,
            register: 0x07,
            value: 0x08,
        };
        let s1 = Ym2608Spec {
            port: 1,
            register: 0x07,
            value: 0x09,
        };
        let mut b0 = Vec::new();
        let mut b1 = Vec::new();
        s0.to_vgm_bytes(&mut b0);
        s1.to_vgm_bytes(&mut b1);
        assert_eq!(b0, vec![0x56, 0x07, 0x08]);
        assert_eq!(b1, vec![0x57, 0x07, 0x09]);
    }

    // YM2610 ports
    {
        let s0 = Ym2610Spec {
            port: 0,
            register: 0x0A,
            value: 0x0B,
        };
        let s1 = Ym2610Spec {
            port: 1,
            register: 0x0A,
            value: 0x0C,
        };
        let mut b0 = Vec::new();
        let mut b1 = Vec::new();
        s0.to_vgm_bytes(&mut b0);
        s1.to_vgm_bytes(&mut b1);
        assert_eq!(b0, vec![0x58, 0x0A, 0x0B]);
        assert_eq!(b1, vec![0x59, 0x0A, 0x0C]);
    }

    // YM3812
    {
        let s = Ym3812Spec {
            register: 0x11,
            value: 0x22,
        };
        let mut buf = Vec::new();
        s.to_vgm_bytes(&mut buf);
        assert_eq!(buf, vec![0x5A, 0x11, 0x22]);
    }

    // YM3526
    {
        let s = Ym3526Spec {
            register: 0x12,
            value: 0x23,
        };
        let mut buf = Vec::new();
        s.to_vgm_bytes(&mut buf);
        assert_eq!(buf, vec![0x5B, 0x12, 0x23]);
    }

    // Y8950
    {
        let s = Y8950Spec {
            register: 0x13,
            value: 0x24,
        };
        let mut buf = Vec::new();
        s.to_vgm_bytes(&mut buf);
        assert_eq!(buf, vec![0x5C, 0x13, 0x24]);
    }

    // YMF262 ports
    {
        let s0 = Ymf262Spec {
            port: 0,
            register: 0x20,
            value: 0x21,
        };
        let s1 = Ymf262Spec {
            port: 1,
            register: 0x20,
            value: 0x22,
        };
        let mut b0 = Vec::new();
        let mut b1 = Vec::new();
        s0.to_vgm_bytes(&mut b0);
        s1.to_vgm_bytes(&mut b1);
        assert_eq!(b0, vec![0x5E, 0x20, 0x21]);
        assert_eq!(b1, vec![0x5F, 0x20, 0x22]);
    }

    // YMF278B
    {
        let s = Ymf278bSpec {
            port: 0x01,
            register: 0x02,
            value: 0x03,
        };
        let mut buf = Vec::new();
        s.to_vgm_bytes(&mut buf);
        assert_eq!(buf, vec![0xD0, 0x01, 0x02, 0x03]);
    }

    // YMF271
    {
        let s = Ymf271Spec {
            port: 0x02,
            register: 0x03,
            value: 0x04,
        };
        let mut buf = Vec::new();
        s.to_vgm_bytes(&mut buf);
        assert_eq!(buf, vec![0xD1, 0x02, 0x03, 0x04]);
    }

    // YMZ280B
    {
        let s = Ymz280bSpec {
            register: 0x30,
            value: 0x31,
        };
        let mut buf = Vec::new();
        s.to_vgm_bytes(&mut buf);
        assert_eq!(buf, vec![0x5D, 0x30, 0x31]);
    }

    // RF5C164
    {
        let s = Rf5c164Spec {
            register: 0x40,
            value: 0x41,
        };
        let mut buf = Vec::new();
        s.to_vgm_bytes(&mut buf);
        assert_eq!(buf, vec![0xB1, 0x40, 0x41]);
    }

    // PWM
    {
        let s = PwmSpec {
            register: 0x01,
            value: 0x00112233,
        };
        let mut buf = Vec::new();
        s.to_vgm_bytes(&mut buf);
        assert_eq!(buf, vec![0xB2, 0x01, 0x11, 0x22, 0x33]);
    }

    // AY8910
    {
        let s = Ay8910Spec {
            register: 0x02,
            value: 0x03,
        };
        let mut buf = Vec::new();
        s.to_vgm_bytes(&mut buf);
        assert_eq!(buf, vec![0xA0, 0x02, 0x03]);
    }

    // GB DMG
    {
        let s = GbDmgSpec {
            register: 0x04,
            value: 0x05,
        };
        let mut buf = Vec::new();
        s.to_vgm_bytes(&mut buf);
        assert_eq!(buf, vec![0xB3, 0x04, 0x05]);
    }

    // NES APU
    {
        let s = NesApuSpec {
            register: 0x06,
            value: 0x07,
        };
        let mut buf = Vec::new();
        s.to_vgm_bytes(&mut buf);
        assert_eq!(buf, vec![0xB4, 0x06, 0x07]);
    }

    // MultiPCM
    {
        let s = MultiPcmSpec {
            register: 0x08,
            value: 0x09,
        };
        let mut buf = Vec::new();
        s.to_vgm_bytes(&mut buf);
        assert_eq!(buf, vec![0xB5, 0x08, 0x09]);
    }

    // UPD7759
    {
        let s = Upd7759Spec {
            register: 0x0A,
            value: 0x0B,
        };
        let mut buf = Vec::new();
        s.to_vgm_bytes(&mut buf);
        assert_eq!(buf, vec![0xB6, 0x0A, 0x0B]);
    }

    // OKIM6258
    {
        let s = Okim6258Spec {
            register: 0x0C,
            value: 0x0D,
        };
        let mut buf = Vec::new();
        s.to_vgm_bytes(&mut buf);
        assert_eq!(buf, vec![0xB7, 0x0C, 0x0D]);
    }

    // OKIM6295
    {
        let s = Okim6295Spec {
            register: 0x0E,
            value: 0x0F,
        };
        let mut buf = Vec::new();
        s.to_vgm_bytes(&mut buf);
        assert_eq!(buf, vec![0xB8, 0x0E, 0x0F]);
    }

    // K051649 (expected to panic/unimplemented)
    {
        let result = std::panic::catch_unwind(|| {
            let s = K051649Spec {
                register: 0x1234,
                value: 0x11,
            };
            let mut buf = Vec::new();
            s.to_vgm_bytes(&mut buf);
        });
        assert!(result.is_err(), "K051649Spec should panic/unimplemented");
    }

    // K054539
    {
        let s = K054539Spec {
            register: 0x1122,
            value: 0x33,
        };
        let mut buf = Vec::new();
        s.to_vgm_bytes(&mut buf);
        assert_eq!(buf, vec![0xD3, 0x11, 0x22, 0x33]);
    }

    // HUC6280
    {
        let s = Huc6280Spec {
            register: 0x12,
            value: 0x13,
        };
        let mut buf = Vec::new();
        s.to_vgm_bytes(&mut buf);
        assert_eq!(buf, vec![0xB9, 0x12, 0x13]);
    }

    // C140
    {
        let s = C140Spec {
            register: 0x3344,
            value: 0x55,
        };
        let mut buf = Vec::new();
        s.to_vgm_bytes(&mut buf);
        assert_eq!(buf, vec![0xD4, 0x33, 0x44, 0x55]);
    }

    // K053260
    {
        let s = K053260Spec {
            register: 0x14,
            value: 0x15,
        };
        let mut buf = Vec::new();
        s.to_vgm_bytes(&mut buf);
        assert_eq!(buf, vec![0xBA, 0x14, 0x15]);
    }

    // POKEY
    {
        let s = PokeySpec {
            register: 0x16,
            value: 0x17,
        };
        let mut buf = Vec::new();
        s.to_vgm_bytes(&mut buf);
        assert_eq!(buf, vec![0xBB, 0x16, 0x17]);
    }

    // QSound
    {
        let s = QsoundSpec {
            register: 0x21,
            value: 0x1234,
        };
        let mut buf = Vec::new();
        s.to_vgm_bytes(&mut buf);
        assert_eq!(buf, vec![0xC4, 0x21, 0x12, 0x34]);
    }

    // SCSP
    {
        let s = ScspSpec {
            offset: 0x5566,
            value: 0x77,
        };
        let mut buf = Vec::new();
        s.to_vgm_bytes(&mut buf);
        assert_eq!(buf, vec![0xC5, 0x55, 0x66, 0x77]);
    }

    // WonderSwan
    {
        let s = WonderSwanSpec {
            offset: 0x2233,
            value: 0x44,
        };
        let mut buf = Vec::new();
        s.to_vgm_bytes(&mut buf);
        assert_eq!(buf, vec![0xC6, 0x22, 0x33, 0x44]);
    }

    // VSU
    {
        let s = VsuSpec {
            offset: 0x4455,
            value: 0x66,
        };
        let mut buf = Vec::new();
        s.to_vgm_bytes(&mut buf);
        assert_eq!(buf, vec![0xC7, 0x44, 0x55, 0x66]);
    }

    // SAA1099
    {
        let s = Saa1099Spec {
            register: 0x18,
            value: 0x19,
        };
        let mut buf = Vec::new();
        s.to_vgm_bytes(&mut buf);
        assert_eq!(buf, vec![0xBD, 0x18, 0x19]);
    }

    // ES5503
    {
        let s = Es5503Spec {
            register: 0x8899,
            value: 0xAA,
        };
        let mut buf = Vec::new();
        s.to_vgm_bytes(&mut buf);
        assert_eq!(buf, vec![0xD5, 0x88, 0x99, 0xAA]);
    }

    // ES5506
    {
        let s = Es5506v16Spec {
            register: 0x1A,
            value: 0xBEEF,
        };
        let mut buf = Vec::new();
        s.to_vgm_bytes(&mut buf);
        assert_eq!(buf, vec![0xD6, 0x1A, 0xBE, 0xEF]);
    }

    // X1010
    {
        let s = X1010Spec {
            offset: 0x7788,
            value: 0x99,
        };
        let mut buf = Vec::new();
        s.to_vgm_bytes(&mut buf);
        assert_eq!(buf, vec![0xC8, 0x77, 0x88, 0x99]);
    }

    // C352
    {
        let s = C352Spec {
            register: 0x3344,
            value: 0xCAFE,
        };
        let mut buf = Vec::new();
        s.to_vgm_bytes(&mut buf);
        assert_eq!(buf, vec![0xE1, 0x33, 0x44, 0xCA, 0xFE]);
    }

    // GA20
    {
        let s = Ga20Spec {
            register: 0x1B,
            value: 0x1C,
        };
        let mut buf = Vec::new();
        s.to_vgm_bytes(&mut buf);
        assert_eq!(buf, vec![0xBF, 0x1B, 0x1C]);
    }

    // Mikey
    {
        let s = MikeySpec {
            register: 0x1D,
            value: 0x1E,
        };
        let mut buf = Vec::new();
        s.to_vgm_bytes(&mut buf);
        assert_eq!(buf, vec![0x40, 0x1D, 0x1E]);
    }

    // GameGear PSG
    {
        let s = GameGearPsgSpec { value: 0x7F };
        let mut buf = Vec::new();
        s.to_vgm_bytes(&mut buf);
        assert_eq!(buf, vec![0x4F, 0x7F]);
    }
}

// Tests using `add_chip_write` to ensure `Into<chip::Chip>` conversions work
#[test]
fn add_chip_write_psg() {
    let mut b = VgmBuilder::new();
    b.add_chip_write(0usize, PsgSpec { value: 0xAB });
    let doc = b.finalize();
    assert_eq!(doc.commands.len(), 1);
    match doc.commands[0].clone() {
        VgmCommand::Sn76489Write(id, s) => {
            assert_eq!(usize::from(id), 0usize);
            assert_eq!(s, PsgSpec { value: 0xAB });
        }
        other => panic!("unexpected command: {:?}", other),
    }
}

#[test]
fn add_chip_write_ym2413() {
    let mut b = VgmBuilder::new();
    b.add_chip_write(
        1usize,
        Ym2413Spec {
            register: 0x10,
            value: 0x22,
        },
    );
    let doc = b.finalize();
    assert_eq!(doc.commands.len(), 1);
    match doc.commands[0].clone() {
        VgmCommand::Ym2413Write(id, s) => {
            assert_eq!(usize::from(id), 1usize);
            assert_eq!(
                s,
                Ym2413Spec {
                    register: 0x10,
                    value: 0x22
                }
            );
        }
        other => panic!("unexpected command: {:?}", other),
    }
}

#[test]
fn add_chip_write_ym2612_ports() {
    let mut b = VgmBuilder::new();
    b.add_chip_write(
        ChipId::Secondary,
        Ym2612Spec {
            port: 0,
            register: 0x2A,
            value: 0x55,
        },
    );
    b.add_chip_write(
        ChipId::Secondary,
        Ym2612Spec {
            port: 1,
            register: 0x2A,
            value: 0x66,
        },
    );
    let doc = b.finalize();
    assert_eq!(doc.commands.len(), 2);
    match doc.commands[0].clone() {
        VgmCommand::Ym2612Write(id, s) => {
            assert_eq!(usize::from(id), 1usize);
            assert_eq!(
                s,
                Ym2612Spec {
                    port: 0,
                    register: 0x2A,
                    value: 0x55
                }
            );
        }
        other => panic!("unexpected: {:?}", other),
    }
    match doc.commands[1].clone() {
        VgmCommand::Ym2612Write(id, s) => {
            assert_eq!(usize::from(id), 1usize);
            assert_eq!(
                s,
                Ym2612Spec {
                    port: 1,
                    register: 0x2A,
                    value: 0x66
                }
            );
        }
        other => panic!("unexpected: {:?}", other),
    }
}

#[test]
fn add_chip_write_pwm() {
    let mut b = VgmBuilder::new();
    b.add_chip_write(
        ChipId::Secondary,
        PwmSpec {
            register: 0x01,
            value: 0x0000_FFEE,
        },
    );
    let doc = b.finalize();
    assert_eq!(doc.commands.len(), 1);
    match doc.commands[0].clone() {
        VgmCommand::PwmWrite(id, s) => {
            assert_eq!(usize::from(id), 1usize);
            assert_eq!(
                s,
                PwmSpec {
                    register: 0x01,
                    value: 0x0000_FFEE
                }
            );
        }
        other => panic!("unexpected: {:?}", other),
    }
}

#[test]
fn add_chip_write_okim6295() {
    let mut b = VgmBuilder::new();
    b.add_chip_write(
        ChipId::Secondary,
        Okim6295Spec {
            register: 0x0F,
            value: 0x10,
        },
    );
    let doc = b.finalize();
    assert_eq!(doc.commands.len(), 1);
    match doc.commands[0].clone() {
        VgmCommand::Okim6295Write(id, s) => {
            assert_eq!(usize::from(id), 1usize);
            assert_eq!(
                s,
                Okim6295Spec {
                    register: 0x0F,
                    value: 0x10
                }
            );
        }
        other => panic!("unexpected: {:?}", other),
    }
}

#[test]
fn add_command_wait_samples() {
    let mut b = VgmBuilder::new();
    b.add_vgm_command(WaitSamples(0x1234));
    let doc = b.finalize();
    assert_eq!(doc.commands.len(), 1);
    match doc.commands[0].clone() {
        VgmCommand::WaitSamples(s) => assert_eq!(s, WaitSamples(0x1234)),
        other => panic!("unexpected command: {:?}", other),
    }
}

#[test]
fn add_command_data_block() {
    let mut b = VgmBuilder::new();
    let data = vec![1u8, 2, 3];
    let spec = DataBlock {
        data_type: 0x01,
        size: data.len() as u32,
        data: data.clone(),
    };
    b.add_vgm_command(spec.clone());
    let doc = b.finalize();
    assert_eq!(doc.commands.len(), 1);
    match doc.commands[0].clone() {
        VgmCommand::DataBlock(s) => assert_eq!(s.data, data),
        other => panic!("unexpected command: {:?}", other),
    }
}

#[test]
fn add_command_pcm_ram_write() {
    let mut b = VgmBuilder::new();
    let spec = PcmRamWrite {
        chip_type: 0x66,
        read_offset: 0x010203,
        write_offset: 0x030201,
        size: 3,
        data: vec![4, 5, 6],
    };
    b.add_vgm_command(spec.clone());
    let doc = b.finalize();
    assert_eq!(doc.commands.len(), 1);
    match doc.commands[0].clone() {
        VgmCommand::PcmRamWrite(s) => assert_eq!(s, spec),
        other => panic!("unexpected command: {:?}", other),
    }
}

#[test]
fn add_command_wait_n_sample() {
    let mut b = VgmBuilder::new();
    b.add_vgm_command(WaitNSample(5));
    let doc = b.finalize();
    assert_eq!(doc.commands.len(), 1);
    match doc.commands[0].clone() {
        VgmCommand::WaitNSample(s) => assert_eq!(s, WaitNSample(5)),
        other => panic!("unexpected command: {:?}", other),
    }
}

#[test]
fn add_command_ay8910_mask_and_seek() {
    let mut b = VgmBuilder::new();
    b.add_vgm_command(Ay8910StereoMask(0xAA));
    b.add_vgm_command(SeekOffset(0xDEADBEEF));
    let doc = b.finalize();
    assert_eq!(doc.commands.len(), 2);
    match doc.commands[0].clone() {
        VgmCommand::AY8910StereoMask(s) => assert_eq!(s, Ay8910StereoMask(0xAA)),
        other => panic!("unexpected command: {:?}", other),
    }
    match doc.commands[1].clone() {
        VgmCommand::SeekOffset(s) => assert_eq!(s, SeekOffset(0xDEADBEEF)),
        other => panic!("unexpected command: {:?}", other),
    }
}

#[test]
fn add_command_stream_controls() {
    let mut b = VgmBuilder::new();
    let setup = SetupStreamControl {
        stream_id: 1,
        chip_type: 2,
        write_port: 3,
        write_command: 4,
    };
    let freq = SetStreamFrequency {
        stream_id: 1,
        frequency: 0x11223344,
    };
    b.add_vgm_command(setup.clone());
    b.add_vgm_command(freq.clone());
    let doc = b.finalize();
    assert_eq!(doc.commands.len(), 2);
    match doc.commands[0].clone() {
        VgmCommand::SetupStreamControl(s) => assert_eq!(s, setup),
        other => panic!("unexpected command: {:?}", other),
    }
    match doc.commands[1].clone() {
        VgmCommand::SetStreamFrequency(s) => assert_eq!(s, freq),
        other => panic!("unexpected command: {:?}", other),
    }
}

#[test]
fn add_command_start_stop_and_fastcall() {
    let mut b = VgmBuilder::new();
    b.add_vgm_command(StartStream {
        stream_id: 7,
        data_start_offset: -1,
        length_mode: 0,
        data_length: 0,
    });
    b.add_vgm_command(StopStream { stream_id: 7 });
    b.add_vgm_command(StartStreamFastCall {
        stream_id: 8,
        block_id: 0x1234,
        flags: 9,
    });
    let doc = b.finalize();
    assert_eq!(doc.commands.len(), 3);
    match doc.commands[0].clone() {
        VgmCommand::StartStream(s) => assert_eq!(
            s,
            StartStream {
                stream_id: 7,
                data_start_offset: -1,
                length_mode: 0,
                data_length: 0
            }
        ),
        other => panic!("unexpected: {:?}", other),
    }
    match doc.commands[1].clone() {
        VgmCommand::StopStream(s) => assert_eq!(s, StopStream { stream_id: 7 }),
        other => panic!("unexpected: {:?}", other),
    }
    match doc.commands[2].clone() {
        VgmCommand::StartStreamFastCall(s) => assert_eq!(
            s,
            StartStreamFastCall {
                stream_id: 8,
                block_id: 0x1234,
                flags: 9
            }
        ),
        other => panic!("unexpected: {:?}", other),
    }
}

#[test]
fn add_command_ym2612_port0_address2a() {
    let mut b = VgmBuilder::new();
    b.add_vgm_command(Ym2612Port0Address2AWriteAndWaitN(3));
    let doc = b.finalize();
    assert_eq!(doc.commands.len(), 1);
    match doc.commands[0].clone() {
        VgmCommand::YM2612Port0Address2AWriteAndWaitN(s) => {
            assert_eq!(s, Ym2612Port0Address2AWriteAndWaitN(3))
        }
        other => panic!("unexpected: {:?}", other),
    }
}

#[test]
fn add_chip_registers_and_sets_header_clock() {
    let mut b = VgmBuilder::new();
    // register a YM2413 instance at id 0
    b.register_chip(Chip::Ym2413, 0, 3579545);
    let doc = b.finalize();
    assert_eq!(doc.header.ym2413_clock, 3579545);
}

#[test]
fn add_chip_sets_msb_for_instance1() {
    let mut b = VgmBuilder::new();
    // chip_id 1 should set MSB of the clock field
    b.register_chip(Chip::Ym2413, 1, 3579545);
    let doc = b.finalize();
    assert_eq!(doc.header.ym2413_clock, 3579545u32 | 0x8000_0000u32);
}

#[test]
fn add_chip_write_uses_registered_instance() {
    let mut b = VgmBuilder::new();
    b.register_chip(Chip::Ym2612, 0, 7987200);
    b.add_chip_write(
        0,
        Ym2612Spec {
            port: 0,
            register: 0x2A,
            value: 0x77,
        },
    );
    let doc = b.finalize();
    assert_eq!(doc.commands.len(), 1);
    match doc.commands[0].clone() {
        VgmCommand::Ym2612Write(i, s) => {
            assert_eq!(usize::from(i), 0usize);
            assert_eq!(
                s,
                Ym2612Spec {
                    port: 0,
                    register: 0x2A,
                    value: 0x77
                }
            );
        }
        other => panic!("unexpected command: {:?}", other),
    }
}

#[test]
fn scc1_write() {
    let s = Scc1Spec {
        port: 0x05,
        register: 0x06,
        value: 0x07,
    };
    let mut buf = Vec::new();
    s.to_vgm_bytes(&mut buf);
    assert_eq!(buf, vec![0xD2, 0x05, 0x06, 0x07]);
}

#[test]
fn add_chip_write_scc1() {
    let mut b = VgmBuilder::new();
    b.add_chip_write(
        ChipId::Secondary,
        Scc1Spec {
            port: 0x05,
            register: 0x06,
            value: 0x07,
        },
    );
    let doc = b.finalize();
    assert_eq!(doc.commands.len(), 1);
    match doc.commands[0].clone() {
        VgmCommand::Scc1Write(id, s) => {
            assert_eq!(usize::from(id), 1usize);
            assert_eq!(
                s,
                Scc1Spec {
                    port: 0x05,
                    register: 0x06,
                    value: 0x07
                }
            );
        }
        other => panic!("unexpected: {:?}", other),
    }
}
