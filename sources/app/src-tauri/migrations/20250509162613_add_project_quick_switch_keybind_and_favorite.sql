alter table project
    add column quick_switch_keybind text default null;

create unique index uidx_project_quick_switch_keybind on project (quick_switch_keybind);

alter table project
    add column favorite boolean not null default false;