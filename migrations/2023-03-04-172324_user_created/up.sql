-- Your SQL goes here
alter table app_user
    add column created timestamp(3) without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL;
