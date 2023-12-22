use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum SeriesType {
    Bool(bool),
    Count(u32),
    Signed(i32),
    Float(f32),
}
