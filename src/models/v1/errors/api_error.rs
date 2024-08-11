use thiserror::Error;
use serde::Serialize;

#[derive(Debug, Error, PartialEq, Serialize)]
pub enum ApiError {
    #[error("Generic error")]
    Generic,
    #[error("Invalid parameter")]
    InvalidParameter,
    #[error("Record not found")]
    NoRecord,
    #[error("Database disconnect")]
    DatabaseConnectionBroken,
    #[error("Both id and name are none")]
    CurrencyInvalid,
    #[error("Currency id is not existed")]
    CurrencyIdNotExisted,
    #[error("Currency name is duplicated")]
    CurrencyNameDuplicated,
    #[error("Both id and name are none")]
    StoreInvalid,
    #[error("Store id is not existed")]
    StoreIdNotExisted,
    #[error("Store name is duplicated")]
    StoreNameDuplicated,
    #[error("Both id and name are none")]
    ProductInvalid,
    #[error("Product id is not existed")]
    ProductIdNotExisted,
    #[error("Product name is duplicated")]
    ProductNameDuplicated,
    #[error("Insert a new currency is failed")]
    InsertCurrencyFailed,
    #[error("Insert a new store is failed")]
    InsertStoreFailed,
    #[error("Insert a new product is failed")]
    InsertProductFailed,
    #[error("Insert a new receipt is failed")]
    InsertReceiptFailed,
    #[error("Insert a new inventory is failed")]
    InsertInventoryFailed,
    #[error("Update a new currency is failed")]
    UpdateCurrencyFailed,
    #[error("Update a new store is failed")]
    UpdateStoreFailed,
    #[error("Update a new product is failed")]
    UpdateProductFailed,
    #[error("Update a new receipt is failed")]
    UpdateReceiptFailed,
    #[error("Update a new inventory is failed")]
    UpdateInventoryFailed,
    #[error("Delete a receipt whose id is not existed")]
    DeleteReceiptIdNotExisted,
    #[error("Delete data which is associated to a receipt failed")]
    DeleteReceiptAssociatedEntryFailed,
    #[error("Delete a receipt failed")]
    DeleteReceiptEntryFailed,
    #[error("Delete data which is related to a receipt failed")]
    DeleteReceiptRelatedEntryFailed
}