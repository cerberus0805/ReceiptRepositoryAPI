use diesel::{
    dsl::count, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper
};

use crate::{
    models::v1::{
        collections::service_collection::ServiceCollection, entities::entity_store::EntityStore, errors::api_error::ApiError, parameters::pagination::Pagination, responses::response_store::ResponseStore
    }, 
    repository::DbRepository, 
    schema::stores, 
    services::v1::{converters::converters_service::ConverterService, fallbacks::fallbacks_service::FallbacksService}
};

pub struct StoreService {
    repository: DbRepository
}

impl StoreService {
    pub fn new(repository: DbRepository) -> Self {
        Self {
            repository
        }
    }

    pub async fn get_store(&self, id: i32) -> Result<ResponseStore, ApiError> {
        let converter = ConverterService::new();
        let conn = &mut self.repository.pool.get().or_else(|e| {
            tracing::error!("database connection broken: {}", e);
            Err(ApiError::DatabaseConnectionBroken)
        })?;

        let store_query = 
            stores::table
            .filter(stores::id.eq(id))
            .select(<EntityStore>::as_select());

        let store = store_query.get_result::<EntityStore>(conn).or_else(|e| {
            tracing::warn!("try to get a non existed store ({}): {}", id, e);
            Err(ApiError::NoRecord)
        })?;

        let store_response = converter.convert_to_store_response(store);
        Ok(store_response)
    }

    pub async fn get_stores(&self, pagination: Pagination) -> Result<ServiceCollection<ResponseStore>, ApiError> {
        let converter: ConverterService = ConverterService::new();
        let fallbacks_service = FallbacksService::new();
        let conn = &mut self.repository.pool.get().or_else(|e| {
            tracing::error!("database connection broken: {}", e);
            Err(ApiError::DatabaseConnectionBroken)
        })?;

        let count: i64 = stores::table.select(count(stores::columns::id)).first(conn).or_else(|_e| Err(ApiError::NoRecord))?;
        
        let (page_offset, per_page) = fallbacks_service.fallback_pagination(&pagination);

        let stores_in_this_page_query = 
            stores::table
                .limit(per_page)
                .offset(page_offset)
                .select(<EntityStore>::as_select());

        let stores_in_this_page = stores_in_this_page_query.get_results::<EntityStore>(conn).or_else(|_e| Err(ApiError::NoRecord))?;

        Ok({
            ServiceCollection { 
                partial_collection: converter.convert_to_all_stores_response(stores_in_this_page),
                total_count: count
            }
        })
    }
}
