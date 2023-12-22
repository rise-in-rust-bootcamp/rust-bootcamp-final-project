# Rusty Store Inventory Management System

The task is to create an inventory management system for a small retail store using Rust. The system should be able to manage the store's inventory, sales, and purchases.

The system should have the following features:

**Inventory Management**: The system should allow store managers to add, edit, and delete products from the inventory. Each product should have a name, description, price, and quantity.

**Sales Management**: The system should allow store managers to record sales transactions, including the product sold, the quantity sold, and the sale price. The system should also calculate and display the total sales and profit made from each transaction.

**Purchase Management**: The system should allow store managers to record purchase transactions, including the product purchased, the quantity purchased, and the purchase price. The system should also calculate and display the total cost of each purchase.

**Reporting**: The system should be able to generate reports that show the store's inventory, sales, and purchase history. Reports should be generated in a user-friendly text format that's easy to read, such as a well-structured table or list.

**Error Handling**: The system should have robust error handling capabilities, including handling of invalid inputs, out-of-stock items, and other possible errors.

**Security**: The system should have basic security measures in place, such as authentication for store managers to prevent unauthorized access to the inventory, sales, and purchase data.

**User Interface**: The system should have a clear and intuitive text-based user interface that allows store managers to easily navigate and perform tasks.

![Program Preview](./program_preview.gif)

## How to run

Copy and edit the database endpoint

```sh
$ cp .example .example
```

Run the migration

```
$ disesel setup
```

### Product Management

Add a new product

```sh
$ cargo run inventory add
```

List all products

```sh
$ cargo run inventory list
```

Edit a product

```sh
$ cargo run inventory edit
```

Delete a product

```sh
$ cargo run inventory delete
```

### Purchase Orders

Add a new purchase order

```sh
$ cargo run purchase-orders add
```

List all purchase orders

```sh
$ cargo run purchase-orders list
```

### Sale Transactions

Add a new sale transaction

```sh
$ cargo run sale-transactions add
```

List all sale transactions

```sh
$ cargo run sale-transactions list
```

## To-Do

- [ ] User Authentication
