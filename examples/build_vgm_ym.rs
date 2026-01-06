use nanonanoda::fnumber::ChipSpec;
use std::fs::{File, create_dir_all};
use std::io::Write;

fn write_doc_to(path: &str, doc: nanonanoda::vgm::VgmDocument) -> std::io::Result<usize> {
    let bytes = doc.to_bytes();
    let mut f = File::create(path)?;
    f.write_all(&bytes)?;
    Ok(bytes.len())
}

fn build_opl3_2op_4ch_ym() -> nanonanoda::vgm::VgmDocument {
    let mut b = nanonanoda::vgm::VgmBuilder::new();

    // GD3 metadata for this example
    let mut gd3 = nanonanoda::vgm::Gd3::default();
    gd3.track_name_en = Some("OPL3 2op 4ch (via ym.rs)".to_string());
    gd3.game_name_en = Some("nanonanoda examples".to_string());
    b.set_gd3(gd3);

    let fnum_table = nanonanoda::fnumber::generate_12edo_fnum_table::<
        nanonanoda::fnumber::YMF262SpecOpl3,
    >(nanonanoda::fnumber::YMF262SpecOpl3::default_master_clock())
    .expect("fnum table");

    let fnum = nanonanoda::fnumber::find_and_tune_fnumber::<nanonanoda::fnumber::YMF262SpecOpl3>(
        &fnum_table,
        440.0,
        nanonanoda::fnumber::YMF262SpecOpl3::default_master_clock(),
    )
    .expect("fnum");

    let fnum_val = fnum.f_num as u16;
    let block = fnum.block as u8;

    b.add_chip_clock(nanonanoda::vgm::VgmChip::Ymf262, 14_318_180);

    nanonanoda::ym::init_ymf262(&mut b);

    let ch = 0u8;
    nanonanoda::ym::init_ymf262_channel_and_op(&mut b, ch, fnum_val, block, 0x10);
    nanonanoda::ym::ymf262_keyon(&mut b, ch, fnum_val, block, 0x10);

    let ch = 1u8;
    nanonanoda::ym::init_ymf262_channel_and_op(&mut b, ch, fnum_val, block, 0x10);
    nanonanoda::ym::ymf262_keyon(&mut b, ch, fnum_val, block, 0x10);

    let ch = 2u8;
    nanonanoda::ym::init_ymf262_channel_and_op(&mut b, ch, fnum_val, block, 0x10);
    nanonanoda::ym::ymf262_keyon(&mut b, ch, fnum_val, block, 0x10);

    let ch = 3u8;
    nanonanoda::ym::init_ymf262_channel_and_op(&mut b, ch, fnum_val, block, 0x10);
    nanonanoda::ym::ymf262_keyon(&mut b, ch, fnum_val, block, 0x10);

    let ch = 4u8;
    nanonanoda::ym::init_ymf262_channel_and_op(&mut b, ch, fnum_val, block, 0x10);
    nanonanoda::ym::ymf262_keyon(&mut b, ch, fnum_val, block, 0x10);

    // wait 10 seconds at 44100 Hz
    b.wait_samples(44100 * 10);
    b.end();
    b.build()
}

fn build_ym2203_4op_1ch_ym() -> nanonanoda::vgm::VgmDocument {
    let mut b = nanonanoda::vgm::VgmBuilder::new();

    let mut gd3 = nanonanoda::vgm::Gd3::default();
    gd3.track_name_en = Some("YM2203 4op 1ch (via ym.rs)".to_string());
    gd3.game_name_en = Some("nanonanoda examples".to_string());
    b.set_gd3(gd3);

    // Carrier frequency/key-on for channel 0 (A4)
    let master = nanonanoda::fnumber::YM2203Spec::default_master_clock();
    let table =
        nanonanoda::fnumber::generate_12edo_fnum_table::<nanonanoda::fnumber::YM2203Spec>(master)
            .unwrap();
    let fnum = nanonanoda::fnumber::find_and_tune_fnumber::<nanonanoda::fnumber::YM2203Spec>(
        &table, 440.0, master,
    )
    .unwrap();

    let fnum_val = fnum.f_num as u16;
    let block = fnum.block as u8;

    let ch = 0u8; // channel 0
    let port = 0u8;

    b.add_chip_clock(nanonanoda::vgm::VgmChip::Ym2203, master as u32);
    b.enable_dual_chip(nanonanoda::vgm::VgmChip::Ym2203);

    nanonanoda::ym::init_ym2203_channel_and_op(&mut b, port, ch, fnum_val, block, 0x10);
    nanonanoda::ym::ym2203_keyon(&mut b, port, ch, fnum_val, block, 0x10);

    // hold a bit after tests
    b.wait_samples(44100 * 10);
    b.end();
    b.build()
}

fn build_ym2203_2chip_3ch_ym() -> nanonanoda::vgm::VgmDocument {
    let mut b = nanonanoda::vgm::VgmBuilder::new();

    let mut gd3 = nanonanoda::vgm::Gd3::default();
    gd3.track_name_en = Some("YM2203 2-chip 3ch (via ym.rs)".to_string());
    gd3.game_name_en = Some("nanonanoda examples".to_string());
    b.set_gd3(gd3);

    let master = nanonanoda::fnumber::YM2203Spec::default_master_clock();
    let table =
        nanonanoda::fnumber::generate_12edo_fnum_table::<nanonanoda::fnumber::YM2203Spec>(master)
            .unwrap();

    b.add_chip_clock(nanonanoda::vgm::VgmChip::Ym2203, master as u32);
    b.enable_dual_chip(nanonanoda::vgm::VgmChip::Ym2203);

    let freq = 440.0;
    let fnum = nanonanoda::fnumber::find_and_tune_fnumber::<nanonanoda::fnumber::YM2203Spec>(
        &table, freq, master,
    )
    .unwrap();
    let fnum_val = fnum.f_num as u16;
    let block = fnum.block as u8;

    for port in 0u8..2u8 {
        for ch in 0u8..3u8 {
            nanonanoda::ym::init_ym2203_channel_and_op(&mut b, port, ch, fnum_val, block, 0x10);
            nanonanoda::ym::ym2203_keyon(&mut b, port, ch, fnum_val, block, 0x10);
        }
    }

    b.wait_samples(44100 * 5);
    b.end();
    b.build()
}

fn build_opl3_1chip_18ch_ym() -> nanonanoda::vgm::VgmDocument {
    let mut b = nanonanoda::vgm::VgmBuilder::new();

    let mut gd3 = nanonanoda::vgm::Gd3::default();
    gd3.track_name_en = Some("YMF262 1-chip 18ch (via ym.rs)".to_string());
    gd3.game_name_en = Some("nanonanoda examples".to_string());
    b.set_gd3(gd3);

    let master = nanonanoda::fnumber::YMF262SpecOpl3::default_master_clock();
    let table =
        nanonanoda::fnumber::generate_12edo_fnum_table::<nanonanoda::fnumber::YMF262SpecOpl3>(
            master,
        )
        .expect("fnum table");

    b.add_chip_clock(nanonanoda::vgm::VgmChip::Ymf262, 14_318_180);
    nanonanoda::ym::init_ymf262(&mut b);

    let freq = 440.0;
    let fnum = nanonanoda::fnumber::find_and_tune_fnumber::<nanonanoda::fnumber::YMF262SpecOpl3>(
        &table, freq, master,
    )
    .expect("fnum");
    let fnum_val = fnum.f_num as u16;
    let block = fnum.block as u8;

    for i in 0u8..18u8 {
        nanonanoda::ym::init_ymf262_channel_and_op(&mut b, i, fnum_val, block, 0x10);
        nanonanoda::ym::ymf262_keyon(&mut b, i, fnum_val, block, 0x10);
    }

    b.wait_samples(44100 * 5);
    b.end();
    b.build()
}

fn build_ym2203_2chip_3ch_init_and_scale() -> nanonanoda::vgm::VgmDocument {
    let mut b = nanonanoda::vgm::VgmBuilder::new();

    let mut gd3 = nanonanoda::vgm::Gd3::default();
    gd3.track_name_en = Some("YM2203 init(C4=261.63Hz) + scale (via ym.rs)".to_string());
    gd3.game_name_en = Some("nanonanoda examples".to_string());
    b.set_gd3(gd3);

    let master = nanonanoda::fnumber::YM2203Spec::default_master_clock();
    let table =
        nanonanoda::fnumber::generate_12edo_fnum_table::<nanonanoda::fnumber::YM2203Spec>(master)
            .unwrap();

    b.add_chip_clock(nanonanoda::vgm::VgmChip::Ym2203, master as u32);
    b.enable_dual_chip(nanonanoda::vgm::VgmChip::Ym2203);

    let fnum = nanonanoda::fnumber::find_and_tune_fnumber::<nanonanoda::fnumber::YM2203Spec>(
        &table, 261.63, master,
    )
    .unwrap();
    let fnum_val = fnum.f_num as u16;
    let block = fnum.block as u8;

    for port in 0u8..2u8 {
        for ch in 0u8..3u8 {
            nanonanoda::ym::init_ym2203_channel_and_op(&mut b, port, ch, fnum_val, block, 0x10);
        }
    }

    let scale: [i32; 8] = [0, 2, 4, 5, 7, 9, 11, 12];
    let total_channels = 2 * 3; // two chips x 3 channels
    for i in 0usize..total_channels {
        let semitone = scale[i % scale.len()] + ((i / scale.len()) as i32) * 12;
        let freq = 261.63 * 2f64.powf((semitone as f64) / 12.0);
        let fnum = nanonanoda::fnumber::find_and_tune_fnumber::<nanonanoda::fnumber::YM2203Spec>(
            &table, freq, master,
        )
        .unwrap();
        let fnum_val = fnum.f_num as u16;
        let block = fnum.block as u8;
        let port = (i / 3) as u8;
        let ch = (i % 3) as u8;
        nanonanoda::ym::ym2203_keyon(&mut b, port, ch, fnum_val, block, 0x10);
        b.wait_samples(22050); // 0.5s between notes
    }

    b.end();
    b.build()
}

fn build_opl3_1chip_18ch_init_and_scale() -> nanonanoda::vgm::VgmDocument {
    let mut b = nanonanoda::vgm::VgmBuilder::new();

    let mut gd3 = nanonanoda::vgm::Gd3::default();
    gd3.track_name_en = Some("OPL3 init(C4=261.63Hz) + scale (via ym.rs)".to_string());
    gd3.game_name_en = Some("nanonanoda examples".to_string());
    b.set_gd3(gd3);

    let master = nanonanoda::fnumber::YMF262SpecOpl3::default_master_clock();
    let table =
        nanonanoda::fnumber::generate_12edo_fnum_table::<nanonanoda::fnumber::YMF262SpecOpl3>(
            master,
        )
        .unwrap();

    b.add_chip_clock(nanonanoda::vgm::VgmChip::Ymf262, master as u32);
    nanonanoda::ym::init_ymf262(&mut b);

    let fnum = nanonanoda::fnumber::find_and_tune_fnumber::<nanonanoda::fnumber::YMF262SpecOpl3>(
        &table, 261.63, master,
    )
    .unwrap();
    let fnum_val = fnum.f_num as u16;
    let block = fnum.block as u8;

    for ch in 0u8..18u8 {
        nanonanoda::ym::init_ymf262_channel_and_op(&mut b, ch, fnum_val, block, 0x10);
    }

    let scale: [i32; 8] = [0, 2, 4, 5, 7, 9, 11, 12];
    for i in 0usize..18usize {
        let semitone = scale[i % scale.len()] + ((i / scale.len()) as i32) * 12;
        let freq = 261.63 * 2f64.powf((semitone as f64) / 12.0);
        let fnum =
            nanonanoda::fnumber::find_and_tune_fnumber::<nanonanoda::fnumber::YMF262SpecOpl3>(
                &table, freq, master,
            )
            .unwrap();
        let fnum_val = fnum.f_num as u16;
        let block = fnum.block as u8;
        nanonanoda::ym::ymf262_keyon(&mut b, i as u8, fnum_val, block, 0x10);
        b.wait_samples(22050);
    }

    b.end();
    b.build()
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
