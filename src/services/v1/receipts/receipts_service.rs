use std::collections::HashMap;

use bigdecimal::ToPrimitive;
use diesel::{
    ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper
};

use crate::{
    models::v1::{
        entities::{
            entity_currency::EntityCurrency, entity_inventory::EntityInventory, entity_product::EntityProduct, entity_receipt::EntityReceipt, entity_store::EntityStore
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

        let receipt_query = 
            receipts::table
                .inner_join(currencies::table)
                .inner_join(stores::table)
                .filter(receipts::id.eq(id))
                .select(<(EntityReceipt, EntityCurrency, EntityStore)>::as_select());

        let (receipt, currency, store) = receipt_query.get_result::<(EntityReceipt, EntityCurrency, EntityStore)>(conn)?;

        let inventories_query = 
            inventories::table
                .inner_join(products::table)
                .filter(inventories::receipt_id.eq(receipt.id))
                .select(<(EntityInventory, EntityProduct)>::as_select());

        let inventories_products = inventories_query.get_results::<(EntityInventory, EntityProduct)>(conn)?;

        let mut inventories = vec![];
        for inventory_product in inventories_products {
            inventories.push(self.convert_to_inventory_response(inventory_product.0, inventory_product.1));
        }


        let receipt_response = self.convert_to_receipt_response(receipt, currency, store, inventories);

        Ok(receipt_response)
    }

    pub async fn get_receipts(&self) -> Result<Vec<ResponseReceipt>, diesel::result::Error> {
        let conn = &mut self.repository.pool.get().unwrap();
        let all_compound_receipts_query = 
            receipts::table
                .inner_join(currencies::table)
                .inner_join(stores::table)
                .select(<(EntityReceipt, EntityCurrency, EntityStore)>::as_select());

        let all_compound_receipts = all_compound_receipts_query.get_results::<(EntityReceipt, EntityCurrency, EntityStore)>(conn)?;

        let all_compound_inventories_query =
            inventories::table
                .inner_join(products::table)
                .select(<(EntityInventory, EntityProduct)>::as_select());
        
        let all_compound_inventories = all_compound_inventories_query.get_results::<(EntityInventory, EntityProduct)>(conn)?;

        Ok(self.convert_to_all_receipt_response(all_compound_receipts, all_compound_inventories))
    }

    fn convert_to_all_receipt_response(&self, compound_receipts: Vec<(EntityReceipt, EntityCurrency, EntityStore)>, compound_inventories: Vec<(EntityInventory, EntityProduct)>) -> Vec<ResponseReceipt> {
        let mut receipts = vec![];
        let mut compound_inventories_hash_map: HashMap<i32, Vec<(EntityInventory, EntityProduct)>> = HashMap::<i32, Vec<(EntityInventory, EntityProduct)>>::new();
        for tuple in compound_inventories {
            if compound_inventories_hash_map.contains_key(&tuple.0.receipt_id) {
                compound_inventories_hash_map.get_mut(&tuple.0.receipt_id).unwrap().push(tuple);
            }
            else {
                compound_inventories_hash_map.insert(tuple.0.receipt_id, vec![tuple]);
            }
        }
        

        for receipt_currency_store in compound_receipts {
            let id = &receipt_currency_store.0.id;
            receipts.push(
                self.convert_to_receipt_response(
                    receipt_currency_store.0.clone(), 
                    receipt_currency_store.1, 
                    receipt_currency_store.2, 
                    self.convert_to_all_inventories_response(compound_inventories_hash_map[id].clone())
                )
            );
        }

        receipts
    }

    fn convert_to_all_inventories_response(&self, compound_inventories: Vec<(EntityInventory, EntityProduct)>) -> Vec<ResponseInventory> {
        let mut inventories = vec![];
        for inventory_product in compound_inventories {
            inventories.push(self.convert_to_inventory_response(inventory_product.0, inventory_product.1));
        }

        inventories
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