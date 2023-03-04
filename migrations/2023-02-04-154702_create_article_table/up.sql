-- Your SQL goes here

create table article (
    id serial primary key,
    title varchar(64) not null,
    content varchar(128) not null
);