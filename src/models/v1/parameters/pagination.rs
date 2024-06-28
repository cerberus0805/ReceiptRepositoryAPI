use serde::Deserialize;

#[derive(Deserialize)]
pub struct Pagination {
    pub limit: i64,
    pub offset: i64
}

pub const MAX_LIMIT: i64 = 100;

impl Default for Pagination {
    fn default() -> Self {
        Self { limit: 20, offset: 0 }
    }
}