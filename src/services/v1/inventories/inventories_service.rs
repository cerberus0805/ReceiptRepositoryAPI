use diesel::{
    dsl::count, insert_into, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper
};

use crate::{models::v1::{collections::service_collection::ServiceCollection, entities::{entity_inventory::{EntityInventory, NewEntityInventory}, entity_product::EntityProduct}, errors::api_error::ApiError, parameters::pagination::Pagination, responses::response_inventory::ResponseInventory}, repository::DbRepository, schema::{inventories, products}, services::v1::{converters::converters_service::ConverterService, fallbacks::fallbacks_service::FallbacksService}};

pub struct InventroyService<'a> {
    repository: &'a DbRepository
}

impl<'a> InventroyService<'a> {
    pub fn new(repository: &'a DbRepository) -> Self {
        Self {
            repository
        }
    }

    pub async fn get_receipt(&self, id: i32) -> Result<ResponseInventory, ApiError> {
        let converter = ConverterService::new();
        let conn = &mut self.repository.pool.get().or_else(
            |e| {
                tracing::error!("database connection broken: {}", e);
                Err(ApiError::DatabaseConnectionBroken)
        })?;

        let inventory_query = 
            inventories::table
                .inner_join(products::table)
                .filter(inventories::id.eq(id))
                .select(<(EntityInventory, EntityProduct)>::as_select());

        let (inventory, product) = inventory_query.get_result::<(EntityInventory, EntityProduct)>(conn).or_else(
            |e| {
                tracing::warn!("try to get a non existed inventory ({}): {}", id, e);
                Err(ApiError::NoRecord)
        })?;


        let inventory_response = converter.convert_to_inventory_response(inventory, product);

        Ok(inventory_response)
    }

    pub async fn get_receipts(&self, pagination: Pagination) -> Result<ServiceCollection<ResponseInventory>, ApiError> {
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

        Ok(ServiceCollection {
            partial_collection: converter.convert_to_all_inventories_response(all_compound_inventories_in_this_page),
            total_count: count
        })
    }

    pub async fn new_inventory(&self, inventory: &NewEntityInventory) -> Result<i32, ApiError> {
        let conn = &mut self.repository.pool.get().or_else(
            |e| {
                tracing::error!("database connection broken: {}", e);
                Err(ApiError::DatabaseConnectionBroken)
        })?;

        let entity_inventory = insert_into(inventories::table)
            .values(inventory)
            .get_result::<EntityInventory>(conn).or_else(|e| {
                tracing::error!("insert inventory entity failed: {}", e);
                Err(ApiError::InsertInventoryFailed)
        })?;

        Ok(entity_inventory.id)
    }
}