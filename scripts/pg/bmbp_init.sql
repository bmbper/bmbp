-- 组织机构表
drop table if exists public.bmbp_rbac_organ;
create table if not exists public.bmbp_rbac_organ
(
    r_id             varchar(36)                                   not null,
    r_level          varchar(4)  default 0                         not null,
    r_flag           varchar(4)  default 0                         not null,
    r_create_time    varchar(24)                                   not null,
    r_create_user    varchar(36)                                   not null,
    r_update_time    varchar(24)                                   not null,
    r_update_user    varchar(36)                                   not null,
    r_owner_org      varchar(36)                                   not null,
    r_owner_user     varchar(36)                                   not null,
    r_sign           varchar(512),
    organ_id         varchar(36)                                   not null,
    organ_parent_id  varchar(36) default 0                         not null,
    organ_title      varchar(64)                                   not null,
    organ_title_path varchar(4096),
    organ_type       varchar(36) default 'Unit'::character varying not null,
    organ_data_id    varchar(36),
    organ_id_path    varchar(4096)                                 not null
);

comment on column public.bmbp_rbac_organ.r_id is '记录ID';

comment on column public.bmbp_rbac_organ.r_level is '记录级別';

comment on column public.bmbp_rbac_organ.r_flag is '记录标识';

comment on column public.bmbp_rbac_organ.r_create_time is '创建时间';

comment on column public.bmbp_rbac_organ.r_create_user is '创建人';

comment on column public.bmbp_rbac_organ.r_update_time is '更新时间';

comment on column public.bmbp_rbac_organ.r_update_user is '更新人';

comment on column public.bmbp_rbac_organ.r_owner_org is '所属组织';

comment on column public.bmbp_rbac_organ.r_owner_user is '所属人';

comment on column public.bmbp_rbac_organ.r_sign is '记录数签';

comment on column public.bmbp_rbac_organ.organ_id is '组织ID';

comment on column public.bmbp_rbac_organ.organ_parent_id is '上级组织ID';

comment on column public.bmbp_rbac_organ.organ_title is '组织名称';

comment on column public.bmbp_rbac_organ.organ_title_path is '组织路径';

comment on column public.bmbp_rbac_organ.organ_type is '组织类型';

comment on column public.bmbp_rbac_organ.organ_data_id is '组织识別标识';

alter table public.bmbp_rbac_organ
    owner to bmbp;

