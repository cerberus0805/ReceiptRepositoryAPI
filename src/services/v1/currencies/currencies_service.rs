use diesel::{
    dsl::{count, exists, select}, insert_into, ExpressionMethods, QueryDsl, RunQueryDsl, SaveChangesDsl, SelectableHelper, TextExpressionMethods
};

use crate::{models::v1::{collections::service_collection::ServiceCollection, entities::entity_currency::{EntityCurrency, NewEntityCurrency, UpdateEntityCurrency}, errors::api_error::ApiError, forms::patch_payload::PatchCurrencyPayload, parameters::pagination::Pagination, responses::response_currency::ResponseCurrency}, repository::DbRepository, schema::currencies, services::v1::{converters::converters_service::ConverterService, fallbacks::fallbacks_service::FallbacksService}};

pub struct CurrencyService<'a> {
    repository: &'a DbRepository
}

impl<'a> CurrencyService<'a> {
    pub fn new(repository: &'a DbRepository) -> Self {
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
            tracing::warn!("try to get a non existed currency ({}): {}", id, e);
            Err(ApiError::NoRecord)
        })?;

        let currency_response = converter.convert_to_currency_response(currency);
        Ok(currency_response)
    }

    pub async fn get_currencies(&self, pagination: &Pagination) -> Result<ServiceCollection<ResponseCurrency>, ApiError> {
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

    pub async fn is_currency_existed_by_id(&self, id: i32) -> Result<bool, ApiError> {
        let conn = &mut self.repository.pool.get().or_else(|e| {
            tracing::error!("database connection broken: {}", e);
            Err(ApiError::DatabaseConnectionBroken)
        })?;

        select(exists(currencies::table.filter(currencies::id.eq(id)))).get_result::<bool>(conn).or_else(|_e| {
            Err(ApiError::CurrencyIdNotExisted)  
        })
    }

    pub async fn is_currency_existed_by_name(&self, name: &String) -> Result<bool, ApiError> {
        let conn = &mut self.repository.pool.get().or_else(|e| {
            tracing::error!("database connection broken: {}", e);
            Err(ApiError::DatabaseConnectionBroken)
        })?;

        select(exists(currencies::table.filter(currencies::name.eq(name)))).get_result::<bool>(conn).or_else(|_e| {
            Err(ApiError::CurrencyIdNotExisted)  
        })
    }

    pub async fn new_currency(&self, currency: &NewEntityCurrency) -> Result<i32, ApiError> {
        let conn = &mut self.repository.pool.get().or_else(|e| {
            tracing::error!("database connection broken: {}", e);
            Err(ApiError::DatabaseConnectionBroken)
        })?;

        let entity_currency = insert_into(currencies::table)
            .values(currency)
            .get_result::<EntityCurrency>(conn).or_else(|e| {
                tracing::error!("insert currency entity failed: {}", e);
                Err(ApiError::InsertCurrencyFailed)
            })?;

        Ok(entity_currency.id)
    }

    pub async fn patch_currency(&self, id: i32, patch_payload: &PatchCurrencyPayload) -> Result<(), ApiError> {
        let conn = &mut self.repository.pool.get().or_else(|e| {
            tracing::error!("database connection broken: {}", e);
            Err(ApiError::DatabaseConnectionBroken)
        })?;

        let update_currency = UpdateEntityCurrency {
            id,
            name: &patch_payload.name
        };

        update_currency.save_changes::<EntityCurrency>(conn).or_else(|e| {
            tracing::error!("update currency entity failed: {}", e);
            Err(ApiError::UpdateCurrencyFailed)
        })?;

        tracing::debug!("patch currency {} successfully", id);
        Ok(())
    }

    pub async fn autocomplete_currencies(&self, keyword: &Option<String>) -> Result<ServiceCollection<ResponseCurrency>, ApiError> {
        let converter: ConverterService = ConverterService::new();
        let conn = &mut self.repository.pool.get().or_else(|e| {
            tracing::error!("database connection broken: {}", e);
            Err(ApiError::DatabaseConnectionBroken)
        })?;

        let build_query = || {
            let mut sql_filters = currencies::table.limit(20).into_boxed();
            if let Some(kw) = &keyword {
                let currency_name_pattern = format!("%{}%", kw);
                sql_filters = sql_filters.filter(currencies::name.like(currency_name_pattern))
            }

            sql_filters
        };

        let count: i64 = build_query().select(count(currencies::columns::name)).first(conn).or_else(|_e| Err(ApiError::NoRecord))?;

        let currencies_query = build_query().select(<EntityCurrency>::as_select());

        let currencies_list = currencies_query.get_results::<EntityCurrency>(conn).or_else(|_e| Err(ApiError::NoRecord))?;

        Ok({
            ServiceCollection { 
                partial_collection: converter.convert_to_all_currencies_response(currencies_list),
                total_count: count
            }
        })
    }
}