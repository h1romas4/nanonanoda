use crate::binutil::{ParseError, read_slice, read_u16_le_at, read_u32_le_at};

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

impl Gd3 {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut out: Vec<u8> = Vec::new();

        out.extend_from_slice(b"Gd3 ");
        out.extend_from_slice(&0x00000100u32.to_le_bytes()); // version 1.00
        out.extend_from_slice(&0u32.to_le_bytes());

        let fields: [&Option<String>; 11] = [
            &self.track_name_en,
            &self.track_name_jp,
            &self.game_name_en,
            &self.game_name_jp,
            &self.system_name_en,
            &self.system_name_jp,
            &self.author_name_en,
            &self.author_name_jp,
            &self.release_date,
            &self.creator,
            &self.notes,
        ];

        let mut data: Vec<u8> = Vec::new();
        for f in &fields {
            if let Some(s) = f {
                for code in s.encode_utf16() {
                    data.extend_from_slice(&code.to_le_bytes());
                }
            }
            data.extend_from_slice(&0_u16.to_le_bytes());
        }

        let len = data.len() as u32;
        out.extend_from_slice(&data);

        let len_bytes = len.to_le_bytes();
        out[8..12].copy_from_slice(&len_bytes);

        out
    }
}

/// Parse a Gd3 block from bytes (full Gd3 chunk starting at offset 0).
/// Returns a populated `Gd3` or a `ParseError` on failure.
pub(crate) fn parse_gd3(bytes: &[u8]) -> Result<Gd3, ParseError> {
    // need at least 12 bytes: ident(4) + version(4) + length(4)
    if bytes.len() < 12 {
        return Err(ParseError::HeaderTooShort);
    }

    let ident = read_slice(bytes, 0, 4)?;
    if ident != b"Gd3 " {
        let mut id: [u8; 4] = [0; 4];
        id.copy_from_slice(ident);
        return Err(ParseError::InvalidIdent(id));
    }

    let _version = read_u32_le_at(bytes, 4)?;
    let data_len = read_u32_le_at(bytes, 8)? as usize;

    let data_off = 0x0Cusize;
    if bytes.len() < data_off + data_len {
        return Err(ParseError::UnexpectedEof);
    }

    let data = read_slice(bytes, data_off, data_len)?;

    // There are 11 UTF-16LE nul-terminated fields.
    let mut fields: Vec<Option<String>> = Vec::with_capacity(11);
    let mut i = 0usize;
    for _ in 0..11 {
        let mut codes: Vec<u16> = Vec::new();
        loop {
            if i + 1 >= data.len() {
                return Err(ParseError::UnexpectedEof);
            }
            let code = read_u16_le_at(data, i)?;
            i += 2;
            if code == 0 {
                break;
            }
            codes.push(code);
        }

        if codes.is_empty() {
            fields.push(None);
        } else {
            match String::from_utf16(&codes) {
                Ok(s) => fields.push(Some(s)),
                Err(e) => return Err(ParseError::Other(format!("invalid utf16 in gd3: {}", e))),
            }
        }
    }

    // Map into Gd3 struct
    Ok(Gd3 {
        track_name_en: fields[0].clone(),
        track_name_jp: fields[1].clone(),
        game_name_en: fields[2].clone(),
        game_name_jp: fields[3].clone(),
        system_name_en: fields[4].clone(),
        system_name_jp: fields[5].clone(),
        author_name_en: fields[6].clone(),
        author_name_jp: fields[7].clone(),
        release_date: fields[8].clone(),
        creator: fields[9].clone(),
        notes: fields[10].clone(),
    })
}

/// Attempt to convert a raw Gd3 byte slice into a `Gd3` value.
///
/// This is a fallible conversion that delegates to `parse_gd3` and returns
/// a `crate::binutil::ParseError` on failure. Use `Gd3::try_from(bytes)` or
/// call `parse_gd3(bytes)` directly to handle parse errors explicitly.
impl std::convert::TryFrom<&[u8]> for Gd3 {
    type Error = crate::binutil::ParseError;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        parse_gd3(bytes)
    }
}
