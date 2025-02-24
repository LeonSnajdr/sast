create table if not exists setting
(
    id                    blob not null,
    meta_date_updated     text not null,
    presentation_language text not null,
    presentation_theme    text not null,
    primary key (id)
);

create table if not exists project
(
    id               blob not null,
    name             text not null,
    date_created     text not null,
    date_last_opened text not null,
    primary key (id)
);

create unique index uidx_project_name on project (name);

create table if not exists placeholder
(
    id                blob not null,
    project_id        blob not null,
    name              text not null,
    value             text not null,
    visibility        text not null,
    kind              text not null,
    source            text not null,
    date_created      text not null,
    date_last_updated text not null,
    primary key (id),
    foreign key (project_id) references project (id) on delete cascade
);

create unique index uidx_placeholder_project_id_name on placeholder (project_id, name);

create table if not exists placeholder_insert_tile
(
    id                  blob    not null,
    placeholder_id      blob,
    task_command_id     blob,
    task_working_dir_id blob,
    kind                text    not null,
    position            integer not null,
    text_value          text,
    primary key (id),
    foreign key (task_command_id) references task (id) on delete cascade,
    foreign key (task_working_dir_id) references task (id) on delete cascade,
    foreign key (placeholder_id) references placeholder (id) on delete cascade
);

create table if not exists task
(
    id                blob    not null,
    project_id        blob    not null,
    name              text    not null,
    tab_name          text,
    no_exit           boolean not null,
    date_created      text    not null,
    date_last_updated text    not null,
    primary key (id),
    foreign key (project_id) references project (id) on delete cascade
);

create unique index uidx_task_project_id_name on task (project_id, name);




