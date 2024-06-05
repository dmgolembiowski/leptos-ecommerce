create table if not exists item (
  id integer primary key not null auto_increment,
  inventory_id integer references inventory(id),
  name text not null,
  asset text not null,
);

create table if not exists inventory (
  id integer primary key not null auto_increment,
  cost money not null,
  quantity_available integer not null default 18446744073709551615,
  item_id  references item(id)
);

create table if not exists bill (
  id integer primary key not null auto_increment,
  customer_id integer references customer(id),
  line_items integer[][],
  total money not null
);

create table if not exists customer (
  id integer primary key not null auto_increment,
  name text not null,
  email text,
  bills integer[] references bill,
  cookies integer[] references cookie
);

create table if not exists cookie (
  id integer primary key not null auto_increment,
  bill_id integer references bill(id),
  customer_id integer references customer(id),
  item_id integer references item(id),
  key text not null,
  value text not null
);

