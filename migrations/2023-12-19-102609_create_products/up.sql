CREATE TABLE products (
  id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  name VARCHAR NOT NULL,
  description TEXT ,
  price DECIMAL(10,2) NOT NULL,
  quantity INTEGER NOT NULL DEFAULT 0
)