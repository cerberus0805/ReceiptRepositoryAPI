use crate::repository::DbRepository;

#[derive(Clone)]
pub struct AppState {
    pub repository: DbRepository
}