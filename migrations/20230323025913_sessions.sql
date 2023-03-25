-- Add migration script here

create table if not exists sessions
(
    id          uuid DEFAULT uuid_generate_v4 (),
    exp         timestamptz      not null,
    user_id     uuid           not null references users (id) on delete cascade,
    user_agent  varchar          not null default ''
);

alter table sessions
    add constraint sessions_id_pk primary key (id);
