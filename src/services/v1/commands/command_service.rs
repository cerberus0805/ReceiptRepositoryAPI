use crate::{models::v1::commands::writer_command::WriterCommand, repository::DbRepository, services::v1::receipts::receipts_service::ReceiptService};

pub struct CommandService {
}

impl CommandService {
    pub fn run(repository: DbRepository, buffer_size: usize) -> tokio::sync::mpsc::Sender<WriterCommand> {
        let (sender, mut receiver) = tokio::sync::mpsc::channel(buffer_size);
        tracing::info!("Create writer channel with size: {}", buffer_size);

        tokio::spawn(async move {
            loop {
                let received_result = receiver.recv().await;
                if let Some(result) = received_result {
                    let service = ReceiptService::new(&repository);
                    match result {
                        WriterCommand::CreateReceipt(new_receipt) => {
                            tracing::debug!("Start to process create new receipt at date: {}", new_receipt.transaction_date);
                            let _ = service.create_receipt(&new_receipt).await;
                        },
                        WriterCommand::DeleteReceipt(id) => {
                            tracing::debug!("Start to process delete receipt {}", id);
                            let _ = service.delete_receipt(id).await;
                        }
                    }
                }
            }
        });

        sender
    }
}