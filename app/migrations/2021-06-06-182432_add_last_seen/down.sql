-- This file should undo anything in `up.sql`

alter table server
drop column last_seen;
