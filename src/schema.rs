// @generated automatically by Diesel CLI.

diesel::table! {
    currencies (id) {
        id -> Int4,
        name -> Text,
    }
}

diesel::table! {
    inventories (id) {
        id -> Int4,
        price -> Numeric,
        quantity -> Int4,
        product_id -> Int4,
        receipt_id -> Int4,
    }
}

diesel::table! {
    products (id) {
        id -> Int4,
        name -> Text,
        alias -> Nullable<Text>,
        brand -> Nullable<Text>,
        specification_amount -> Nullable<Int4>,
        specification_unit -> Nullable<Text>,
        specification_others -> Nullable<Text>,
    }
}

diesel::table! {
    receipts (id) {
        id -> Int4,
        transaction_date -> Timestamp,
        is_inventory_taxed -> Bool,
        currency_id -> Int4,
        store_id -> Int4,
        transaction_id -> Nullable<Uuid>,
    }
}

diesel::table! {
    stores (id) {
        id -> Int4,
        name -> Text,
        alias -> Nullable<Text>,
        branch -> Nullable<Text>,
        address -> Nullable<Text>,
    }
}

diesel::joinable!(inventories -> products (product_id));
diesel::joinable!(inventories -> receipts (receipt_id));
diesel::joinable!(receipts -> currencies (currency_id));
diesel::joinable!(receipts -> stores (store_id));

diesel::allow_tables_to_appear_in_same_query!(
    currencies,
    inventories,
    products,
    receipts,
    stores,
);
