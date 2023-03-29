-- Add migration script here
create extension if not exists "uuid-ossp";

create table if not exists users
(
    id         uuid DEFAULT uuid_generate_v4 (),
    name       varchar     not null default '',
    email      varchar     not null default '',
    password   varchar     not null default '',
    created_at timestamptz not null default current_timestamp,
    updated_at timestamptz not null default current_timestamp
);

alter table users
    add constraint users_id_pk primary key (id);

create unique index if not exists users_email_idx on users (email);
