use crate::Ops;

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

pub struct BoldItalicTagState {
    pub single_asterisk_active: bool,
    pub double_asterisk_active: bool,
    pub single_asterisk_complete: bool,
    pub start_offset: i32,
    pub end_offset: i32,
    pub single_asterisk_tag_name: String,
    pub double_asterisk_tag_name: String,
}

impl BoldItalicTagState {
    pub fn new(single_asterisk_tag_name: String, double_asterisk_tag_name: String) -> Self {
        return BoldItalicTagState {
            single_asterisk_active: false,
            double_asterisk_active: false,
            single_asterisk_complete: false,
            start_offset: -1,
            end_offset: -1,
            single_asterisk_tag_name,
            double_asterisk_tag_name,
        };
    }

    fn reset_state(&mut self) {
        self.start_offset = -1;
        self.single_asterisk_complete = false;
        self.single_asterisk_active = false;
        self.double_asterisk_active = false;
    }

    pub fn update_state(
        &mut self,
        offset: i32,
        string_to_modify: String,
        last_char: String,
    ) -> (&Self, Vec<Ops>) {
        let mut ops_to_perform = vec![];

        if last_char != "*".to_string()
            && self.single_asterisk_complete
            && self.double_asterisk_active
        {
            // single_asterisk is matched and realized

            ops_to_perform.push(Ops::ApplyTag((
                self.single_asterisk_tag_name.to_string(),
                self.start_offset,
                offset,
            )));
            ops_to_perform.push(Ops::Delete((self.start_offset, self.start_offset + 1)));
            ops_to_perform.push(Ops::Delete((offset - 1, offset)));
            self.reset_state();
            return (self, ops_to_perform);
        } else if last_char == "*".to_string() {
            return self.update_state_with_asterisk(offset, string_to_modify);
        } else {
            return (self, ops_to_perform);
        }
    }

    pub fn update_state_with_asterisk(
        &mut self,
        offset: i32,
        string_to_modify: String,
    ) -> (&Self, Vec<Ops>) {
        let mut tag_to_apply = None;
        if self.single_asterisk_active == false && self.double_asterisk_active == false {
            self.single_asterisk_active = true;
            self.start_offset = offset;
        } else if self.single_asterisk_active == true && self.double_asterisk_active == false {
            if string_to_modify.len() == 1 {
                self.double_asterisk_active = true
            } else if string_to_modify[..1] == " ".to_string()
                || string_to_modify[(string_to_modify.len() - 2)..] == " ".to_string()
            {
                self.single_asterisk_active = true;
                self.start_offset = offset;
            } else {
                tag_to_apply = Some(self.single_asterisk_tag_name.to_string());
            }
        } else {
            if self.single_asterisk_complete == false {
                if string_to_modify == "*".to_string() || string_to_modify.len() == 0 {
                    self.double_asterisk_active = true;
                    self.single_asterisk_active = true;
                    self.start_offset = self.start_offset + 1;
                } else if string_to_modify[..1] == " ".to_string()
                    || string_to_modify[(string_to_modify.len() - 1)..] == " ".to_string()
                {
                    self.single_asterisk_active = true;
                    self.start_offset = offset;
                    self.double_asterisk_active = false;
                } else {
                    self.single_asterisk_complete = true;
                }
            } else {
                // do not need to check for spaces or inali
                tag_to_apply = Some(self.double_asterisk_tag_name.to_string());
            }
        }

        let mut ops_to_perform = vec![];
        match tag_to_apply {
            Some(tag_name) => {
                ops_to_perform.push(Ops::Delete((
                    self.start_offset,
                    self.start_offset
                        + if self.single_asterisk_tag_name == tag_name.to_string() {
                            1
                        } else {
                            2
                        },
                )));
                ops_to_perform.push(Ops::Delete((
                    offset
                        - if self.single_asterisk_tag_name == tag_name.to_string() {
                            1
                        } else {
                            3
                        },
                    offset,
                )));
                ops_to_perform.push(Ops::ApplyTag((
                    tag_name.to_string(),
                    self.start_offset,
                    offset,
                )));
                self.reset_state();
            }
            None => {}
        }

        return (self, ops_to_perform);
    }
}
