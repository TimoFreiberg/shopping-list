create table if not exists open_items
(
    id bigserial primary key,
    name text not null,
    created_at timestamptz not null
);

create table if not exists done_items
(
    id bigserial primary key,
    name text not null,
    created_at timestamptz not null,
    done_at timestamptz not null
);
