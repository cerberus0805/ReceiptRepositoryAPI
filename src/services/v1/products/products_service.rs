use diesel::{
    dsl::count, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper
};

use crate::{
    models::v1::{
        collections::service_collection::ServiceCollection, entities::entity_product::EntityProduct, errors::api_error::ApiError, parameters::pagination::Pagination, responses::response_product::ResponseProduct
    }, 
    repository::DbRepository, 
    schema::products, 
    services::v1::{converters::converters_service::ConverterService, fallbacks::fallbacks_service::FallbacksService}
};

pub struct ProductService {
    pub repository: DbRepository
}

impl ProductService {
    pub fn new(repository: DbRepository) -> Self {
        Self {
            repository
        }
    }

    pub async fn get_product(&self, id: i32) -> Result<ResponseProduct, ApiError> {
        let converter = ConverterService::new();
        let conn = &mut self.repository.pool.get().or_else(|e| {
            tracing::error!("database connection broken: {}", e);
            Err(ApiError::DatabaseConnectionBroken)
        })?;

        let product_query = 
            products::table
            .filter(products::id.eq(id))
            .select(<EntityProduct>::as_select());

        let product = product_query.get_result::<EntityProduct>(conn).or_else(|e| {
            tracing::warn!("try to get a non existed store ({}): {}", id, e);
            Err(ApiError::NoRecord)
        })?;

        let product_response = converter.convert_to_product_response(product);
        Ok(product_response)
    }

    pub async fn get_products(&self, pagination: Pagination) -> Result<ServiceCollection<ResponseProduct>, ApiError> {
        let converter: ConverterService = ConverterService::new();
        let fallbacks_service = FallbacksService::new();
        let conn = &mut self.repository.pool.get().or_else(|e| {
            tracing::error!("database connection broken: {}", e);
            Err(ApiError::DatabaseConnectionBroken)
        })?;

        let count: i64 = products::table.select(count(products::columns::id)).first(conn).or_else(|_e| Err(ApiError::NoRecord))?;
        
        let (page_offset, per_page) = fallbacks_service.fallback_pagination(&pagination);

        let products_in_this_page_query = 
            products::table
                .limit(per_page)
                .offset(page_offset)
                .select(<EntityProduct>::as_select());

        let products_in_this_page = products_in_this_page_query.get_results::<EntityProduct>(conn).or_else(|_e| Err(ApiError::NoRecord))?;

        Ok({
            ServiceCollection { 
                partial_collection: converter.convert_to_all_products_response(products_in_this_page),
                total_count: count
            }
        })
    }
}