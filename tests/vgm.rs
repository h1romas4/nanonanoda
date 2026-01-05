use nanonanoda::vgm::{Gd3, VgmBuilder, VgmChip};

#[test]
fn test_to_bytes_waits_and_eof() {
    let mut b = VgmBuilder::new();
    // add a 1000-sample wait, a 60Hz wait, a 50Hz wait, and EOF
    b.wait_samples(1000);
    b.wait_60hz();
    b.wait_50hz();
    b.end();
    let doc = b.build();

    let bytes = doc.to_bytes();

    assert_eq!(&bytes[0..4], b"Vgm ");

    let version = u32::from_le_bytes(bytes[0x08..0x0C].try_into().unwrap());
    assert_eq!(version, doc.header.version);

    let eof_offset = u32::from_le_bytes(bytes[0x04..0x08].try_into().unwrap());
    let file_size = bytes.len() as u32;
    assert_eq!(eof_offset, file_size.wrapping_sub(4));

    let cmd_start = 0x100usize;
    assert!(bytes.len() >= cmd_start + 4);

    let seq = &bytes[cmd_start..];
    assert_eq!(seq[0], 0x61u8);
    assert_eq!(seq[1], 0xE8u8);
    assert_eq!(seq[2], 0x03u8);

    let mut found = 0;
    for &b in &seq[3..] {
        if found == 0 && b == 0x62u8 {
            found = 1;
            continue;
        }
        if found == 1 && b == 0x63u8 {
            found = 2;
            continue;
        }
        if found == 2 && b == 0x66u8 {
            found = 3;
            break;
        }
    }
    assert_eq!(
        found, 3,
        "did not find 0x62/0x63/0x66 sequence in command stream"
    );
}

#[test]
fn test_gd3_serialization() {
    let mut b = VgmBuilder::new();
    b.wait_samples(1);
    b.end();

    let mut gd3 = Gd3::default();
    gd3.track_name_en = Some("TrackX".to_string());
    gd3.notes = Some("Note".to_string());

    b.set_gd3(gd3);
    let doc = b.build();
    let bytes = doc.to_bytes();

    let pos = bytes
        .windows(4)
        .position(|w| w == b"Gd3 ")
        .expect("Gd3 chunk not found");

    let hdr_off = u32::from_le_bytes(bytes[0x14..0x18].try_into().unwrap());
    assert_eq!(hdr_off, (pos as u32).wrapping_sub(0x14));

    let gd3_len = u32::from_le_bytes(bytes[pos + 8..pos + 12].try_into().unwrap());

    let fields: [Option<&str>; 11] = [
        Some("TrackX"),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        Some("Note"),
    ];

    let mut expected: Vec<u8> = Vec::new();
    for f in &fields {
        if let Some(s) = f {
            for code in s.encode_utf16() {
                expected.extend_from_slice(&code.to_le_bytes());
            }
        }
        expected.extend_from_slice(&0u16.to_le_bytes());
    }

    assert_eq!(gd3_len, expected.len() as u32);
    let data_start = pos + 12;
    assert_eq!(
        &bytes[data_start..data_start + expected.len()],
        &expected[..]
    );
}

#[test]
fn test_ym2203_port1_write_encoding() {
    let mut b = VgmBuilder::new();

    b.enable_dual_chip(VgmChip::Ym2203);
    b.ym2203_write(1, 0x2A, 0x55);
    b.end();
    let doc = b.build();

    let bytes = doc.to_bytes();
    let cmd_start = 0x100usize;
    assert!(bytes.len() > cmd_start + 3);
    let seq = &bytes[cmd_start..];

    let mut found = false;
    for i in 0..seq.len().saturating_sub(2) {
        if seq[i] == 0xA5u8 && seq[i + 1] == 0x2A && seq[i + 2] == 0x55 {
            found = true;
            break;
        }
    }
    assert!(
        found,
        "did not find YM2203 port1 write triplet (0xA5,0x2A,0x55)"
    );
}

#[test]
fn test_to_bytes_chip_writes() {
    let mut b = VgmBuilder::new();

    b.ymf262_write(0, 0x20, 0x99);
    b.ym2203_write(0, 0x2A, 0x55);
    b.end();
    let doc = b.build();

    let bytes = doc.to_bytes();
    let cmd_start = 0x100usize;
    let seq = &bytes[cmd_start..];

    let mut idx = 0usize;
    while idx + 3 <= seq.len() {
        if seq[idx] == 0x5E && seq[idx + 1] == 0x20 && seq[idx + 2] == 0x99 {
            break;
        }
        idx += 1;
    }
    assert!(idx + 3 <= seq.len(), "did not find YMF262 write sequence");
    idx += 3;

    while idx + 3 <= seq.len() {
        if seq[idx] == 0x55 && seq[idx + 1] == 0x2A && seq[idx + 2] == 0x55 {
            break;
        }
        idx += 1;
    }
    assert!(idx + 3 <= seq.len(), "did not find YM2203 write sequence");
}
