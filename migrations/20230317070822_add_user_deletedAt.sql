alter table users
  add deleted_at  timestamptz default null;
