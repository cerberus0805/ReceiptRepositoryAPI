use diesel::{
    dsl::{count, exists, select}, insert_into, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper
};

use crate::{
    models::v1::{
        collections::service_collection::ServiceCollection, entities::entity_product::{EntityProduct, NewEntityProduct}, errors::api_error::ApiError, parameters::pagination::Pagination, responses::response_product::ResponseProduct
    }, 
    repository::DbRepository, 
    schema::products, 
    services::v1::{converters::converters_service::ConverterService, fallbacks::fallbacks_service::FallbacksService}
};

pub struct ProductService<'a> {
    repository: &'a DbRepository
}

impl<'a> ProductService<'a> {
    pub fn new(repository: &'a DbRepository) -> Self {
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
            tracing::warn!("try to get a non existed product ({}): {}", id, e);
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

    pub async fn is_product_existed_by_id(&self, id: i32) -> Result<bool, ApiError> {
        let conn = &mut self.repository.pool.get().or_else(|e| {
            tracing::error!("database connection broken: {}", e);
            Err(ApiError::DatabaseConnectionBroken)
        })?;

        select(exists(products::table.filter(products::id.eq(id)))).get_result::<bool>(conn).or_else(|_e| {
            Err(ApiError::FormFieldCurrencyIdNotExisted)  
        })
    }

    pub async fn is_product_existed_by_name(&self, name: &String, brand: Option<&String>, spec_amount: Option<&i32>, spec_unit: Option<&String>, spec_others: Option<&String>) -> Result<bool, ApiError> {
        let conn = &mut self.repository.pool.get().or_else(|e| {
            tracing::error!("database connection broken: {}", e);
            Err(ApiError::DatabaseConnectionBroken)
        })?;
        let mut product_filter = products::table.into_boxed()
            .filter(products::name.eq(name));
        
        if brand.is_some() {
            product_filter = product_filter.filter(products::brand.eq(brand.expect("brand should not be none")));
        }

        if spec_amount.is_some() {
            product_filter = product_filter.filter(products::specification_amount.eq(spec_amount.expect("spec_amount should not be none")));
        }
        
        if spec_unit.is_some() {
            product_filter = product_filter.filter(products::specification_unit.eq(spec_unit.expect("spec_unit should not be none")));
        }
        
        if spec_others.is_some() {
            product_filter = product_filter.filter(products::specification_others.eq(spec_others.expect("spec_others should not be none")));
        }

        select(exists(product_filter)).get_result::<bool>(conn).or_else(|_e| {
            Err(ApiError::FormFieldCurrencyIdNotExisted)  
        })
    }

    pub async fn new_product(&self, product: &NewEntityProduct) -> Result<i32, ApiError> {
        let conn = &mut self.repository.pool.get().or_else(|e| {
            tracing::error!("database connection broken: {}", e);
            Err(ApiError::DatabaseConnectionBroken)
        })?;

        let entity_product = insert_into(products::table)
            .values(product)
            .get_result::<EntityProduct>(conn).or_else(|e| {
                tracing::error!("insert product entity failed: {}", e);
                Err(ApiError::InsertProductFailed)
        })?;

        Ok(entity_product.id)
    }
}