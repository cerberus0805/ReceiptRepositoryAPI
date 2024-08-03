use bigdecimal::{BigDecimal, FromPrimitive};
use diesel::{
    dsl::count, insert_into, update, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper
};

use crate::{
    models::v1::{
        collections::service_collection::ServiceCollection, entities::{
            entity_currency::{EntityCurrency, NewEntityCurrency}, entity_inventory::{EntityInventory, NewEntityInventory}, entity_product::{EntityProduct, NewEntityProduct}, entity_receipt::{EntityReceipt, NewEntityReceipt}, entity_store::{EntityStore, NewEntityStore}
        }, errors::api_error::ApiError, forms::{create_payload::{CreateCurrencyInReceiptPayload, CreateProductInReceiptPayload, CreateReceiptPayload, CreateStoreInReceiptPayload}, patch_payload::PatchReceiptPayload}, parameters::pagination::Pagination, responses::response_receipt::{ResponseCreateReceipt, ResponseReceipt}
    }, 
    repository::DbRepository, 
    schema::{
        currencies, inventories, products, receipts, stores
    }, services::v1::{converters::converters_service::ConverterService, currencies::currencies_service::CurrencyService, fallbacks::fallbacks_service::FallbacksService, inventories::inventories_service::InventroyService, products::products_service::ProductService, stores::stores_service::StoreService, validators::formdata_validators_service::{FormDataValidatorService, FormRelationshipModelStatus}}
};

#[derive(Clone)]
pub struct ReceiptService<'a> {
    repository: &'a DbRepository
}

impl<'a> ReceiptService<'a> {
    pub fn new(repository: &'a DbRepository) -> Self {
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
            }
        )?;

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
            }
        )?;

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

    async fn new_receipt(&self, receipt: &NewEntityReceipt) -> Result<i32, ApiError> {
        let conn = &mut self.repository.pool.get().or_else(
            |e| {
                tracing::error!("database connection broken: {}", e);
                Err(ApiError::DatabaseConnectionBroken)
            }
        )?;

        let entity_receipt = insert_into(receipts::table)
            .values(receipt)
            .get_result::<EntityReceipt>(conn).or_else(|e| {
                tracing::error!("insert receipt entity failed: {}", e);
                Err(ApiError::InsertReceiptFailed)
        })?;

        Ok(entity_receipt.id)
    }

    pub async fn create_receipt(&self, form_receipt: &CreateReceiptPayload) -> Result<ResponseCreateReceipt, ApiError> {
        let currency_status = self.validate_currency(&form_receipt.currency).await.or_else(|e| {
            tracing::error!("validate_currency failed");
            return Err(e);
        })?;

        let store_status = self.validate_store(&form_receipt.store).await.or_else(|e| {
            tracing::error!("validate_store failed");
            return Err(e);
        })?;
        
        let mut inventories_metadata = vec![];
        for inventory in &form_receipt.inventories {
            let product_status = self.validate_product(&inventory.product).await.or_else(|e| {
                tracing::error!("validate_product failed");
                return Err(e);
            })?;
            inventories_metadata.push((product_status, inventory));
        }

        let currency_ref_id;
        if currency_status == FormRelationshipModelStatus::Id {
            currency_ref_id = form_receipt.currency.id.expect("currency id should not be none after validation");
        }
        else  {
            // None status will not reach here
            let new_currency = NewEntityCurrency {
                name: form_receipt.currency.name.clone().expect("currency name should not ")
            };
            
            let currency_service = CurrencyService::new(&self.repository);
            currency_ref_id = currency_service.new_currency(&new_currency).await?;
        }

        let store_ref_id;
        if store_status == FormRelationshipModelStatus::Id {
            store_ref_id = form_receipt.store.id.expect("store id should not be none after validation")
        }
        else {
            // None status will not reach here
            let new_store = NewEntityStore {
                name: form_receipt.store.name.clone().expect("store name should not be none after validation"),
                alias: form_receipt.store.alias.clone(),
                branch: form_receipt.store.branch.clone(),
                address: form_receipt.store.address.clone()
            };

            let store_service = StoreService::new(&self.repository);
            store_ref_id = store_service.new_store(&new_store).await?;
        }

        let new_receipt = NewEntityReceipt {
            transaction_date: form_receipt.transaction_date,
            is_inventory_taxed: form_receipt.is_inventory_taxed,
            currency_id: currency_ref_id,
            store_id: store_ref_id
        };

        let receipt_ref_id = self.new_receipt(&new_receipt).await?;
        
        for pair in inventories_metadata {
            let product_ref_id;
            if pair.0 == FormRelationshipModelStatus::Id {
                product_ref_id = pair.1.product.id.expect("product id should not be none")
            }
            else {
                // None status will not reach here
                let new_product = NewEntityProduct {
                    name: pair.1.product.name.clone().expect("product name should not be none after validation"),
                    alias: pair.1.product.alias.clone(),
                    specification_amount: pair.1.product.specification_amount.clone(),
                    specification_unit: pair.1.product.specification_unit.clone(),
                    specification_others: pair.1.product.specification_others.clone(),
                    brand: pair.1.product.brand.clone()
                };

                let product_service = ProductService::new(&self.repository);
                product_ref_id = product_service.new_product(&new_product).await?;
            }

            let new_inventory = NewEntityInventory {
                price: BigDecimal::from_f64(pair.1.price.clone()).expect("parse price should be successful after validation"),
                quantity: pair.1.quantity.clone(),
                product_id: product_ref_id,
                receipt_id: receipt_ref_id
            };

            let inventory_service = InventroyService::new(&self.repository);
            let _inventory_id = inventory_service.new_inventory(&new_inventory).await?;
        }
        
        // let mut insert_clauses = vec![];
        Ok(ResponseCreateReceipt {
            id: receipt_ref_id
        })
    }

    async fn validate_currency(&self, currency: &CreateCurrencyInReceiptPayload) -> Result<FormRelationshipModelStatus, ApiError> {
        let formdata_validators_service = FormDataValidatorService::new();
        let currency_status = formdata_validators_service.validate_relationship_model(currency);
        if currency_status == FormRelationshipModelStatus::None {
            tracing::error!("invalid currency");
            return Err(ApiError::CurrencyInvalid);
        }
        
        let currency_service = CurrencyService::new(&self.repository);
        if currency_status == FormRelationshipModelStatus::Id {
            let currency_id = currency.id.expect("currency id should not be none").clone();
            let is_existed = currency_service.is_currency_existed_by_id(currency_id).await?;
            if !is_existed {
                return Err(ApiError::CurrencyIdNotExisted);
            }
        }
        else if currency_status == FormRelationshipModelStatus::ItemName {
            let currency_name = currency.name.as_ref().expect("currency name should not be none");
            let is_existed = currency_service.is_currency_existed_by_name(currency_name).await?;
            if is_existed {
                return Err(ApiError::CurrencyNameDuplicated);
            }
        }

        Ok(currency_status)
    }

    async fn validate_store(&self, store: &CreateStoreInReceiptPayload) -> Result<FormRelationshipModelStatus, ApiError> {
        let formdata_validators_service = FormDataValidatorService::new();
        let store_status = formdata_validators_service.validate_relationship_model(store);
        if store_status == FormRelationshipModelStatus::None {
            tracing::error!("invalid store");
            return Err(ApiError::StoreInvalid);
        }
        let store_id;
        let store_name;
        let store_branch;
        let store_service = StoreService::new(&self.repository);
        if store_status == FormRelationshipModelStatus::Id {
            store_id = store.id.expect("store id should not be none");
            let is_existed = store_service.is_store_existed_by_id(store_id).await?;
            if !is_existed {
                return Err(ApiError::StoreInvalid);
            }
        }
        else if store_status == FormRelationshipModelStatus::ItemName {
            store_name = store.name.as_ref().expect("store name should not be none");
            store_branch = store.branch.as_ref();
            let is_existed = store_service.is_store_existed_by_name_and_branch(store_name, store_branch).await?;
            if is_existed {
                return Err(ApiError::StoreNameDuplicated);
            }
        }

        Ok(store_status)
    }

    async fn validate_product(&self, product: &CreateProductInReceiptPayload) -> Result<FormRelationshipModelStatus, ApiError> {
        let formdata_validators_service = FormDataValidatorService::new();
        let product_status = formdata_validators_service.validate_relationship_model(product);
        if product_status == FormRelationshipModelStatus::None {
            return Err(ApiError::StoreInvalid);
        }

        let product_service = ProductService::new(&self.repository);
        if product_status == FormRelationshipModelStatus::Id {
            let product_id = product.id.expect("product id should not be none");
            let is_existed =  product_service.is_product_existed_by_id(product_id).await?;
            if !is_existed {
                return Err(ApiError::ProductIdNotExisted);
            }
        }
        else {
            let product_name = product.name.as_ref().expect("product name should not be none");
            let product_brand = product.brand.as_ref();
            let product_spec_amount = product.specification_amount.as_ref();
            let product_spec_unit = product.specification_unit.as_ref();
            let product_spec_others = product.specification_others.as_ref();
            let is_existed = product_service.is_product_existed_by_name(product_name, product_brand, product_spec_amount, product_spec_unit, product_spec_others).await?;
            if is_existed {
                return Err(ApiError::CurrencyNameDuplicated);
            }
        }

        Ok(product_status)
    }

    pub async fn patch_receipt(&self, id: i32, receipt: &PatchReceiptPayload) -> Result<(), ApiError> {
        let conn = &mut self.repository.pool.get().or_else(|e| {
            tracing::error!("database connection broken: {}", e);
            Err(ApiError::DatabaseConnectionBroken)
        })?;

        if receipt.transaction_date.is_some() && receipt.is_inventory_taxed.is_some() {
            update(receipts::table.filter(receipts::id.eq(id))).set((receipts::transaction_date.eq(receipt.transaction_date.expect("transaction_date should not be none")), receipts::is_inventory_taxed.eq(receipt.is_inventory_taxed.expect("is_inventory_taxed should nt be none")))).execute(conn).or_else(|e| {
                tracing::error!("update receipt entity failed: {}", e);
                Err(ApiError::UpdateReceiptFailed)
            })?;
        }
        else if receipt.transaction_date.is_some() {
            update(receipts::table.filter(receipts::id.eq(id))).set(receipts::transaction_date.eq(receipt.transaction_date.expect("transaction_date should not be none"))).execute(conn).or_else(|e| {
                tracing::error!("update receipt entity failed: {}", e);
                Err(ApiError::UpdateReceiptFailed)
            })?;
        }
        else if receipt.is_inventory_taxed.is_some() {
            update(receipts::table.filter(receipts::id.eq(id))).set(receipts::is_inventory_taxed.eq(receipt.is_inventory_taxed.expect("is_inventory_taxed should not be none"))).execute(conn).or_else(|e| {
                tracing::error!("update receipt entity failed: {}", e);
                Err(ApiError::UpdateReceiptFailed)
            })?;
        }

        Ok(())
    }
}