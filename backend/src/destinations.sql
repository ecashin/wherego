drop table if exists wherego_destinations;
create table if not exists wherego_destinations ( name varchar (256) not null, description varchar (1024) not null, id bigserial primary key );
insert into wherego_destinations (name, description) values ('Whitby', 'Sea port');
insert into wherego_destinations (name, description) values ('Kailua Kona', 'Beach town');
