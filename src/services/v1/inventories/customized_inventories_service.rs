use diesel::{
    dsl::count, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper
};

use crate::{
    models::v1::{
        collections::service_collection::ServiceCollection, 
        entities::{
            entity_currency::EntityCurrency, entity_inventory::EntityInventory, entity_product::EntityProduct, entity_receipt::EntityReceipt, entity_store::EntityStore
        }, 
        errors::api_error::ApiError, 
        parameters::pagination::Pagination, 
        responses::response_inventory::ResponseCustomizedInventory
    }, 
    repository::DbRepository, 
    schema::{
        currencies, inventories, products, receipts, stores
    }, 
    services::v1::{
        converters::converters_service::ConverterService, 
        fallbacks::fallbacks_service::FallbacksService
    }
};

pub struct CustomizedInventroyService {
    pub repository: DbRepository
}

impl CustomizedInventroyService {
    pub fn new(repository: DbRepository) -> Self {
        Self {
            repository
        }
    }

    pub async fn get_customized_inventory(&self, id: i32) -> Result<ResponseCustomizedInventory, ApiError> {
        let converter = ConverterService::new();
        let conn = &mut self.repository.pool.get().or_else(
            |e| {
                tracing::error!("database connection broken: {}", e);
                Err(ApiError::DatabaseConnectionBroken)
            }
        )?;

        let inventory_query = 
            inventories::table
                .inner_join(products::table)
                .filter(inventories::id.eq(id))
                .select(<(EntityInventory, EntityProduct)>::as_select());

        let (inventory, product) = inventory_query.get_result::<(EntityInventory, EntityProduct)>(conn).or_else(
            |e| {
                tracing::warn!("try to get a non existed inventory ({}): {}", id, e);
                Err(ApiError::NoRecord)
            }
        )?;

        let receipt_store_currency_by_receipt_query = 
            receipts::table
                .inner_join(stores::table)
                .inner_join(currencies::table)
                .filter(receipts::id.eq(inventory.receipt_id))
                .select(<(EntityReceipt, EntityStore, EntityCurrency)>::as_select());

        let (receipt, store, currency) = receipt_store_currency_by_receipt_query.get_result::<(EntityReceipt, EntityStore, EntityCurrency)>(conn).or_else(
            |e| {
                tracing::warn!("try to get a non existed receipt ({}): {}", inventory.id, e);
                Err(ApiError::NoRecord)
            }
        )?;
            


        let customized_inventory_response = converter.convert_to_customized_inventory_response(inventory, product, receipt, store, currency);

        Ok(customized_inventory_response)
    }

    pub async fn get_customized_inventories(&self, pagination: Pagination) -> Result<ServiceCollection<ResponseCustomizedInventory>, ApiError> {
        let converter = ConverterService::new();
        let fallbacks_service = FallbacksService::new();
        let conn = &mut self.repository.pool.get().or_else(
            |e| {
                tracing::error!("database connection broken: {}", e);
                Err(ApiError::DatabaseConnectionBroken)
            })?;

        let count: i64 = inventories::table.select(count(inventories::columns::id)).first(conn).or_else(|_e| Err(ApiError::NoRecord))?;
        
        let (page_offset, per_page) = fallbacks_service.fallback_pagination(&pagination);

        let all_compound_inventories_in_this_page_query = 
            inventories::table
                .inner_join(products::table)
                .limit(per_page)
                .offset(page_offset)
                .select(<(EntityInventory, EntityProduct)>::as_select());

        let all_compound_inventories_in_this_page = all_compound_inventories_in_this_page_query.get_results::<(EntityInventory, EntityProduct)>(conn).or_else(|_e| Err(ApiError::NoRecord))?;

        let receipt_ids = AsRef::<Vec<(EntityInventory, EntityProduct)>>::as_ref(&all_compound_inventories_in_this_page).into_iter().map(|x| x.0.receipt_id as i32).collect::<Vec<i32>>();

        let all_related_receipts_store_currency_in_this_page_query = 
            receipts::table
                .inner_join(stores::table)
                .inner_join(currencies::table)
                .filter(receipts::columns::id.eq_any(receipt_ids))
                .select(<(EntityReceipt, EntityStore, EntityCurrency)>::as_select());

        let all_related_receipts_store_currency_in_this_page = all_related_receipts_store_currency_in_this_page_query.get_results::<(EntityReceipt, EntityStore, EntityCurrency)>(conn).or_else(|_e| Err(ApiError::NoRecord))?;


        Ok(ServiceCollection {
            partial_collection: converter.convert_to_customized_inventories_response(all_compound_inventories_in_this_page, all_related_receipts_store_currency_in_this_page),
            total_count: count
        })
    }
}