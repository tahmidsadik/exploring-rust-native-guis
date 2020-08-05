pub enum TagType {
    Bold,
    Italic,
    BoldItalic,
}

pub struct TextTagState {
    pub is_active: bool,
    pub start_offset: i32,
    pub tag_type: TagType,
    pub tag_symbol: char,
}

impl TextTagState {
    pub fn new(tag_type: TagType, tag_symbol: char) -> Self {
        TextTagState {
            is_active: false,
            start_offset: -1,
            tag_type,
            tag_symbol,
        }
    }
}
