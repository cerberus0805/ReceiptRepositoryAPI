use crate::{models::v1::commands::writer_command::WriterCommand, repository::DbRepository, services::v1::receipts::receipts_service::ReceiptService};

pub struct CommandService {
}

impl CommandService {
    pub fn run(repository: DbRepository) -> tokio::sync::mpsc::Sender<WriterCommand> {
        let (sender, mut receiver): (tokio::sync::mpsc::Sender<WriterCommand>, tokio::sync::mpsc::Receiver<WriterCommand>) 
        = tokio::sync::mpsc::channel(128);

        tokio::spawn(async move {
            loop {
                let received_result = receiver.recv().await;
                if let Some(result) = received_result {
                    let service = ReceiptService::new(&repository);
                    match result {
                        WriterCommand::CreateReceipt(new_receipt) => {
                            let _ = service.create_receipt(&new_receipt).await;
                        },
                        WriterCommand::DeleteReceipt(id) => {
                            let _ = service.delete_receipt(id).await;
                        }
                    }
                }
            }
        });

        sender
    }
}