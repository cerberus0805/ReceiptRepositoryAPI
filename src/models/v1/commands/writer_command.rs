use crate::models::v1::forms::create_payload::CreateReceiptPayload;

#[derive(Clone, Debug)]
pub enum WriterCommand {
    CreateReceipt(CreateReceiptPayload),
    DeleteReceipt(i32)
}