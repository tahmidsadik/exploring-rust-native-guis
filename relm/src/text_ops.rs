use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Ops {
    Insert(String),
    Delete(i32),
    MoveCursor(i32),
}
