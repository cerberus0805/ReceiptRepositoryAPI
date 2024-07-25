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
    FormFieldCurrencyInvalid,
    #[error("Currency id is not existed")]
    FormFieldCurrencyIdNotExisted,
    #[error("Currency name is duplicated")]
    FormFieldCurrencyNameDuplicated,
    #[error("Both id and name are none")]
    FormFieldStoreInvalid,
    #[error("Store id is not existed")]
    FormFieldStoreIdNotExisted,
    #[error("Store name is duplicated")]
    FormFieldStoreNameDuplicated,
    #[error("Both id and name are none")]
    FormFieldProductInvalid,
    #[error("Product id is not existed")]
    FormFieldProductIdNotExisted,
    #[error("Product name is duplicated")]
    FormFieldProductNameDuplicated,
    #[error("Insert a new currency is failed")]
    InsertCurrencyFailed,
    #[error("Insert a new store is failed")]
    InsertStoreFailed,
    #[error("Insert a new product is failed")]
    InsertProductFailed,
    #[error("Insert a new receipt is failed")]
    InsertReceiptFailed,
    #[error("Insert a new inventory is failed")]
    InsertInventoryFailed
}