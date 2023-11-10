use std::env;

use clap::{Parser, Subcommand};
use diesel::prelude::*;
use dotenvy::dotenv;

pub mod inventory;
pub mod models;
pub mod schema;
pub mod utils;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Inventory {
        #[clap(subcommand)]
        action: CommandAction,
    },
    Sales {
        #[clap(subcommand)]
        action: CommandAction,
    },
    PurchaseOrders {
        #[clap(subcommand)]
        action: CommandAction,
    },
}

#[derive(Subcommand, Debug)]
enum CommandAction {
    Add,
    Edit,
    Remove,
    List,
}

fn main() {
    let args = Args::parse();

    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let conn: &mut SqliteConnection = &mut SqliteConnection::establish(&database_url).unwrap();

    match args.command {
        Command::Inventory { action } => match action {
            CommandAction::Add => inventory::prompt_add_product(conn),
            CommandAction::List => inventory::list_products(conn),
            CommandAction::Edit => inventory::prompt_edit_product(conn),
            CommandAction::Remove => inventory::prompt_remove_product(conn),
        },
        _ => {
            println!("other")
        }
    }
}
