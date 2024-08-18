use crate::{models::v1::commands::writer_command::WriterCommand, repository::DbRepository};

#[derive(Clone)]
pub struct HandlerState {
    pub repository: DbRepository,
    pub sender: tokio::sync::mpsc::Sender<WriterCommand>
}

impl HandlerState {
    pub fn new(repository: DbRepository, sender: tokio::sync::mpsc::Sender<WriterCommand>) -> Self {
        Self {
            repository,
            sender
        }
    }
}