-- This file should undo anything in `up.sql`
alter table app_user
    drop column created;
