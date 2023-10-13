drop table if exists wherego_scores;
create table if not exists wherego_scores ( username varchar (256) not null, score bigint, dest_id bigint );
insert into wherego_scores (username, score, dest_id) values ('Ed', 100, 1);
