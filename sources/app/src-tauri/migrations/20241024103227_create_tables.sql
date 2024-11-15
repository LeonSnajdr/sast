create table if not exists setting (
    id blob not null primary key,
    meta_date_updated text not null,
    presentation_language text not null,
    presentation_theme text not null
);

create table if not exists project (
    id blob not null primary key,
    name text not null,
    date_created text not null,
    date_last_opened text not null
);

create unique index uidx_name on project(name);
