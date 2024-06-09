alter  table customers rename to customers_old;
create table customers (
  id integer not null primary key autoincrement,
  name text  not null,
  email text,
  created_at integer default(unixepoch()),
  updated_at integer default(unixepoch()) not null
);

insert into customers (id, name, email, created_at, updated_at) 
select id, name, email, created_at, updated_at from customers_old;

alter  table bills rename to bills_old;
create table bills (
  id integer primary key autoincrement,
  customer_id integer not null,
  /*: json of '{ "inventory_name_text": quantity_purchased }' */ 
  line_items text not null,
  total integer not null,
  created_at integer default(unixepoch()),
  updated_at integer default(unixepoch()) not null
); 

/* At this point, if this were a pre-established production
   schema with existing customers and receipt information
   it would be imperative to remove `drop table __customers_old`
   from above and extract the primary keys saved under `.bills`
   and have that guide a batch processing procedure which 
   creates the new rows in the bills table.

   Since this is a demo application, I'm not taking the time out 
   to grok this since there are no receipts to even migrate.
*/
drop table bills_old;
drop table customers_old;
