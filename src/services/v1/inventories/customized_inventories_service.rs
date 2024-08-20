use chrono::{NaiveDateTime, NaiveTime};
use diesel::{
    dsl::count, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper, TextExpressionMethods
};

use crate::{
    models::v1::{
        collections::service_collection::ServiceCollection, 
        entities::{
            entity_currency::EntityCurrency, entity_inventory::EntityInventory, entity_product::EntityProduct, entity_receipt::EntityReceipt, entity_store::EntityStore
        }, 
        errors::api_error::ApiError, 
        parameters::{pagination::Pagination, query_filters::QueryFilters}, 
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

pub struct CustomizedInventoryService<'a> {
    repository: &'a DbRepository
}

impl<'a> CustomizedInventoryService<'a> {
    pub fn new(repository: &'a DbRepository) -> Self {
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

    pub async fn get_customized_inventories(&self, pagination: &Pagination, query_filters: &QueryFilters) -> Result<ServiceCollection<ResponseCustomizedInventory>, ApiError> {
        let converter = ConverterService::new();
        let fallbacks_service = FallbacksService::new();
        let conn = &mut self.repository.pool.get().or_else(
            |e| {
                tracing::error!("database connection broken: {}", e);
                Err(ApiError::DatabaseConnectionBroken)
            }
        )?;
        
        let (page_offset, per_page) = fallbacks_service.fallback_pagination(&pagination);

        let build_query = || {
            let mut sql_filters = inventories::table
                .inner_join(products::table)
                .inner_join(receipts::table.inner_join(stores::table).inner_join(currencies::table))
                .into_boxed();
            if let Some(product_name) = &query_filters.product_name {
                let product_name_pattern = format!("%{}%", product_name);
                sql_filters = sql_filters.filter(products::name.like(product_name_pattern))
            }
            if let Some(product_brand) = &query_filters.product_brand {
                let product_brand_pattern = format!("%{}%", product_brand);
                sql_filters = sql_filters.filter(products::brand.like(product_brand_pattern))
            }
            if let Some(product_alias) = &&query_filters.product_alias {
                let product_alias_pattern = format!("%{}%", product_alias);
                sql_filters = sql_filters.filter(products::alias.like(product_alias_pattern))
            }

            if query_filters.start_date.is_some() && query_filters.end_date.is_some() {
                let start = NaiveDateTime::new(query_filters.start_date.expect("start date should not be none"), NaiveTime::MIN);
                let end = NaiveDateTime::new(query_filters.end_date.expect("end date should not be none"), NaiveTime::from_hms_milli_opt(23, 59, 59, 999).expect("23:59:59.999 should be unwraped without error"));
                sql_filters = sql_filters.filter(receipts::transaction_date.ge(start));
                sql_filters = sql_filters.filter(receipts::transaction_date.le(end));
            }

            if let Some(currency_name) = &query_filters.currency {
                sql_filters = sql_filters.filter(currencies::name.eq(currency_name))
            }

            if let Some(store_name) = &query_filters.store_name {
                let store_name_pattern = format!("%{}%", store_name);
                sql_filters = sql_filters.filter(stores::name.like(store_name_pattern));
            }

            if let Some(store_alias) = &query_filters.store_alias {
                let store_alias_pattern = format!("%{}%", store_alias);
                sql_filters = sql_filters.filter(stores::alias.like(store_alias_pattern));
            }

            sql_filters
        };

        let count: i64 = build_query().select(count(inventories::columns::id)).first(conn).or_else(|_e| Err(ApiError::NoRecord))?;

        let all_compound_inventories_in_this_page_query = 
            build_query()
                .limit(per_page)
                .offset(page_offset)
                .select(<(EntityInventory, EntityProduct, EntityReceipt, EntityStore, EntityCurrency)>::as_select());

        let all_compound_inventories_in_this_page = all_compound_inventories_in_this_page_query.get_results::<(EntityInventory, EntityProduct, EntityReceipt, EntityStore, EntityCurrency)>(conn).or_else(|_e| Err(ApiError::NoRecord))?;

        Ok(ServiceCollection {
            partial_collection: all_compound_inventories_in_this_page.into_iter().map(|inv| converter.convert_to_customized_inventory_response(inv.0, inv.1, inv.2, inv.3, inv.4)).collect(),
            total_count: count
        })
    }

    pub async fn get_customized_inventories_by_product_id(&self, product_id: i32, pagination: &Pagination) -> Result<ServiceCollection<ResponseCustomizedInventory>, ApiError> {
        let converter = ConverterService::new();
        let fallbacks_service = FallbacksService::new();
        let conn = &mut self.repository.pool.get().or_else(
            |e| {
                tracing::error!("database connection broken: {}", e);
                Err(ApiError::DatabaseConnectionBroken)
            }
        )?;

        let count: i64 = inventories::table
            .filter(inventories::columns::product_id.eq(product_id))
            .select(count(inventories::columns::id))
            .first(conn)
            .or_else(|_e| Err(ApiError::NoRecord))?;
        
        let (page_offset, per_page) = fallbacks_service.fallback_pagination(&pagination);

        let all_compound_inventories_by_product_id_in_this_page_query = 
            inventories::table
                .inner_join(products::table)
                .filter(inventories::columns::product_id.eq(product_id))
                .limit(per_page)
                .offset(page_offset)
                .select(<(EntityInventory, EntityProduct)>::as_select());

        let all_compound_inventories_by_product_id_in_this_page = all_compound_inventories_by_product_id_in_this_page_query.get_results::<(EntityInventory, EntityProduct)>(conn).or_else(|_e| Err(ApiError::NoRecord))?;

        let receipt_ids_by_product_id = AsRef::<Vec<(EntityInventory, EntityProduct)>>::as_ref(&all_compound_inventories_by_product_id_in_this_page).into_iter().map(|x| x.0.receipt_id as i32).collect::<Vec<i32>>();

        let all_related_receipts_store_currency_by_product_id_in_this_page_query = 
            receipts::table
                .inner_join(stores::table)
                .inner_join(currencies::table)
                .filter(receipts::columns::id.eq_any(receipt_ids_by_product_id))
                .select(<(EntityReceipt, EntityStore, EntityCurrency)>::as_select());

        let all_related_receipts_store_currency_by_product_id_in_this_page = all_related_receipts_store_currency_by_product_id_in_this_page_query.get_results::<(EntityReceipt, EntityStore, EntityCurrency)>(conn).or_else(|_e| Err(ApiError::NoRecord))?;


        Ok(ServiceCollection {
            partial_collection: converter.convert_to_customized_inventories_response(all_compound_inventories_by_product_id_in_this_page, all_related_receipts_store_currency_by_product_id_in_this_page),
            total_count: count
        })
    }

    pub async fn get_customized_inventories_by_receipt_id(&self, receipt_id: i32, pagination: &Pagination) -> Result<ServiceCollection<ResponseCustomizedInventory>, ApiError> {
        let converter = ConverterService::new();
        let fallbacks_service = FallbacksService::new();
        let conn = &mut self.repository.pool.get().or_else(
            |e| {
                tracing::error!("database connection broken: {}", e);
                Err(ApiError::DatabaseConnectionBroken)
            }
        )?;

        let count: i64 = inventories::table
            .filter(inventories::columns::receipt_id.eq(receipt_id))
            .select(count(inventories::columns::id))
            .first(conn)
            .or_else(|_e| Err(ApiError::NoRecord))?;
        
        let (page_offset, per_page) = fallbacks_service.fallback_pagination(&pagination);

        let all_compound_inventories_by_receipt_id_in_this_page_query = 
            inventories::table
                .inner_join(products::table)
                .filter(inventories::columns::receipt_id.eq(receipt_id))
                .limit(per_page)
                .offset(page_offset)
                .select(<(EntityInventory, EntityProduct)>::as_select());

        let all_compound_inventories_by_receipt_id_in_this_page = all_compound_inventories_by_receipt_id_in_this_page_query.get_results::<(EntityInventory, EntityProduct)>(conn).or_else(|_e| Err(ApiError::NoRecord))?;

        let receipt_store_currency_query = 
            receipts::table
                .inner_join(stores::table)
                .inner_join(currencies::table)
                .filter(receipts::columns::id.eq(receipt_id))
                .select(<(EntityReceipt, EntityStore, EntityCurrency)>::as_select());

        let receipt_store_currency = receipt_store_currency_query.get_results::<(EntityReceipt, EntityStore, EntityCurrency)>(conn).or_else(|_e| Err(ApiError::NoRecord))?;


        Ok(ServiceCollection {
            partial_collection: converter.convert_to_customized_inventories_response(all_compound_inventories_by_receipt_id_in_this_page, receipt_store_currency),
            total_count: count
        })
    }

    pub async fn get_customized_inventories_by_store_id(&self, store_id: i32, pagination: &Pagination) -> Result<ServiceCollection<ResponseCustomizedInventory>, ApiError> {
        let converter = ConverterService::new();
        let fallbacks_service = FallbacksService::new();
        let conn = &mut self.repository.pool.get().or_else(
            |e| {
                tracing::error!("database connection broken: {}", e);
                Err(ApiError::DatabaseConnectionBroken)
            }
        )?;

        let (page_offset, per_page) = fallbacks_service.fallback_pagination(&pagination);

        let all_related_receipts_by_store_id_query = 
            receipts::table
                .inner_join(stores::table)
                .inner_join(currencies::table)
                .filter(receipts::columns::store_id.eq(store_id))
                .select(<(EntityReceipt, EntityStore, EntityCurrency)>::as_select());

        let receipts_store_currency = all_related_receipts_by_store_id_query.get_results::<(EntityReceipt, EntityStore, EntityCurrency)>(conn).or_else(|_e| Err(ApiError::NoRecord))?;

        let receipt_ids = AsRef::<Vec<(EntityReceipt, EntityStore, EntityCurrency)>>::as_ref(&receipts_store_currency).into_iter().map(|t| t.0.id).collect::<Vec<i32>>();

        let count: i64 = inventories::table
            .filter(inventories::columns::receipt_id.eq_any(&receipt_ids))
            .select(count(inventories::columns::id))
            .first(conn)
            .or_else(|_e| Err(ApiError::NoRecord))?;
        

        let all_compound_inventories_by_receipt_ids_in_this_page_query = 
            inventories::table
                .inner_join(products::table)
                .filter(inventories::columns::receipt_id.eq_any(&receipt_ids))
                .limit(per_page)
                .offset(page_offset)
                .select(<(EntityInventory, EntityProduct)>::as_select());

        let all_compound_inventories_by_receipt_ids_in_this_page = all_compound_inventories_by_receipt_ids_in_this_page_query.get_results::<(EntityInventory, EntityProduct)>(conn).or_else(|_e| Err(ApiError::NoRecord))?;

        Ok(ServiceCollection {
            partial_collection: converter.convert_to_customized_inventories_response(all_compound_inventories_by_receipt_ids_in_this_page, receipts_store_currency),
            total_count: count
        })
    }

    pub async fn get_customized_inventories_by_currency_id(&self, currency_id: i32, pagination: &Pagination) -> Result<ServiceCollection<ResponseCustomizedInventory>, ApiError> {
        let converter = ConverterService::new();
        let fallbacks_service = FallbacksService::new();
        let conn = &mut self.repository.pool.get().or_else(
            |e| {
                tracing::error!("database connection broken: {}", e);
                Err(ApiError::DatabaseConnectionBroken)
            }
        )?;

        let (page_offset, per_page) = fallbacks_service.fallback_pagination(&pagination);

        let all_related_receipts_by_currency_id_query = 
            receipts::table
                .inner_join(stores::table)
                .inner_join(currencies::table)
                .filter(receipts::columns::currency_id.eq(currency_id))
                .select(<(EntityReceipt, EntityStore, EntityCurrency)>::as_select());

        let receipts_store_currency = all_related_receipts_by_currency_id_query.get_results::<(EntityReceipt, EntityStore, EntityCurrency)>(conn).or_else(|_e| Err(ApiError::NoRecord))?;

        let receipt_ids = AsRef::<Vec<(EntityReceipt, EntityStore, EntityCurrency)>>::as_ref(&receipts_store_currency).into_iter().map(|t| t.0.id).collect::<Vec<i32>>();

        let count: i64 = inventories::table
            .filter(inventories::columns::receipt_id.eq_any(&receipt_ids))
            .select(count(inventories::columns::id))
            .first(conn)
            .or_else(|_e| Err(ApiError::NoRecord))?;
        

        let all_compound_inventories_by_currency_ids_in_this_page_query = 
            inventories::table
                .inner_join(products::table)
                .filter(inventories::columns::receipt_id.eq_any(&receipt_ids))
                .limit(per_page)
                .offset(page_offset)
                .select(<(EntityInventory, EntityProduct)>::as_select());

        let all_compound_inventories_by_currency_ids_in_this_page = all_compound_inventories_by_currency_ids_in_this_page_query.get_results::<(EntityInventory, EntityProduct)>(conn).or_else(|_e| Err(ApiError::NoRecord))?;

        Ok(ServiceCollection {
            partial_collection: converter.convert_to_customized_inventories_response(all_compound_inventories_by_currency_ids_in_this_page, receipts_store_currency),
            total_count: count
        })
    }
}