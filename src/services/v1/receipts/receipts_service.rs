use std::collections::HashMap;

use bigdecimal::ToPrimitive;
use diesel::{
    dsl::count, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper
};

use crate::{
    models::v1::{
        collections::service_collection::ServiceCollection, entities::{
            entity_currency::EntityCurrency, entity_inventory::EntityInventory, entity_product::EntityProduct, entity_receipt::EntityReceipt, entity_store::EntityStore
        }, errors::api_error::ApiError, parameters::pagination::{Pagination, DEFAULT_LIMIT, DEFAULT_OFFSET, MAX_LIMIT}, responses::{
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

    pub async fn get_receipt(&self, id: i32) -> Result<ResponseReceipt, ApiError> {
        let conn = &mut self.repository.pool.get().or_else(
            |e| {
                tracing::error!("database connection broken: {}", e);
                Err(ApiError::DatabaseConnectionBroken)
            })?;

        let receipt_query = 
            receipts::table
                .inner_join(currencies::table)
                .inner_join(stores::table)
                .filter(receipts::id.eq(id))
                .select(<(EntityReceipt, EntityCurrency, EntityStore)>::as_select());

        let (receipt, currency, store) = receipt_query.get_result::<(EntityReceipt, EntityCurrency, EntityStore)>(conn).or_else(
            |e| {
                tracing::warn!("try to get a non existed receipt ({}): {}", id, e);
                Err(ApiError::NoRecord)
            })?;

        let inventories_query = 
            inventories::table
                .inner_join(products::table)
                .filter(inventories::receipt_id.eq(receipt.id))
                .select(<(EntityInventory, EntityProduct)>::as_select());

        let inventories_products = inventories_query.get_results::<(EntityInventory, EntityProduct)>(conn).or_else(|_e| Err(ApiError::NoRecord))?;

        let mut inventories = vec![];
        for inventory_product in inventories_products {
            inventories.push(self.convert_to_inventory_response(inventory_product.0, inventory_product.1));
        }


        let receipt_response = self.convert_to_receipt_response(receipt, currency, store, inventories);

        Ok(receipt_response)
    }

    pub async fn get_receipts(&self, pagination: Pagination) -> Result<ServiceCollection<ResponseReceipt>, ApiError> {
        let conn = &mut self.repository.pool.get().or_else(
            |e| {
                tracing::error!("database connection broken: {}", e);
                Err(ApiError::DatabaseConnectionBroken)
            })?;

        let count: i64 = receipts::table.select(count(receipts::columns::id)).first(conn).or_else(|_e| Err(ApiError::NoRecord))?;
        
        let mut page_offset: i64 = pagination.offset;
        let mut per_page: i64 = pagination.limit;
        if page_offset < 0 {
            tracing::debug!("fallback to default offset for negative offset");
            page_offset = DEFAULT_OFFSET;
        }

        if per_page < 1 {
            tracing::debug!("fallback to default limit for negative or zeor offset");
            per_page = DEFAULT_LIMIT;
        }

        if per_page > MAX_LIMIT {
            tracing::debug!("fallback to max limit due to max limit exceeds");
            per_page = MAX_LIMIT;
        }

        let all_compound_receipts_in_this_page_query = 
            receipts::table
                .inner_join(currencies::table)
                .inner_join(stores::table)
                .limit(per_page)
                .offset(page_offset)
                .select(<(EntityReceipt, EntityCurrency, EntityStore)>::as_select());

        let all_compound_receipts_in_this_page = all_compound_receipts_in_this_page_query.get_results::<(EntityReceipt, EntityCurrency, EntityStore)>(conn).or_else(|_e| Err(ApiError::NoRecord))?;
        let receipts_ids = AsRef::<Vec<(EntityReceipt, EntityCurrency, EntityStore)>>::as_ref(&all_compound_receipts_in_this_page).into_iter().map(|x| x.0.id as i32).collect::<Vec<i32>>();

        let all_compound_inventories_in_this_page_query =
            inventories::table
                .inner_join(products::table)
                .filter(inventories::columns::receipt_id.eq_any(receipts_ids))
                .select(<(EntityInventory, EntityProduct)>::as_select());
        
        let all_compound_inventories_in_this_page = all_compound_inventories_in_this_page_query.get_results::<(EntityInventory, EntityProduct)>(conn).or_else(|_e| Err(ApiError::NoRecord))?;

        Ok(ServiceCollection {
            partial_collection: self.convert_to_all_receipt_response(all_compound_receipts_in_this_page, all_compound_inventories_in_this_page),
            total_count: count
        })
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