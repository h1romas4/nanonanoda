use nanonanoda::ym::{
    init_ym2203_channel_and_op, init_ymf262, init_ymf262_channel_and_op, ym2203_keyon, ymf262_keyon,
};
use soundlog::chip::Chip;
use soundlog::chip::fnumber::{
    ChipTypeSpec, Opl3Spec, OpnSpec, find_and_tune_fnumber, generate_12edo_fnum_table,
};
use soundlog::meta::Gd3;
use soundlog::{EndOfData, Instance, VgmBuilder, VgmDocument, WaitSamples};
use std::fs::{File, create_dir_all};
use std::io::Write;

/// Helper: write VGM document to disk using the soundlog conversion.
fn write_doc_to(path: &str, doc: VgmDocument) -> std::io::Result<usize> {
    let bytes = Vec::from(&doc);
    let mut f = File::create(path)?;
    f.write_all(&bytes)?;
    Ok(bytes.len())
}

/// Helper: add WaitSamples commands splitting into u16 chunks.
fn add_wait_samples(b: &mut VgmBuilder, mut samples: u32) {
    while samples > 0 {
        let this = if samples > 0xFFFF {
            0xFFFFu16
        } else {
            samples as u16
        };
        b.add_vgm_command(WaitSamples(this));
        samples = samples.saturating_sub(this as u32);
    }
}

fn build_opl3_2op_4ch_ym() -> VgmDocument {
    let mut b = VgmBuilder::new();

    // GD3 metadata
    let gd3 = Gd3 {
        track_name_en: Some("OPL3 2op 4ch (via ym.rs)".to_string()),
        game_name_en: Some("nanonanoda examples".to_string()),
        ..Default::default()
    };
    b.set_gd3(gd3);

    // Register chip and clock (primary instance)
    let master = Opl3Spec::default_master_clock();
    b.register_chip(Chip::Ymf262, Instance::Primary, master as u32);

    // prepare fnumber table and tune A4
    let fnum_table = generate_12edo_fnum_table::<Opl3Spec>(master).expect("fnum table");
    let fnum = find_and_tune_fnumber::<Opl3Spec>(&fnum_table, 440.0, master).expect("fnum");
    let fnum_val = fnum.f_num as u16;
    let block = fnum.block as u8;

    // initialize chip (uses low-level add_chip_write via nanonanoda::ym helpers)
    init_ymf262(&mut b);

    // program a few channels and key-on
    for ch in 0u8..5u8 {
        init_ymf262_channel_and_op(&mut b, ch, fnum_val, block, 0x10);
        ymf262_keyon(&mut b, ch, fnum_val, block, 0x10);
    }

    // wait 10 seconds at 44100 Hz
    add_wait_samples(&mut b, 44100 * 10);
    b.add_vgm_command(EndOfData);

    // finalize into a VgmDocument
    b.finalize()
}

fn build_ym2203_4op_1ch_ym() -> VgmDocument {
    let mut b = VgmBuilder::new();

    let gd3 = Gd3 {
        track_name_en: Some("YM2203 4op 1ch (via ym.rs)".to_string()),
        game_name_en: Some("nanonanoda examples".to_string()),
        ..Default::default()
    };
    b.set_gd3(gd3);

    // Register YM2203 primary (and secondary later if needed)
    let master = OpnSpec::default_master_clock();
    b.register_chip(Chip::Ym2203, Instance::Primary, master as u32);

    // F-number table + tune
    let table = generate_12edo_fnum_table::<OpnSpec>(master).unwrap();
    let fnum = find_and_tune_fnumber::<OpnSpec>(&table, 440.0, master).unwrap();
    let fnum_val = fnum.f_num as u16;
    let block = fnum.block as u8;

    // init channel 0 (port 0) and key on
    let ch = 0u8;
    let port = 0u8;
    init_ym2203_channel_and_op(&mut b, port, ch, fnum_val, block, 0x10);
    ym2203_keyon(&mut b, port, ch, fnum_val, block, 0x10);

    // hold a bit after tests
    add_wait_samples(&mut b, 44100 * 10);
    b.add_vgm_command(EndOfData);
    b.finalize()
}

fn build_ym2203_2chip_3ch_ym() -> VgmDocument {
    let mut b = VgmBuilder::new();

    let gd3 = Gd3 {
        track_name_en: Some("YM2203 2-chip 3ch (via ym.rs)".to_string()),
        game_name_en: Some("nanonanoda examples".to_string()),
        ..Default::default()
    };
    b.set_gd3(gd3);

    // Register primary and secondary YM2203 instances
    let master = OpnSpec::default_master_clock();
    b.register_chip(Chip::Ym2203, Instance::Primary, master as u32);
    b.register_chip(Chip::Ym2203, Instance::Secondary, master as u32);

    let table = generate_12edo_fnum_table::<OpnSpec>(master).unwrap();
    let fnum = find_and_tune_fnumber::<OpnSpec>(&table, 440.0, master).unwrap();
    let fnum_val = fnum.f_num as u16;
    let block = fnum.block as u8;

    for port in 0u8..2u8 {
        for ch in 0u8..3u8 {
            init_ym2203_channel_and_op(&mut b, port, ch, fnum_val, block, 0x10);
            ym2203_keyon(&mut b, port, ch, fnum_val, block, 0x10);
        }
    }

    add_wait_samples(&mut b, 44100 * 5);
    b.add_vgm_command(EndOfData);
    b.finalize()
}

fn build_opl3_1chip_18ch_ym() -> VgmDocument {
    let mut b = VgmBuilder::new();

    let gd3 = Gd3 {
        track_name_en: Some("YMF262 1-chip 18ch (via ym.rs)".to_string()),
        game_name_en: Some("nanonanoda examples".to_string()),
        ..Default::default()
    };
    b.set_gd3(gd3);

    // Register OPL3 primary
    let master = Opl3Spec::default_master_clock();
    b.register_chip(Chip::Ymf262, Instance::Primary, master as u32);

    // prepare fnumber table and base tuning
    let table = generate_12edo_fnum_table::<Opl3Spec>(master).expect("fnum table");
    let fnum = find_and_tune_fnumber::<Opl3Spec>(&table, 440.0, master).expect("fnum");
    let fnum_val = fnum.f_num as u16;
    let block = fnum.block as u8;

    // init full 18 channels and key them on
    init_ymf262(&mut b);
    for ch in 0u8..18u8 {
        init_ymf262_channel_and_op(&mut b, ch, fnum_val, block, 0x10);
        ymf262_keyon(&mut b, ch, fnum_val, block, 0x10);
    }

    add_wait_samples(&mut b, 44100 * 5);
    b.add_vgm_command(EndOfData);
    b.finalize()
}

fn build_ym2203_2chip_3ch_init_and_scale() -> VgmDocument {
    let mut b = VgmBuilder::new();

    let gd3 = Gd3 {
        track_name_en: Some("YM2203 init(C4=261.63Hz) + scale (via ym.rs)".to_string()),
        game_name_en: Some("nanonanoda examples".to_string()),
        ..Default::default()
    };
    b.set_gd3(gd3);

    // Register YM2203 (dual)
    let master = OpnSpec::default_master_clock();
    b.register_chip(Chip::Ym2203, Instance::Primary, master as u32);
    b.register_chip(Chip::Ym2203, Instance::Secondary, master as u32);

    let table = generate_12edo_fnum_table::<OpnSpec>(master).unwrap();
    let fnum = find_and_tune_fnumber::<OpnSpec>(&table, 261.63, master).unwrap();
    let fnum_val = fnum.f_num as u16;
    let block = fnum.block as u8;

    for port in 0u8..2u8 {
        for ch in 0u8..3u8 {
            init_ym2203_channel_and_op(&mut b, port, ch, fnum_val, block, 0x10);
        }
    }

    let scale: [i32; 8] = [0, 2, 4, 5, 7, 9, 11, 12];
    let total_channels = 2 * 3; // two chips x 3 channels
    for i in 0usize..total_channels {
        let semitone = scale[i % scale.len()] + ((i / scale.len()) as i32) * 12;
        let freq = 261.63 * 2f64.powf((semitone as f64) / 12.0);
        let fnum = find_and_tune_fnumber::<OpnSpec>(&table, freq, master).unwrap();
        let fnum_val = fnum.f_num as u16;
        let block = fnum.block as u8;
        let port = (i / 3) as u8;
        let ch = (i % 3) as u8;
        ym2203_keyon(&mut b, port, ch, fnum_val, block, 0x10);
        add_wait_samples(&mut b, 22050); // 0.5s between notes
    }

    b.add_vgm_command(EndOfData);
    b.finalize()
}

fn build_opl3_1chip_18ch_init_and_scale() -> VgmDocument {
    let mut b = VgmBuilder::new();

    let gd3 = Gd3 {
        track_name_en: Some("OPL3 init(C4=261.63Hz) + scale (via ym.rs)".to_string()),
        game_name_en: Some("nanonanoda examples".to_string()),
        ..Default::default()
    };
    b.set_gd3(gd3);

    let master = Opl3Spec::default_master_clock();
    b.register_chip(Chip::Ymf262, Instance::Primary, master as u32);

    let table = generate_12edo_fnum_table::<Opl3Spec>(master).unwrap();
    init_ymf262(&mut b);

    let fnum = find_and_tune_fnumber::<Opl3Spec>(&table, 261.63, master).unwrap();
    let fnum_val = fnum.f_num as u16;
    let block = fnum.block as u8;

    for ch in 0u8..18u8 {
        init_ymf262_channel_and_op(&mut b, ch, fnum_val, block, 0x10);
    }

    let scale: [i32; 8] = [0, 2, 4, 5, 7, 9, 11, 12];
    for i in 0usize..18usize {
        let semitone = scale[i % scale.len()] + ((i / scale.len()) as i32) * 12;
        let freq = 261.63 * 2f64.powf((semitone as f64) / 12.0);
        let fnum = find_and_tune_fnumber::<Opl3Spec>(&table, freq, master).unwrap();
        let fnum_val = fnum.f_num as u16;
        let block = fnum.block as u8;
        ymf262_keyon(&mut b, i as u8, fnum_val, block, 0x10);
        add_wait_samples(&mut b, 22050);
    }

    b.add_vgm_command(EndOfData);
    b.finalize()
}

fn main() -> std::io::Result<()> {
    create_dir_all("out")?;

    let doc_opl3 = build_opl3_2op_4ch_ym();
    let len_opl3 = write_doc_to("out/vgm_opl3_2op_4ch_ym.vgm", doc_opl3)?;
    println!("Wrote: out/vgm_opl3_2op_4ch_ym.vgm ({} bytes)", len_opl3);

    let doc_ym2203 = build_ym2203_4op_1ch_ym();
    let len_ym2203 = write_doc_to("out/vgm_ym2203_4op_1ch_ym.vgm", doc_ym2203)?;
    println!(
        "Wrote: out/vgm_ym2203_4op_1ch_ym.vgm ({} bytes)",
        len_ym2203
    );

    let doc_ym2203_2chip_3ch = build_ym2203_2chip_3ch_ym();
    let len_ym2203_2chip_3ch =
        write_doc_to("out/vgm_ym2203_2chip_3ch_ym.vgm", doc_ym2203_2chip_3ch)?;
    println!(
        "Wrote: out/vgm_ym2203_2chip_3ch_ym.vgm ({} bytes)",
        len_ym2203_2chip_3ch
    );

    let doc_opl3_18ch = build_opl3_1chip_18ch_ym();
    let len_opl3_18ch = write_doc_to("out/vgm_opl3_1chip_18ch_ym.vgm", doc_opl3_18ch)?;
    println!(
        "Wrote: out/vgm_opl3_1chip_18ch_ym.vgm ({} bytes)",
        len_opl3_18ch
    );

    let doc_ym2203_init_scale = build_ym2203_2chip_3ch_init_and_scale();
    let len_ym2203_init_scale =
        write_doc_to("out/vgm_ym2203_init_and_scale.vgm", doc_ym2203_init_scale)?;
    println!(
        "Wrote: out/vgm_ym2203_init_and_scale.vgm ({} bytes)",
        len_ym2203_init_scale
    );

    let doc_opl3_init_scale = build_opl3_1chip_18ch_init_and_scale();
    let len_opl3_init_scale = write_doc_to("out/vgm_opl3_init_and_scale.vgm", doc_opl3_init_scale)?;
    println!(
        "Wrote: out/vgm_opl3_init_and_scale.vgm ({} bytes)",
        len_opl3_init_scale
    );

    Ok(())
}
