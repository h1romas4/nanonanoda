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

    assert_eq!(&bytes[..4], b"Vgm ");

    let version = u32::from_le_bytes(bytes[0x08..0x0C].try_into().unwrap());
    assert_eq!(version, doc.header.version);

    let eof_offset = u32::from_le_bytes(bytes[0x04..0x08].try_into().unwrap());
    let file_size = bytes.len() as u32;
    assert_eq!(eof_offset, file_size.wrapping_sub(4));

    let cmd_start: usize = 0x100;
    assert!(bytes.len() >= cmd_start + 4);

    let seq = &bytes[cmd_start..];
    assert_eq!(seq[0], 0x61);
    assert_eq!(seq[1], 0xE8);
    assert_eq!(seq[2], 0x03);

    let found_seq = seq[3..].windows(3).position(|w| w == b"\x62\x63\x66");
    assert!(
        found_seq.is_some(),
        "did not find 0x62/0x63/0x66 sequence in command stream"
    );
}

#[test]
fn test_gd3_serialization() {
    let mut b = VgmBuilder::new();
    b.wait_samples(1);
    b.end();

    let gd3 = Gd3 {
        track_name_en: Some("TrackX".to_string()),
        notes: Some("Note".to_string()),
        ..Default::default()
    };

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
    let cmd_start: usize = 0x100;
    assert!(bytes.len() > cmd_start + 3);
    let seq = &bytes[cmd_start..];

    assert!(
        seq.windows(3).any(|w| w == b"\xA5\x2A\x55"),
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
    let cmd_start: usize = 0x100;
    let seq = &bytes[cmd_start..];

    // find first sequence (YMF262 write)
    let pos1 = seq
        .windows(3)
        .position(|w| w == b"\x5E\x20\x99")
        .expect("did not find YMF262 write sequence");
    // search for the next sequence after the first (YM2203 write)
    let pos2 = seq[pos1 + 3..]
        .windows(3)
        .position(|w| w == b"\x55\x2A\x55")
        .expect("did not find YM2203 write sequence");
    let _idx_after = pos1 + 3 + pos2;
}
