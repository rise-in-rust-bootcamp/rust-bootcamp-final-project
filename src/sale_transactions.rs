use diesel::prelude::*;
use diesel::SqliteConnection;
use tabled::{settings::Panel, Table};

use crate::schema::sale_transactions;
use crate::utils::*;
use crate::{
    models::{self, SaleTransaction},
    schema,
};

pub fn prompt_add_sale_transaction(connection: &mut SqliteConnection) {
    println!("Please enter the following:\n");
    let product_id: i32 = prompt_number("Product Id:", None);
    let sold_price = prompt_amount("Sold Price($):", None);
    let quantity = prompt_number("Quantity:", None);

    let mut product = schema::products::table
        .find(product_id)
        .select(models::Product::as_select())
        .first(connection)
        .expect("Error loading product");

    if product.quantity < quantity {
        eprintln!("Insufficient inventory");
        return;
    }

    connection
        .transaction(|connection| {
            let new_sale_transaction = models::NewSaleTransaction {
                product_id,
                product_price: product.price,
                sold_price,
                quantity,
            };

            diesel::insert_into(schema::sale_transactions::table)
                .values(&new_sale_transaction)
                .returning(SaleTransaction::as_returning())
                .get_result(connection)
                .expect("Error saving new sale transaction");

            // update product's inventory
            product.quantity -= quantity;
            diesel::update(&product)
                .set(&product)
                .execute(connection)
                .expect("Error updating product");

            println!(
                "Successfully added sale transaction. Total cost: ${:.2}",
                sold_price * quantity as f64
            );

            diesel::result::QueryResult::Ok(())
        })
        .expect("Error updating product inventory");
}

pub fn list_sale_transactions(connection: &mut SqliteConnection) {
    let results = sale_transactions::table
        .select(SaleTransaction::as_select())
        .load(connection)
        .expect("Error loading sale transactions");

    let mut table = Table::new(results);
    table.with(Panel::header("Sale Transactions"));

    println!("{}", table);
}
