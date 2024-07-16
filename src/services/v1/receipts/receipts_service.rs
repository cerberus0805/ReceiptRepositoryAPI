use diesel::{
    dsl::count, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper
};

use crate::{
    models::v1::{
        collections::service_collection::ServiceCollection, entities::{
            entity_currency::EntityCurrency, entity_inventory::EntityInventory, entity_product::EntityProduct, entity_receipt::EntityReceipt, entity_store::EntityStore
        }, errors::api_error::ApiError, parameters::pagination::Pagination, responses::response_receipt::ResponseReceipt
    }, 
    repository::DbRepository, 
    schema::{
        currencies, inventories, products, receipts, stores
    }, services::v1::{converters::converters_service::ConverterService, fallbacks::fallbacks_service::FallbacksService}
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
        let converter = ConverterService::new();
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
            inventories.push(converter.convert_to_inventory_response(inventory_product.0, inventory_product.1));
        }


        let receipt_response = converter.convert_to_receipt_response(receipt, currency, store, inventories);

        Ok(receipt_response)
    }

    pub async fn get_receipts(&self, pagination: Pagination) -> Result<ServiceCollection<ResponseReceipt>, ApiError> {
        let converter = ConverterService::new();
        let fallbacks_service = FallbacksService::new();
        let conn = &mut self.repository.pool.get().or_else(
            |e| {
                tracing::error!("database connection broken: {}", e);
                Err(ApiError::DatabaseConnectionBroken)
            })?;

        let count: i64 = receipts::table.select(count(receipts::columns::id)).first(conn).or_else(|_e| Err(ApiError::NoRecord))?;
        
        let (page_offset, per_page) = fallbacks_service.fallback_pagination(&pagination);

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
            partial_collection: converter.convert_to_all_receipt_response(all_compound_receipts_in_this_page, all_compound_inventories_in_this_page),
            total_count: count
        })
    }
}