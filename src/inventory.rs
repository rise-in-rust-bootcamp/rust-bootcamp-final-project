use diesel::prelude::*;
use diesel::SqliteConnection;
use tabled::{settings::Panel, Table};

use crate::schema::products;
use crate::utils::*;
use crate::{
    models::{self, Product},
    schema,
};

pub fn prompt_add_product(connection: &mut SqliteConnection) {
    println!("Please enter the following:\n");

    let name = prompt_text("Name:", true, None);
    let description = prompt_text("Description:", false, None);
    let price = prompt_amount("Price($):", None);
    let quantity = prompt_number("Quantity:", None);

    let new_product = models::NewProduct {
        name,
        description: Some(description),
        price,
        quantity,
    };

    diesel::insert_into(schema::products::table)
        .values(&new_product)
        .returning(Product::as_returning())
        .get_result(connection)
        .expect("Error saving new product");
}

pub fn prompt_edit_product(connection: &mut SqliteConnection) {
    println!("Please enter the product id to edit:");
    let product_id: i32 = prompt_number("Product Id:", None);

    let product = products::table
        .find(product_id)
        .select(Product::as_select())
        .first(connection)
        .optional();

    match product {
        Ok(Some(mut product)) => {
            let name = prompt_text("Name:", true, Some(&product.name));
            let description = prompt_text(
                "Description:",
                false,
                Some(&product.description.unwrap_or_default()),
            );
            let price = prompt_amount("Price($):", Some(product.price));
            let quantity = prompt_number("Quantity:", Some(product.quantity));

            product.name = name;
            product.description = Some(description);
            product.price = price;
            product.quantity = quantity;

            diesel::update(&product)
                .set(&product)
                .execute(connection)
                .expect("Error updating product");
        }
        Ok(None) => println!("Unable to find product {}", product_id),
        Err(_) => println!("An error occured while fetching product {}", product_id),
    }
}

pub fn prompt_remove_product(connection: &mut SqliteConnection) {
    println!("Please enter the product id to remove:");
    let product_id: i32 = prompt_number("Product Id:", None);

    let product = products::table
        .find(product_id)
        .select(Product::as_select())
        .first(connection)
        .optional();

    match product {
        Ok(Some(product)) => {
            diesel::delete(&product)
                .execute(connection)
                .expect("Error deleting posts");
        }
        Ok(None) => println!("Unable to find product {}", product_id),
        Err(_) => println!("An error occured while fetching product {}", product_id),
    }
}

pub fn list_products(connection: &mut SqliteConnection) {
    let results = products::table
        .select(Product::as_select())
        .load(connection)
        .expect("Error loading posts");

    let mut table = Table::new(results);
    table.with(Panel::header("Products"));

    println!("{}", table);
}
