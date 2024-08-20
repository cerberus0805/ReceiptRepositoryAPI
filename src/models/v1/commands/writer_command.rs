use crate::models::v1::forms::create_payload::CreateReceiptPayload;
use crate::models::v1::forms::patch_payload::{PatchReceiptPayload, PatchCurrencyPayload, PatchStorePayload, PatchProductPayload, PatchInventoryPayload};

#[derive(Clone, Debug)]
pub enum WriterCommand {
    CreateReceipt(CreateReceiptPayload),
    DeleteReceipt(i32),
    PatchReceipt(i32, PatchReceiptPayload),
    PatchCurrency(i32, PatchCurrencyPayload),
    PatchStore(i32, PatchStorePayload),
    PatchProduct(i32, PatchProductPayload),
    PatchInventory(i32, PatchInventoryPayload)
}