use diesel::prelude::*;
use tabled::Tabled;

#[derive(Queryable, Selectable, AsChangeset, Identifiable, Tabled, Debug)]
#[diesel(table_name = crate::schema::products)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Product {
    pub id: i32,
    pub name: String,
    #[tabled(display_with = "display_option")]
    pub description: Option<String>,
    #[tabled(display_with = "display_price")]
    pub price: f64,
    pub quantity: i32,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::products)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewProduct {
    pub name: String,
    pub description: Option<String>,
    pub price: f64,
    pub quantity: i32,
}

#[derive(Queryable, Selectable, AsChangeset, Identifiable, Tabled, Debug)]
#[diesel(table_name = crate::schema::purchase_orders)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(belongs_to(Product))]
pub struct PurchaseOrder {
    pub id: i32,
    pub product_id: i32,
    #[tabled(display_with = "display_price")]
    pub price: f64,
    pub quantity: i32,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::purchase_orders)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(belongs_to(Product))]
pub struct NewPurchaseOrder {
    pub product_id: i32,
    pub price: f64,
    pub quantity: i32,
}

#[derive(Queryable, Selectable, AsChangeset, Identifiable, Tabled, Debug)]
#[diesel(table_name = crate::schema::sale_transactions)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(belongs_to(Product))]
pub struct SaleTransaction {
    pub id: i32,
    pub product_id: i32,
    #[tabled(display_with = "display_price")]
    pub product_price: f64,
    #[tabled(display_with = "display_price")]
    pub sold_price: f64,
    pub quantity: i32,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::sale_transactions)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(belongs_to(Product))]
pub struct NewSaleTransaction {
    pub product_id: i32,
    pub product_price: f64,
    pub sold_price: f64,
    pub quantity: i32,
}

fn display_option(o: &Option<String>) -> String {
    match o {
        Some(s) => s.to_string(),
        None => "".to_string(),
    }
}

fn display_price(n: &f64) -> String {
    format!("${:.2}", n)
}
