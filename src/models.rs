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

fn display_option(o: &Option<String>) -> String {
    match o {
        Some(s) => s.to_string(),
        None => "".to_string(),
    }
}

fn display_price(n: &f64) -> String {
    format!("${:.2}", n)
}
