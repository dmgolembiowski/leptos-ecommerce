create table if not exists inventory (
  id int primary key generated always as identity,
  name varchar(80) not null,
  asset varchar(80) not null,
  cost money not null,
  quantity_available int not null default 18446744073709551615
);
create table if not exists bill (
  id int primary key generated always as identity,
  line_items int[][],
  total money not null
);
create table if not exists customer (
  id int primary key generated always as identity,
  name text not null,
  email text,
  bills int[]
);
create table if not exists cookie (
  id int primary key generated always as identity,
  customer_id int references customer(id),
  inventory_id int references inventory(id),
  key text not null,
  value text not null
);
