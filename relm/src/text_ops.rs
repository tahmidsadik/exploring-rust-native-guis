use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct InsertOpsData {
    pub content: String,
    pub tag: String,
}

impl InsertOpsData {
    pub fn new(content: String, tag: String) -> Self {
        return InsertOpsData { content, tag };
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Ops {
    Insert(InsertOpsData),
    Delete((i32, i32)),
    MoveCursor(i32),
    SelectColorTag(String),
    ApplyTag((String, i32, i32)),
}

pub struct InsertTextEventData {
    pub offset: i32,
    pub content: String,
}

impl InsertTextEventData {
    pub fn new(offset: i32, content: &str) -> Self {
        InsertTextEventData {
            offset,
            content: String::from(content),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct DeleteTextEventData {
    pub start_offset: i32,
    pub end_offset: i32,
}

impl DeleteTextEventData {
    pub fn new(start_offset: i32, end_offset: i32) -> Self {
        return DeleteTextEventData {
            start_offset,
            end_offset,
        };
    }
}
