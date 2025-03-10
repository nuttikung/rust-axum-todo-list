create table if not exists todos
(
    id          bigserial primary key,
    description text    not null,
    completed   boolean not null default false,
    created_at  timestamptz not null default now(),
    updated_at  timestamptz not null default now()
);

create index on todos(created_at desc);
