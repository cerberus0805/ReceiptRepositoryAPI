use diesel::{
    dsl::count, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper
};

use crate::{models::v1::{collections::service_collection::ServiceCollection, entities::entity_currency::EntityCurrency, errors::api_error::ApiError, parameters::pagination::Pagination, responses::response_currency::ResponseCurrency}, repository::DbRepository, schema::currencies, services::v1::{converters::converters_service::ConverterService, fallbacks::fallbacks_service::FallbacksService}};

pub struct CurrencyService {
    pub repository: DbRepository
}

impl CurrencyService {
    pub fn new(repository: DbRepository) -> Self {
        Self {
            repository
        }
    }

    pub async fn get_currency(&self, id: i32) -> Result<ResponseCurrency, ApiError> {
        let converter = ConverterService::new();
        let conn = &mut self.repository.pool.get().or_else(|e| {
            tracing::error!("database connection broken: {}", e);
            Err(ApiError::DatabaseConnectionBroken)
        })?;

        let currency_query = 
            currencies::table
            .filter(currencies::id.eq(id))
            .select(<EntityCurrency>::as_select());

        let currency = currency_query.get_result::<EntityCurrency>(conn).or_else(|e| {
            tracing::warn!("try to get a non existed store ({}): {}", id, e);
            Err(ApiError::NoRecord)
        })?;

        let currency_response = converter.convert_to_currency_response(currency);
        Ok(currency_response)
    }

    pub async fn get_currencies(&self, pagination: Pagination) -> Result<ServiceCollection<ResponseCurrency>, ApiError> {
        let converter: ConverterService = ConverterService::new();
        let fallbacks_service = FallbacksService::new();
        let conn = &mut self.repository.pool.get().or_else(|e| {
            tracing::error!("database connection broken: {}", e);
            Err(ApiError::DatabaseConnectionBroken)
        })?;

        let count: i64 = currencies::table.select(count(currencies::columns::id)).first(conn).or_else(|_e| Err(ApiError::NoRecord))?;
        
        let (page_offset, per_page) = fallbacks_service.fallback_pagination(&pagination);

        let currencies_in_this_page_query = 
            currencies::table
                .limit(per_page)
                .offset(page_offset)
                .select(<EntityCurrency>::as_select());

        let currencies_in_this_page = currencies_in_this_page_query.get_results::<EntityCurrency>(conn).or_else(|_e| Err(ApiError::NoRecord))?;

        Ok({
            ServiceCollection { 
                partial_collection: converter.convert_to_all_currencies_response(currencies_in_this_page),
                total_count: count
            }
        })
    }
}