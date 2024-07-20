use std::collections::HashMap;
use bigdecimal::ToPrimitive;

use crate::models::v1::{entities::{entity_currency::EntityCurrency, entity_inventory::EntityInventory, entity_product::EntityProduct, entity_receipt::EntityReceipt, entity_store::EntityStore}, responses::{response_currency::ResponseCurrency, response_inventory::{ResponseCustomizedInventory, ResponseInventory}, response_product::ResponseProduct, response_receipt::ResponseReceipt, response_store::ResponseStore}};

pub struct ConverterService {
}

impl ConverterService {
    pub fn new() -> Self {
        Self {}
    }

    pub fn convert_to_all_receipt_response(&self, compound_receipts: Vec<(EntityReceipt, EntityCurrency, EntityStore)>, compound_inventories: Vec<(EntityInventory, EntityProduct)>) -> Vec<ResponseReceipt> {
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

    pub fn convert_to_all_inventories_response(&self, compound_inventories: Vec<(EntityInventory, EntityProduct)>) -> Vec<ResponseInventory> {
        let mut inventories = vec![];
        for inventory_product in compound_inventories {
            inventories.push(self.convert_to_inventory_response(inventory_product.0, inventory_product.1));
        }

        inventories
    }

    pub fn convert_to_all_stores_response(&self, entity_stores: Vec<EntityStore>) ->  Vec<ResponseStore> {
        let stores: Vec<ResponseStore> = entity_stores.into_iter().map(|es| self.convert_to_store_response(es)).collect();
        stores
    }

    pub fn convert_to_all_currencies_response(&self, entity_currencies: Vec<EntityCurrency>) ->  Vec<ResponseCurrency> {
        let currencies: Vec<ResponseCurrency> = entity_currencies.into_iter().map(|ec| self.convert_to_currency_response(ec)).collect();
        currencies
    }

    pub fn convert_to_all_products_response(&self, entity_products: Vec<EntityProduct>) ->  Vec<ResponseProduct> {
        let products: Vec<ResponseProduct> = entity_products.into_iter().map(|ep| self.convert_to_product_response(ep)).collect();
        products
    }

    pub fn convert_to_currency_response(&self, currency: EntityCurrency) -> ResponseCurrency {
        ResponseCurrency {
            id: currency.id,
            name: currency.name
        }
    }

    pub fn convert_to_store_response(&self, store: EntityStore) -> ResponseStore {
        ResponseStore {
            id: store.id,
            name: store.name,
            alias: store.alias,
            branch: store.branch,
            address: store.address
        }
    }

    pub fn convert_to_product_response(&self, product: EntityProduct) -> ResponseProduct {
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

    pub fn convert_to_inventory_response(&self, inventory: EntityInventory, product: EntityProduct) -> ResponseInventory {
        ResponseInventory {
            id: inventory.id,
            price: inventory.price.to_f64().unwrap(),
            quantity: inventory.quantity,
            product: self.convert_to_product_response(product)
        }
    }

    pub fn convert_to_receipt_response(&self, receipt: EntityReceipt, currency: EntityCurrency, store: EntityStore, inventories: Vec<ResponseInventory>) -> ResponseReceipt {
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

    pub fn convert_to_customized_inventory_response(&self, inventory: EntityInventory, product: EntityProduct, receipt: EntityReceipt, store: EntityStore, currency: EntityCurrency) -> ResponseCustomizedInventory {
        let customized_inventory = ResponseCustomizedInventory {
            id: inventory.id,
            product: self.convert_to_product_response(product),
            price: inventory.price.to_f64().unwrap(),
            receipt_id: receipt.id,
            taxed: receipt.is_inventory_taxed,
            transaction_date: receipt.transaction_date,
            store_id: store.id,
            store_name: store.name,
            store_alias: store.alias,
            currency: self.convert_to_currency_response(currency)
        };

        customized_inventory
    }

    pub fn convert_to_customized_inventories_response(&self, compound_inventories: Vec<(EntityInventory, EntityProduct)>, compound_receipts: Vec<(EntityReceipt, EntityStore, EntityCurrency)>) -> Vec<ResponseCustomizedInventory> {
        let receipt_tuple_hash_map = compound_receipts.into_iter().map(|t| (t.0.id, t)).collect::<HashMap<i32, (EntityReceipt, EntityStore, EntityCurrency)>>();
        let customized_inventories = compound_inventories.into_iter().map(|t| {
            let compound_receipt = receipt_tuple_hash_map.get(&t.0.receipt_id).unwrap();
            ResponseCustomizedInventory {
                id: t.0.id,
                product: self.convert_to_product_response(t.1),
                price: t.0.price.to_f64().unwrap(),
                receipt_id: t.0.receipt_id,
                taxed: compound_receipt.0.is_inventory_taxed,
                transaction_date: compound_receipt.0.transaction_date,
                store_id: compound_receipt.0.id,
                store_name: compound_receipt.1.name.clone(),
                store_alias: compound_receipt.1.alias.clone(),
                currency: self.convert_to_currency_response(compound_receipt.2.clone())
            }
        }).collect();

        customized_inventories
    }
}