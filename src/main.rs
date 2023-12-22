use clap::{Parser, Subcommand};
use diesel::SqliteConnection;
use utils::get_db_connection;

pub mod models;
pub mod schema;
pub mod utils;

pub mod inventory;
pub mod purchase_orders;
pub mod sale_transactions;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum CommandCRADAction {
    Add,
    Edit,
    Remove,
    List,
}

#[derive(Subcommand, Debug)]
enum CommandAddListAction {
    Add,
    List,
}

#[derive(Subcommand, Debug)]
enum Command {
    Inventory {
        #[clap(subcommand)]
        action: CommandCRADAction,
    },
    SaleTransactions {
        #[clap(subcommand)]
        action: CommandAddListAction,
    },
    PurchaseOrders {
        #[clap(subcommand)]
        action: CommandAddListAction,
    },
}

fn main() {
    let args = Args::parse();
    let mut conn: SqliteConnection = get_db_connection();

    match args.command {
        Command::Inventory { action } => match action {
            CommandCRADAction::Add => inventory::prompt_add_product(&mut conn),
            CommandCRADAction::List => inventory::list_products(&mut conn),
            CommandCRADAction::Edit => inventory::prompt_edit_product(&mut conn),
            CommandCRADAction::Remove => inventory::prompt_remove_product(&mut conn),
        },
        Command::PurchaseOrders { action } => match action {
            CommandAddListAction::Add => purchase_orders::prompt_add_purchase_order(&mut conn),
            CommandAddListAction::List => purchase_orders::list_purchase_orders(&mut conn),
        },
        Command::SaleTransactions { action } => match action {
            CommandAddListAction::Add => sale_transactions::prompt_add_sale_transaction(&mut conn),
            CommandAddListAction::List => sale_transactions::list_sale_transactions(&mut conn),
        },
    }
}
