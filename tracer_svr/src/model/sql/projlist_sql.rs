pub static PROJ_LIST_TABLE_SQL: &str = "
CREATE TABLE IF NOT EXISTS `<TABLE_NAME>` (
    `id`                    integer     primary key autoincrement not null,
    `create_time`           datetime    default (strftime('%Y-%m-%d %H:%M:%f','now','localtime')),
    `update_time`           datetime    default (strftime('%Y-%m-%d %H:%M:%f','now','localtime')),
    `name`                  text        default null,
    `desc`                  text        default null,
    `avatar`                text        default null
);";