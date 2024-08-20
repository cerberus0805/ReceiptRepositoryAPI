use crate::{models::v1::commands::writer_command::WriterCommand, repository::DbRepository, services::v1::{currencies::currencies_service::CurrencyService, inventories::inventories_service::InventoryService, products::products_service::ProductService, receipts::receipts_service::ReceiptService, stores::stores_service::StoreService}};

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
                    match result {
                        WriterCommand::CreateReceipt(new_receipt) => {
                            let service = ReceiptService::new(&repository);
                            tracing::debug!("Start to process create new receipt at date: {}", new_receipt.transaction_date);
                            let _ = service.create_receipt(&new_receipt).await;
                        },
                        WriterCommand::DeleteReceipt(id) => {
                            let service = ReceiptService::new(&repository);
                            tracing::debug!("Start to process delete receipt {}", id);
                            let _ = service.delete_receipt(id).await;
                        },
                        WriterCommand::PatchReceipt(id, patch_receipt) => {
                            let service = ReceiptService::new(&repository);
                            tracing::debug!("Start to process patch receipt {}", id);
                            let _ = service.patch_receipt(id, &patch_receipt).await;
                        },
                        WriterCommand::PatchCurrency(id, patch_currency) => {
                            let service = CurrencyService::new(&repository);
                            tracing::debug!("Start to process patch currency {}", id);
                            let _ = service.patch_currency(id, &patch_currency).await;
                        }, 
                        WriterCommand::PatchStore(id, patch_store) => {
                            let service = StoreService::new(&repository);
                            tracing::debug!("Start to process patch store {}", id);
                            let _ = service.patch_store(id, &patch_store).await;
                        }, 
                        WriterCommand::PatchProduct(id, patch_product) => {
                            let service = ProductService::new(&repository);
                            tracing::debug!("Start to process patch product {}", id);
                            let _ = service.patch_product(id, &patch_product).await;
                        }, 
                        WriterCommand::PatchInventory(id, patch_inventory) => {
                            let service = InventoryService::new(&repository);
                            tracing::debug!("Start to process patch inventory {}", id);
                            let _ = service.patch_inventory(id, &patch_inventory).await;
                        }
                    }
                }
            }
        });

        sender
    }
}