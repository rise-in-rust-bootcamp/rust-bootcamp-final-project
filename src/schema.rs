// @generated automatically by Diesel CLI.

diesel::table! {
    products (id) {
        id -> Integer,
        name -> Text,
        description -> Nullable<Text>,
        price -> Double,
        quantity -> Integer,
    }
}

diesel::table! {
    purchase_orders (id) {
        id -> Integer,
        product_id -> Integer,
        price -> Double,
        quantity -> Integer,
    }
}

diesel::joinable!(purchase_orders -> products (product_id));

diesel::allow_tables_to_appear_in_same_query!(
    products,
    purchase_orders,
);
