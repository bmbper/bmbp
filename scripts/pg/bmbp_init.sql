create table bmbp_setting_dict
(
    code             varchar(36)                                not null,
    code_path        text                                       not null,
    parent_code      varchar(36) default '0'::character varying not null,
    name             varchar(64)                                not null,
    name_path        text                                       not null,
    edit_type        varchar(4)  default '0'::character varying not null,
    node_type        varchar(4)  default '0'::character varying not null,
    dict_alias       varchar(32) default ''::character varying  not null,
    dict_value       varchar(32) default ''::character varying  not null,
    data_id          varchar(36)                                not null
        primary key,
    data_flag        varchar(4)  default '0'::character varying not null,
    data_level       varchar(4)  default '0'::character varying not null,
    data_status      varchar(4)  default '0'::character varying not null,
    data_sort        bigint      default 0                      not null,
    data_create_time varchar(20),
    data_create_user varchar(36),
    data_update_time varchar(20),
    data_update_user varchar(36),
    data_owner_org   varchar(36),
    data_owner_user  varchar(36),
    data_sign        varchar(512),
    data_remark      varchar(512),
    node_level       bigint,
    node_leaf        bigint,
    dict_type        varchar(4)
);

comment on column bmbp_setting_dict.dict_type is '字典类型：0 内置、1自定义';

alter table bmbp_setting_dict
    owner to bmbp;

create table bmbp_rbac_organ
(
    code             varchar(36)                                not null,
    code_path        text                                       not null,
    parent_code      varchar(36) default '0'::character varying not null,
    name             varchar(64)                                not null,
    name_path        text                                       not null,
    data_id          varchar(36)                                not null
        primary key,
    data_flag        varchar(4)  default '0'::character varying not null,
    data_level       varchar(4)  default '0'::character varying not null,
    data_status      varchar(4)  default '0'::character varying not null,
    data_sort        bigint      default 0                      not null,
    data_create_time varchar(20),
    data_create_user varchar(36),
    data_update_time varchar(20),
    data_update_user varchar(36),
    data_owner_org   varchar(36),
    data_owner_user  varchar(36),
    data_sign        varchar(512),
    data_remark      varchar(512),
    node_level       bigint,
    node_leaf        bigint,
    organ_type       smallint
);

comment on column bmbp_rbac_organ.organ_type is '组织类型';

alter table bmbp_rbac_organ
    owner to bmbp;

create table bmbp_rbac_role
(
    role_code        varchar(36)                               not null,
    role_name        varchar(128)                              not null,
    role_desc        text                                      not null,
    data_id          varchar(36)                               not null
        primary key,
    data_flag        varchar(4) default '0'::character varying not null,
    data_level       varchar(4) default '0'::character varying not null,
    data_status      varchar(4) default '0'::character varying not null,
    data_sort        bigint     default 0                      not null,
    data_create_time varchar(20),
    data_create_user varchar(36),
    data_update_time varchar(20),
    data_update_user varchar(36),
    data_owner_org   varchar(36),
    data_owner_user  varchar(36),
    data_sign        varchar(512),
    data_remark      varchar(512)
);

alter table bmbp_rbac_role
    owner to bmbp;

create table bmbp_rbac_app
(
    app_code         varchar(36)                               not null,
    app_name         varchar(128)                              not null,
    app_short_name   varchar(128),
    app_icon         text,
    app_desc         varchar(512),
    app_type         varchar(64),
    app_url          varchar(256),
    data_id          varchar(36)                               not null
        primary key,
    data_flag        varchar(4) default '0'::character varying not null,
    data_level       varchar(4) default '0'::character varying not null,
    data_status      varchar(4) default '0'::character varying not null,
    data_sort        bigint     default 0                      not null,
    data_create_time varchar(20),
    data_create_user varchar(36),
    data_update_time varchar(20),
    data_update_user varchar(36),
    data_owner_org   varchar(36),
    data_owner_user  varchar(36),
    data_sign        varchar(512),
    data_remark      varchar(512),
    app_theme        varchar(64)
);

comment on column bmbp_rbac_app.app_theme is '应用专题';

alter table bmbp_rbac_app
    owner to bmbp;

create table bmbp_rbac_app_menu
(
    app_id           varchar(36)                               not null,
    menu_type        varchar(36)                               not null,
    menu_open_type   varchar(36),
    menu_icon        varchar(256),
    menu_desc        varchar(512),
    menu_url         varchar(512),
    menu_code        varchar(36)                               not null,
    menu_parent_code varchar(36)                               not null,
    menu_name        varchar(128)                              not null,
    menu_code_path   text                                      not null,
    menu_name_path   text                                      not null,
    menu_grade       bigint                                    not null,
    data_id          varchar(36)                               not null
        primary key,
    data_flag        varchar(4) default '0'::character varying not null,
    data_level       varchar(4) default '0'::character varying not null,
    data_status      varchar(4) default '0'::character varying not null,
    data_sort        bigint     default 0                      not null,
    data_create_time varchar(20),
    data_create_user varchar(36),
    data_update_time varchar(20),
    data_update_user varchar(36),
    data_owner_org   varchar(36),
    data_owner_user  varchar(36),
    data_sign        varchar(512),
    menu_meta_type   varchar(64),
    menu_short_name  varchar(64)
);

comment on column bmbp_rbac_app_menu.menu_meta_type is '配置类型';

comment on column bmbp_rbac_app_menu.menu_short_name is '资源简称';

alter table bmbp_rbac_app_menu
    owner to bmbp;

create table bmbp_rbac_user
(
    org_id           varchar(36)                               not null,
    person_id        varchar(36),
    user_name        varchar(64)                               not null,
    nick_name        varchar(64)                               not null,
    passwd           varchar(64)                               not null,
    mobile           varchar(64),
    email            varchar(64),
    data_id          varchar(36)                               not null
        primary key,
    data_flag        varchar(4) default '0'::character varying not null,
    data_level       varchar(4) default '0'::character varying not null,
    data_status      varchar(4) default '0'::character varying not null,
    data_sort        bigint     default 0                      not null,
    data_create_time varchar(20),
    data_create_user varchar(36),
    data_update_time varchar(20),
    data_update_user varchar(36),
    data_owner_org   varchar(36),
    data_owner_user  varchar(36),
    data_sign        varchar(512),
    org_id_path      text
);

comment on column bmbp_rbac_user.org_id_path is '组织路径';

alter table bmbp_rbac_user
    owner to bmbp;

create table bmbp_rbac_user_extend
(
    user_id          varchar(36)                               not null,
    pwd_status       varchar(36),
    lock_status      varchar(64),
    pwd_modify_time  varchar(64),
    pwd_modify_count bigint,
    pwd_expire_time  varchar(64),
    data_id          varchar(36)                               not null
        primary key,
    data_flag        varchar(4) default '0'::character varying not null,
    data_level       varchar(4) default '0'::character varying not null,
    data_status      varchar(4) default '0'::character varying not null,
    data_sort        bigint     default 0                      not null,
    data_create_time varchar(20),
    data_create_user varchar(36),
    data_update_time varchar(20),
    data_update_user varchar(36),
    data_owner_org   varchar(36),
    data_owner_user  varchar(36),
    data_sign        varchar(512)
);

alter table bmbp_rbac_user_extend
    owner to bmbp;

