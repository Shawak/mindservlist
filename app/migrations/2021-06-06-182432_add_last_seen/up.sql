-- Your SQL goes here

alter table server
add column last_seen int8 not null default 0;
