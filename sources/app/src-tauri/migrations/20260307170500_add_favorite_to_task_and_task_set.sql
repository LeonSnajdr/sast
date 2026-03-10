alter table task
    add column favorite boolean not null default false;

alter table task_set
    add column favorite boolean not null default false;

alter table placeholder
    add column favorite boolean not null default false;
