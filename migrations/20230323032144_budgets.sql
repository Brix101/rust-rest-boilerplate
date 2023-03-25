-- add the article tables
drop type if exists plan_type;

create type plan_type as ENUM (
  'Daily','Weekly','Monthly'
  );

create table if not exists budgets
(
    id          uuid DEFAULT uuid_generate_v4 (),
    category_id     uuid      not null references categories (id) on delete cascade,
    amount      float      not null default 0.00,
    description varchar    not null default '',
    plan        plan_type  not null default 'Monthly',
    created_at  timestamptz not null default current_timestamp,
    updated_at  timestamptz not null default current_timestamp,
    deleted_at  timestamptz default null
);

alter table budgets
    add constraint budgets_id_pk primary key (id);
