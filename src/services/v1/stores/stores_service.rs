use diesel::{
    dsl::{count, exists, select}, insert_into, ExpressionMethods, QueryDsl, RunQueryDsl, SaveChangesDsl, SelectableHelper
};

use crate::{
    models::v1::{
        collections::service_collection::ServiceCollection, entities::entity_store::{EntityStore, NewEntityStore, UpdateEntityStore}, errors::api_error::ApiError, forms::patch_payload::PatchStorePayload, parameters::pagination::Pagination, responses::response_store::ResponseStore
    }, 
    repository::DbRepository, 
    schema::stores, 
    services::v1::{converters::converters_service::ConverterService, fallbacks::fallbacks_service::FallbacksService}
};

pub struct StoreService<'a> {
    repository: &'a DbRepository
}

impl<'a> StoreService<'a> {
    pub fn new(repository: &'a DbRepository) -> Self {
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
    
    pub async fn is_store_existed_by_id(&self, id: i32) -> Result<bool, ApiError> {
        let conn = &mut self.repository.pool.get().or_else(|e| {
            tracing::error!("database connection broken: {}", e);
            Err(ApiError::DatabaseConnectionBroken)
        })?;

        select(exists(stores::table.filter(stores::id.eq(id)))).get_result::<bool>(conn).or_else(|_e| {
            Err(ApiError::CurrencyIdNotExisted)  
        })
    }

    pub async fn is_store_existed_by_name_and_branch(&self, name: &String, branch: Option<&String>) -> Result<bool, ApiError> {
        let conn = &mut self.repository.pool.get().or_else(|e| {
            tracing::error!("database connection broken: {}", e);
            Err(ApiError::DatabaseConnectionBroken)
        })?;

        // Compare the below with products_service::is_product_existed_by_name method, we keep the below to make a contrast
        match branch {
            Some(store_branch) => {
                select(exists(stores::table.filter(stores::name.eq(name)).filter(stores::branch.eq(store_branch)))).get_result::<bool>(conn).or_else(|_e| {
                    Err(ApiError::StoreNameDuplicated)
                })
            },
            None => {
                select(exists(stores::table.filter(stores::name.eq(name)))).get_result::<bool>(conn).or_else(|_e| {
                    Err(ApiError::StoreNameDuplicated)
                })
            }
        }
    }

    pub async fn new_store(&self, store: &NewEntityStore) -> Result<i32, ApiError> {
        let conn = &mut self.repository.pool.get().or_else(|e| {
            tracing::error!("database connection broken: {}", e);
            Err(ApiError::DatabaseConnectionBroken)
        })?;

        let entity_store = insert_into(stores::table)
            .values(store)
            .get_result::<EntityStore>(conn).or_else(|e| {
                tracing::error!("insert store entity failed: {}", e);
                Err(ApiError::InsertStoreFailed)
        })?;

        Ok(entity_store.id)
    }

    pub async fn patch_store(&self, id: i32, store: &PatchStorePayload) -> Result<(), ApiError> {
        let conn = &mut self.repository.pool.get().or_else(|e| {
            tracing::error!("database connection broken: {}", e);
            Err(ApiError::DatabaseConnectionBroken)
        })?;

        let mut updated_store = UpdateEntityStore {
            id,
            name: None,
            alias: None,
            branch: None,
            address: None
        };

        if store.name.is_some() {
            let updated_name = store.name.as_ref().expect("name should not be none");
            updated_store.name = Some(updated_name);
        }

        if store.alias.is_some() {
            let updated_option_alias = store.alias.as_ref().expect("option alias should not be none");
            if updated_option_alias.is_some() {
                let updated_alias = updated_option_alias.as_ref().expect("alias should not be none");
                updated_store.alias = Some(Some(&updated_alias));
            }
            else {
                updated_store.alias = Some(None);
            }
        }

        if store.branch.is_some() {
            let updated_option_branch = store.branch.as_ref().expect("option branch should not be none");
            if updated_option_branch.is_some() {
                let updated_branch = updated_option_branch.as_ref().expect("branch should not be none");
                updated_store.branch = Some(Some(&updated_branch));
            }
            else {
                updated_store.branch = Some(None);
            }
        }

        if store.address.is_some() {
            let updated_option_address = store.address.as_ref().expect("option address should not be none");
            if updated_option_address.is_some() {
                let updated_address = updated_option_address.as_ref().expect("address should not be none");
                updated_store.address = Some(Some(&updated_address));
            }
            else {
                updated_store.address = Some(None);
            }
        }

        updated_store.save_changes::<EntityStore>(conn).or_else(|e| {
            tracing::error!("update store entity failed: {}", e);
            Err(ApiError::UpdateStoreFailed)
        })?;
        
        Ok(())
    }
}
