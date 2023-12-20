use diesel::prelude::*;
use diesel::SqliteConnection;
use tabled::{settings::Panel, Table};

use crate::schema::purchase_orders;
use crate::utils::*;
use crate::{
    models::{self, PurchaseOrder},
    schema,
};

pub fn prompt_add_purchase_order(connection: &mut SqliteConnection) {
    println!("Please enter the following:\n");
    let product_id: i32 = prompt_number("Product Id:", None);
    let price = prompt_amount("Price($):", None);
    let quantity = prompt_number("Quantity:", None);

    let new_purchase_order = models::NewPurchaseOrder {
        product_id,
        price,
        quantity,
    };

    diesel::insert_into(schema::purchase_orders::table)
        .values(&new_purchase_order)
        .returning(PurchaseOrder::as_returning())
        .get_result(connection)
        .expect("Error saving new purchase order");

    let mut product = schema::products::table
        .find(product_id)
        .select(models::Product::as_select())
        .first(connection)
        .expect("Error loading product");

    // update latest price and quantity
    product.price = price;
    product.quantity += quantity;
    diesel::update(&product)
        .set(&product)
        .execute(connection)
        .expect("Error updating product");
}

pub fn list_purchase_orders(connection: &mut SqliteConnection) {
    let results = purchase_orders::table
        .select(PurchaseOrder::as_select())
        .load(connection)
        .expect("Error loading purchase orders");

    let mut table = Table::new(results);
    table.with(Panel::header("Purchase Orders"));

    println!("{}", table);
}
