create table if not exists inventory (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  name TEXT NOT NULL,
  asset TEXT NOT NULL,
  cost INTEGER NOT NULL,
  quantity_available INTEGER NOT NULL default 0,
  created_at INTEGER DEFAULT(unixepoch()),
  updated_at INTEGER DEFAULT(unixepoch()) NOT NULL
);
create table if not exists bills (
  id INTEGER primary key AUTOINCREMENT,
  line_items BLOB,
  total INTEGER NOT NULL,
  created_at INTEGER DEFAULT(unixepoch()),
  updated_at INTEGER DEFAULT(unixepoch()) NOT NULL
);
create table if not exists customers (
  id INTEGER primary key AUTOINCREMENT,
  name TEXT NOT NULL,
  email TEXT,
  bills BLOB,
  created_at INTEGER DEFAULT(unixepoch()),
  updated_at INTEGER DEFAULT(unixepoch()) NOT NULL
);
create table if not exists cookies (
  id INTEGER primary key AUTOINCREMENT,
  customer_id INTEGER references customer(id),
  inventory_id INTEGER references inventory(id),
  key TEXT NOT NULL,
  value TEXT NOT NULL,
  created_at INTEGER DEFAULT(unixepoch()),
  updated_at INTEGER DEFAULT(unixepoch()) NOT NULL
);

  CREATE TRIGGER IF NOT EXISTS Inv_Updated
  AFTER UPDATE ON inventory
  FOR EACH ROW
  BEGIN
      UPDATE inventory SET updated_at = unixepoch() WHERE id = OLD.id;
  END;

  CREATE TRIGGER IF NOT EXISTS Bills_Updated
  AFTER UPDATE ON bills
  FOR EACH ROW
  BEGIN
      UPDATE bills SET updated_at = unixepoch() WHERE id = OLD.id;
  END;

  CREATE TRIGGER IF NOT EXISTS Customers_Updated
  AFTER UPDATE ON customers
  FOR EACH ROW
  BEGIN
      UPDATE customers SET updated_at = unixepoch() WHERE id = OLD.id;
  END;


  CREATE TRIGGER IF NOT EXISTS Cookies_Updated
  AFTER UPDATE ON cookies
  FOR EACH ROW
  BEGIN
      UPDATE cookies SET updated_at = unixepoch() WHERE id = OLD.id;
  END;
