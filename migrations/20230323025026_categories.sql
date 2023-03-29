-- add the article tables
drop type if exists category_type;

create type category_type as ENUM (
  'Essential','NonEssential'
  );

create table if not exists categories
(
    id         uuid DEFAULT uuid_generate_v4 (),
    name        varchar     not null default '',
    cat_type        category_type not null default 'NonEssential',
    user_id     uuid      not null references users (id) on delete cascade,
    created_at  timestamptz not null default current_timestamp,
    updated_at  timestamptz not null default current_timestamp
);

alter table categories
  add constraint categories_id_pk primary key (id);
