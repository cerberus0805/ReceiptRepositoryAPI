use serde::Deserialize;

#[derive(Deserialize)]
pub struct Pagination {
    pub limit: i64,
    pub offset: i64
}

pub const DEFAULT_OFFSET: i64 = 0;
pub const DEFAULT_LIMIT: i64 = 20;
pub const MAX_LIMIT: i64 = 100;

impl Default for Pagination {
    fn default() -> Self {
        Self { 
            limit: DEFAULT_LIMIT, 
            offset: DEFAULT_OFFSET
        }
    }
}