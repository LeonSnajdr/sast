-- WORKAROUND: To disable foreign keys; https://github.com/launchbadge/sqlx/issues/2085
commit transaction;

pragma foreign_keys = off;

begin transaction;

create table placeholder_new
(
    id                blob not null,
    project_id        blob,
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

insert into placeholder_new (id, project_id, name, value, visibility, kind, source, date_created, date_last_updated)
select id,
       project_id,
       name,
       value,
       visibility,
       kind,
       source,
       date_created,
       date_last_updated
from placeholder;

drop table placeholder;

alter table placeholder_new
    rename to placeholder;

update placeholder
set project_id = null
where visibility = 'Global';

drop index if exists uidx_placeholder_project_id_name;

create unique index uidx_placeholder_project_id_name on placeholder (coalesce(project_id, 'null'), name);

commit transaction;

pragma foreign_keys = on;

begin transaction;