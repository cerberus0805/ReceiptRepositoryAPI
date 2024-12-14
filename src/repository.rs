use std::sync::Arc;
use diesel::{r2d2::{ConnectionManager, Pool}, PgConnection};

#[derive(Clone)]
pub struct DbRepository {
    pub pool: Arc<Pool<ConnectionManager<PgConnection>>>
}

impl DbRepository {
    pub fn new(url: &str) -> Self {
        Self {
            pool: Arc::new(
                Pool::builder().build(ConnectionManager::<PgConnection>::new(url)).expect("Create database pool failed")
            )
        }
    }
}