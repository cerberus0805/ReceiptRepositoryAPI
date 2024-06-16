use bigdecimal::ToPrimitive;
use diesel::{
    query_dsl::{
        methods::FilterDsl, 
        methods::SelectDsl
    }, 
    ExpressionMethods, 
    RunQueryDsl, SelectableHelper
};

use crate::{
    models::v1::{
        entities::{
            entity_currency::EntityCurrency, 
            entity_inventory::EntityInventory, 
            entity_product::EntityProduct, 
            entity_receipt::EntityReceipt, 
            entity_store::EntityStore
        }, 
        responses::{
            response_currency::ResponseCurrency, 
            response_inventory::ResponseInventory, 
            response_product::ResponseProduct, 
            response_receipt::ResponseReceipt, 
            response_store::ResponseStore
        }
    }, 
    repository::DbRepository, 
    schema::{
        currencies, inventories, products, receipts, stores
    }
};

#[derive(Clone)]
pub struct ReceiptService {
    pub repository: DbRepository
}

impl ReceiptService {
    pub fn new(repository: DbRepository) -> Self {
        Self {
            repository
        }
    }

    pub async fn get_receipt(&self, id: i32) -> Result<ResponseReceipt, diesel::result::Error> {
        let conn = &mut self.repository.pool.get().unwrap();

        let receipt = 
            receipts::table
                .filter(receipts::id.eq(id))
                .select(EntityReceipt::as_select())
                .get_result(conn)?;
        
        let store = 
            stores::table.filter(stores::id.eq(receipt.store_id))
            .select(EntityStore::as_select())
            .get_result(conn)?;

        let currency = 
            currencies::table.filter(currencies::id.eq(receipt.currency_id))
            .select(EntityCurrency::as_select())
            .get_result(conn)?;

        let inventories = 
            inventories::table.filter(inventories::receipt_id.eq(receipt.id))
            .select(EntityInventory::as_select())
            .get_results(conn)?;

        let mut response_inventories: Vec<ResponseInventory> = Vec::new();

        for inventory in inventories {
            let product = 
                products::table
                    .filter(products::id.eq(inventory.product_id))
                    .select(EntityProduct::as_select())
                    .get_result(conn)?;

            let response_inventory = self.convert_to_inventory_response(inventory, product);
            response_inventories.push(response_inventory)
        }

        let receipt_response = self.convert_to_receipt_response(receipt, currency, store, response_inventories);


        Ok(receipt_response)
    }

    fn convert_to_currency_response(&self, currency: EntityCurrency) -> ResponseCurrency {
        ResponseCurrency {
            id: currency.id,
            name: currency.name
        }
    }

    fn convert_to_store_response(&self, store: EntityStore) -> ResponseStore {
        ResponseStore {
            id: store.id,
            name: store.name,
            alias: store.alias,
            branch: store.branch,
            address: store.address
        }
    }

    fn convert_to_product_response(&self, product: EntityProduct) -> ResponseProduct {
        ResponseProduct {
            id: product.id,
            name: product.name,
            alias: product.alias,
            specification_amount: product.specification_amount,
            specification_others: product.specification_others,
            specification_unit: product.specification_unit,
            brand: product.brand
        }
    }

    fn convert_to_inventory_response(&self, inventory: EntityInventory, product: EntityProduct) -> ResponseInventory {
        ResponseInventory {
            id: inventory.id,
            price: inventory.price.to_f64().unwrap(),
            quantity: inventory.quantity,
            product: self.convert_to_product_response(product)
        }
    }

    fn convert_to_receipt_response(&self, receipt: EntityReceipt, currency: EntityCurrency, store: EntityStore, inventories: Vec<ResponseInventory>) -> ResponseReceipt {
        let response_receipt = ResponseReceipt {
            id: receipt.id,
            transaction_date: receipt.transaction_date,
            is_inventory_taxed: receipt.is_inventory_taxed,
            currency: self.convert_to_currency_response(currency),
            store: self.convert_to_store_response(store),
            inventories
        };

        response_receipt
    }
}