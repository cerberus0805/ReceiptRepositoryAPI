use crate::models::v1::parameters::pagination::{Pagination, DEFAULT_LIMIT, DEFAULT_OFFSET, MAX_LIMIT};

pub struct FallbacksService {
}

impl FallbacksService {
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn fallback_pagination(&self, pagination: &Pagination) -> (i64, i64) {
        let mut page_offset: i64 = pagination.offset;
        let mut per_page: i64 = pagination.limit;
        if pagination.offset < 0 {
            tracing::debug!("fallback to default offset for negative offset");
            page_offset = DEFAULT_OFFSET;
        }

        if pagination.limit < 1 {
            tracing::debug!("fallback to default limit for negative or zeor offset");
            per_page = DEFAULT_LIMIT;
        }

        if pagination.limit > MAX_LIMIT {
            tracing::debug!("fallback to max limit due to max limit exceeds");
            per_page = MAX_LIMIT;
        }

        (page_offset, per_page)
    }
}