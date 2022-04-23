-- Add migration script here

 CREATE TABLE IF NOT EXISTS devices (
     id INTEGER PRIMARY KEY AUTOINCREMENT,
     name VARCHAR(45) NOT NULL,
     brand VARCHAR(45) NOT NULL,
     serial VARCHAR(45) NOT NULL,
     power INTEGER NOT NULL
   );

--    insert into devices values (1,'abc', 'efg', 'hij', 1);