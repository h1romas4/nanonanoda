use soundlog::chip::*;
use soundlog::vgm::command::{
    Ay8910StereoMask, ChipId, DataBlock, PcmRamWrite, SeekOffset, SetStreamFrequency,
    SetupStreamControl, StartStream, StartStreamFastCall, StopStream, VgmCommand, Wait735Samples,
    Wait882Samples, WaitNSample, WaitSamples, Ym2612Port0Address2AWriteAndWaitN,
};
use soundlog::{VgmBuilder, VgmDocument, VgmHeader};

#[test]
fn build_minimal_vgmdocument() {
    // Build an empty/default VGM document using the builder API.
    let doc: VgmDocument = VgmBuilder::new().finalize();
    // Header defaults are set and commands are empty.
    assert_eq!(doc.iter().count(), 0);
    assert_eq!(doc.header, VgmHeader::default());
}

#[test]
fn test_total_samples_computed_correctly() {
    // build vgm
    let mut b = VgmBuilder::new();
    b.add_vgm_command(WaitSamples(100)); // 100
    b.add_vgm_command(Wait735Samples); // 735
    b.add_vgm_command(Wait882Samples); // 882
    b.add_vgm_command(WaitNSample(5)); // 5
    b.add_vgm_command(Ym2612Port0Address2AWriteAndWaitN(3)); // 3
    let doc = b.finalize();

    // test re-parse
    let bytes: Vec<u8> = (&doc).into();
    let parsed: VgmDocument = (bytes.as_slice())
        .try_into()
        .expect("failed to parse serialized VGM");

    // compute total samples manually
    let computed_total: u32 = parsed
        .iter()
        .map(|cmd| match cmd {
            VgmCommand::WaitSamples(s) => s.0 as u32,
            VgmCommand::Wait735Samples(_) => 735,
            VgmCommand::Wait882Samples(_) => 882,
            VgmCommand::WaitNSample(s) => s.0 as u32,
            VgmCommand::YM2612Port0Address2AWriteAndWaitN(s) => s.0 as u32,
            _ => 0,
        })
        .sum();

    assert_eq!(computed_total, 1725u32);
    assert_eq!(parsed.total_samples(), computed_total);
}

// Tests using `add_chip_write` to ensure `Into<chip::Chip>` conversions work
#[test]
fn add_chip_write_psg() {
    let mut b = VgmBuilder::new();
    b.add_chip_write(0usize, PsgSpec { value: 0xAB });
    let doc = b.finalize();
    assert_eq!(doc.iter().count(), 1);
    match doc.iter().next().unwrap().clone() {
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
    assert_eq!(doc.iter().count(), 1);
    match doc.iter().next().unwrap().clone() {
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
    assert_eq!(doc.iter().count(), 2);
    match doc.iter().next().unwrap().clone() {
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
    match doc.iter().nth(1).unwrap().clone() {
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
    assert_eq!(doc.iter().count(), 1);
    match doc.iter().next().unwrap().clone() {
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
    assert_eq!(doc.iter().count(), 1);
    match doc.iter().next().unwrap().clone() {
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
    assert_eq!(doc.iter().count(), 1);
    match doc.iter().next().unwrap().clone() {
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
    assert_eq!(doc.iter().count(), 1);
    match doc.iter().next().unwrap().clone() {
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
    assert_eq!(doc.iter().count(), 1);
    match doc.iter().next().unwrap().clone() {
        VgmCommand::PcmRamWrite(s) => assert_eq!(s, spec),
        other => panic!("unexpected command: {:?}", other),
    }
}

#[test]
fn add_command_wait_n_sample() {
    let mut b = VgmBuilder::new();
    b.add_vgm_command(WaitNSample(5));
    let doc = b.finalize();
    assert_eq!(doc.iter().count(), 1);
    match doc.iter().next().unwrap().clone() {
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
    assert_eq!(doc.iter().count(), 2);
    match doc.iter().next().unwrap().clone() {
        VgmCommand::AY8910StereoMask(s) => assert_eq!(s, Ay8910StereoMask(0xAA)),
        other => panic!("unexpected command: {:?}", other),
    }
    match doc.iter().nth(1).unwrap().clone() {
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
    assert_eq!(doc.iter().count(), 2);
    match doc.iter().next().unwrap().clone() {
        VgmCommand::SetupStreamControl(s) => assert_eq!(s, setup),
        other => panic!("unexpected command: {:?}", other),
    }
    match doc.iter().nth(1).unwrap().clone() {
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
    assert_eq!(doc.iter().count(), 3);
    match doc.iter().next().unwrap().clone() {
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
    match doc.iter().nth(1).unwrap().clone() {
        VgmCommand::StopStream(s) => assert_eq!(s, StopStream { stream_id: 7 }),
        other => panic!("unexpected: {:?}", other),
    }
    match doc.iter().nth(2).unwrap().clone() {
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
    assert_eq!(doc.iter().count(), 1);
    match doc.iter().next().unwrap().clone() {
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
    assert_eq!(doc.iter().count(), 1);
    match doc.iter().next().unwrap().clone() {
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
    assert_eq!(doc.iter().count(), 1);
    match doc.iter().next().unwrap().clone() {
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

#[test]
fn roundtrip_vgmdocument_into_vec_and_parse() {
    // Build a small document with a registered chip and a couple commands.
    let mut b = VgmBuilder::new();
    b.register_chip(Chip::Ym2612, 0, 7987200);
    b.add_chip_write(
        0usize,
        Ym2612Spec {
            port: 0,
            register: 0x2A,
            value: 0x77,
        },
    );
    b.add_vgm_command(WaitSamples(0x1234));

    let doc = b.finalize();

    let bytes: Vec<u8> = (&doc).into();

    let parsed: VgmDocument = (bytes.as_slice())
        .try_into()
        .expect("failed to parse serialized VGM");

    let mut parsed_commands: Vec<VgmCommand> = parsed.iter().cloned().collect();
    if let Some(c) = parsed_commands.last()
        && matches!(c, VgmCommand::EndOfData(_))
    {
        parsed_commands.pop();
    }
    assert_eq!(parsed_commands, doc.iter().cloned().collect::<Vec<_>>());
    assert_eq!(parsed.gd3, doc.gd3);
    assert_eq!(parsed.header.ident, doc.header.ident);
    assert_eq!(parsed.header.version, doc.header.version);
    assert_eq!(parsed.header.total_samples, doc.header.total_samples);
    assert_eq!(parsed.header.sample_rate, doc.header.sample_rate);
    assert_eq!(parsed.header.ym2612_clock, doc.header.ym2612_clock);
}

#[test]
fn iterate_vgmdocument_by_ref_mut_and_value() {
    // Build a document with a couple commands
    let mut b = VgmBuilder::new();
    b.add_vgm_command(WaitSamples(0x10));
    b.add_vgm_command(SeekOffset(0x1234));
    let mut doc = b.finalize();

    // Iterate by reference
    let mut collected_ref: Vec<VgmCommand> = Vec::new();
    for c in &doc {
        collected_ref.push(c.clone());
    }
    assert_eq!(collected_ref, doc.iter().cloned().collect::<Vec<_>>());

    let first_before = doc.iter().next().unwrap().clone();
    if let Some(c) = doc.iter_mut().next() {
        *c = WaitSamples(0xFFFF).into();
    }
    assert_ne!(doc.iter().next().unwrap(), &first_before);
    assert_eq!(doc.iter().next().unwrap(), &WaitSamples(0xFFFF).into());

    let expected: Vec<VgmCommand> = doc.iter().cloned().collect();
    let consumed: Vec<VgmCommand> = doc.into_iter().collect();
    assert_eq!(consumed, expected);
}

#[test]
fn test_create_and_parse_vgm_document() {
    let mut builder = VgmBuilder::new();

    builder.add_chip_write(
        0,
        Ym2203Spec {
            register: 0x22,
            value: 0x33,
        },
    );
    builder.add_chip_write(
        1,
        Ym2203Spec {
            register: 0x22,
            value: 0x33,
        },
    );
    builder.add_chip_write(
        1,
        Ymf262Spec {
            port: 0,
            register: 0x22,
            value: 0x33,
        },
    );
    builder.add_vgm_command(WaitNSample(20));

    let vgm: VgmDocument = builder.finalize();

    vgm.into_iter().for_each(|cmd| {
        println!("{:?}", cmd);
    });
}
