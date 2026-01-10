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
