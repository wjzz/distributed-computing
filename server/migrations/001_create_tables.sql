create table state (
    from_ bigint not null,
    to_ bigint not null,
    step bigint not null
);

insert into
    state(from_, to_, step)
values
    (1, 1000000000, 1000000);

create table queue (
    client_name text primary key,
    task_from bigint not null,
    task_to bigint not null,
    queued_at timestamp with time zone not null default current_timestamp
);

create table results (
    task_from bigint not null,
    task_to bigint not null,
    result text not null,
    client_name text not null,
    started_at timestamp with time zone not null,
    finished_at timestamp with time zone not null default current_timestamp
);