// Utilities used by VGM/XGM parsers: parse error type and byte readers.

#[derive(Debug, Clone)]
pub enum ParseError {
    UnexpectedEof,
    InvalidIdent([u8; 4]),
    UnsupportedVersion(u32),
    HeaderTooShort,
    Other(String),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::UnexpectedEof => write!(f, "unexpected end of input"),
            ParseError::InvalidIdent(id) => write!(f, "invalid ident: {:?}", id),
            ParseError::UnsupportedVersion(v) => write!(f, "unsupported version: {}", v),
            ParseError::HeaderTooShort => write!(f, "header too short"),
            ParseError::Other(s) => write!(f, "{}", s),
        }
    }
}

impl std::error::Error for ParseError {}

pub fn read_u32_le_at(bytes: &[u8], off: usize) -> Result<u32, ParseError> {
    if bytes.len() < off + 4 {
        return Err(ParseError::UnexpectedEof);
    }
    let mut tmp: [u8; 4] = [0; 4];
    tmp.copy_from_slice(&bytes[off..off + 4]);
    Ok(u32::from_le_bytes(tmp))
}

pub fn read_u16_le_at(bytes: &[u8], off: usize) -> Result<u16, ParseError> {
    if bytes.len() < off + 2 {
        return Err(ParseError::UnexpectedEof);
    }
    let mut tmp: [u8; 2] = [0; 2];
    tmp.copy_from_slice(&bytes[off..off + 2]);
    Ok(u16::from_le_bytes(tmp))
}

pub fn read_u8_at(bytes: &[u8], off: usize) -> Result<u8, ParseError> {
    if bytes.len() <= off {
        return Err(ParseError::UnexpectedEof);
    }
    Ok(bytes[off])
}

pub fn read_slice(bytes: &[u8], off: usize, len: usize) -> Result<&[u8], ParseError> {
    if bytes.len() < off + len {
        return Err(ParseError::UnexpectedEof);
    }
    Ok(&bytes[off..off + len])
}

pub fn read_u24_be_at(bytes: &[u8], off: usize) -> Result<u32, ParseError> {
    if bytes.len() < off + 3 {
        return Err(ParseError::UnexpectedEof);
    }
    let b0 = bytes[off] as u32;
    let b1 = bytes[off + 1] as u32;
    let b2 = bytes[off + 2] as u32;
    Ok((b0 << 16) | (b1 << 8) | b2)
}

pub fn read_i32_le_at(bytes: &[u8], off: usize) -> Result<i32, ParseError> {
    let v = read_u32_le_at(bytes, off)?;
    Ok(i32::from_le_bytes(v.to_le_bytes()))
}

pub fn write_u32(buf: &mut [u8], off: usize, v: u32) {
    let bytes = v.to_le_bytes();
    buf[off..off + 4].copy_from_slice(&bytes);
}

pub fn write_u16(buf: &mut [u8], off: usize, v: u16) {
    let bytes = v.to_le_bytes();
    buf[off..off + 2].copy_from_slice(&bytes);
}

pub fn write_u8(buf: &mut [u8], off: usize, v: u8) {
    buf[off] = v;
}

pub fn write_slice(buf: &mut [u8], off: usize, s: &[u8]) {
    buf[off..off + s.len()].copy_from_slice(s);
}
