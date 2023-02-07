pub static TRACE_TABLE_SQL: &str = "
CREATE TABLE IF NOT EXISTS `<TABLE_NAME>`
(
    `id`                    integer     primary key not null,
    `create_time`           datetime    default (strftime('%Y-%m-%d %H:%M:%f','now','localtime')),
    `bug_err_txt`          text        default null,
    `bug_err_md5`          text        default null,
    `bug_ctx`               text        default null,
    `bug_tag`               text        default null,
    `app_token`              text        default null,
    `app_cuid`              text        default null,
    `app_uid`               text        default null,
    `app_version`           text        default null,
    `app_os`                text        default null,
    `app_channel`                text        default null,
    `app_device`                text        default null,
    `app_business_version`                text        default null
);

create index if not exists BugRawData_app_cuid_index
	on BugRawData (app_cuid);

create index if not exists BugRawData_app_os_index
	on BugRawData (app_os);

create index if not exists BugRawData_app_uid_index
	on BugRawData (app_uid);

create index if not exists BugRawData_app_version_index
	on BugRawData (app_version);

create index if not exists BugRawData_bug_type_md5_index
	on BugRawData (bug_err_md5);

create index if not exists BugRawData_create_time_index
	on BugRawData (create_time desc);
";

pub static TRACE_STAT_TABLE_SQL: &str = "
CREATE TABLE IF NOT EXISTS `<TABLE_NAME>`
(
    `id`                    integer     primary key not null,
    `uid` integer default null,
    `create_time`           datetime    default (strftime('%Y-%m-%d %H:%M:%f','now','localtime')),
    `bug_type_md5`          text        default null,
    `app_version`           text        default null,
    `app_business_version`                text        default null
);
";