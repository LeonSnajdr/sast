create table if not exists setting (
    id blob not null,
    meta_date_updated text not null,
    presentation_language text not null,
    presentation_theme text not null,
    primary key (id)
);

create table if not exists project (
    id blob not null,
    name text not null,
    date_created text not null,
    date_last_opened text not null,
    primary key (id)
);

create table if not exists placeholder (
    id blob not null,
    project_id blob,
    name text not null,
    value text not null,
    visibility text not null,
    kind text not null,
    source text not null,
    date_created text not null,
    date_last_updated text not null,
    primary key (id),
    foreign key (project_id) references project(id)
);

create unique index uidx_name on project(name);
create unique index uidx_project_id_name on placeholder(coalesce(project_id, -1), name);
