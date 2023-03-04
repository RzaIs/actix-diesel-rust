-- Your SQL goes here

create table app_user (
    id serial primary key,
    email varchar(64) not null,
    username varchar(64) not null,
    secret text not null
);

alter table app_user
    add constraint app_user_email_key unique(email);

alter table app_user
    add constraint app_user_username_key unique(username);
