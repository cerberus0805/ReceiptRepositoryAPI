use axum::{extract::Path, response::IntoResponse, Json};
use chrono::NaiveDate;

use crate::models::v1::responses::{response_currency::ResponseCurrency, response_inventory::ResponseInventory, response_product::ResponseProduct, response_receipt::{ReponseReceiptPayload, ResponseReceipt}, response_store::ResponseStore};

pub struct ReceiptsHandlers {
}

impl ReceiptsHandlers {
    pub async fn get_receipt(Path(id): Path<u32>) -> impl IntoResponse {
        println!("id: {}", id);
        let response_receipt = Self::get_mock_receipt();
        let payload = ReponseReceiptPayload {
            data: Some(response_receipt),
            error: None
        };

        Json(payload)
    }

    fn get_mock_receipt() -> ResponseReceipt {
        ResponseReceipt {
            id: 1,
            transaction_date: NaiveDate::from_ymd_opt(2013, 4, 5).unwrap().and_hms_opt(9, 48, 0).unwrap(),
            is_inventory_taxed: true,
            currency: ResponseCurrency {
                id: 1,
                name: "JPY".to_owned()
            },
            store: ResponseStore {
                id: 1,
                name: "セブンイレブン".to_string(),
                alias: Some("7-11 京都烏丸高辻店".to_string()),
                branch: Some("京都烏丸高辻店".to_string()),
                address: Some("京都府京都市下京区高辻通烏丸東入南側因幡堂町６６".to_string())
            },
            inventories: vec![ ResponseInventory {
                id: 1,
                product: ResponseProduct {
                    id: 1,
                    name: "明治おいしい牛乳".to_string(),
                    alias: Some("明治好喝牛奶".to_string()),
                    specification_amount: Some(500),
                    specification_unit: Some("ml".to_string()),
                    specification_size: None,
                    brand: Some("明治".to_string())
                },
                price: 148.0,
                quantity: 1
            }, ResponseInventory {
                id: 2,
                product: ResponseProduct{
                    id: 2,
                    name: "伊藤園 充実野菜".to_string(),
                    alias: None,
                    specification_amount: Some(350),
                    specification_unit: Some("g".to_string()),
                    specification_size: None,
                    brand: Some("伊藤園".to_string())
                },
                price: 157.0,
                quantity: 1
            }]
        }
    }
}