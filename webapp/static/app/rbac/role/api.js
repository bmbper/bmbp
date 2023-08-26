const RoleApi = {
  queryTreeUrl: '/rbac/v1/role/find/tree',
  queryTreeWithOutRoleUrl: '/rbac/v1/role/find/tree/with/out/id/',
  queryPageUrl: '/rbac/v1/role/find/page',
  queryInfoUrl: '/rbac/v1/role/find/info/id/',
  saveUrl: '/rbac/v1/role/save',
  changeParentUrl: '/rbac/v1/role/update/parent/',
  removeUrl: '/rbac/v1/role/remove/id/',
  disableUrl: '/rbac/v1/role/disable/id/',
  enableUrl: '/rbac/v1/role/enable/id/'
}

const onAddRootRole = () => {
  AppIns.setRoleFromDailogTitle("新增组织");
  AppIns.setAddRoleFormShow(true);
  AppIns.setInitRoleValue({ roleParentTitle: "", roleParentCode: "0" });
}
const onRefreshRoleTree = () => {
  onQueryTreeData({});
}
const onRoleTreeNodeClick = (role) => {
  AppIns.setCurrentRoleCode(role.roleCode);
}
const onChangeRoleParent = (role) => {
  AppIns.setRoleFromDailogTitle("选择上级");
  AppIns.setInitRoleValue({ recordId: role.recordId });
  AppIns.setChangeParentRoleShow(true);
}
const onAddRoleChild = (role) => {
  AppIns.setRoleFromDailogTitle("新增下级组织");
  AppIns.setAddRoleFormShow(true);
  AppIns.setInitRoleValue({ roleParentTitle: role.roleTitle, roleParentCode: role.roleCode });
}
const onEditRole = (role) => {
  AppIns.setRoleFromDailogTitle("编辑组织");
  AppIns.setEditRoleFormShow(true);
  AppIns.setInitRoleValue({ recordId: role.recordId });
}
const onInfoRole = (role) => {
  AppIns.setRoleFromDailogTitle("查看组织");
  AppIns.setInfoRoleFormShow(true);
  AppIns.setInitRoleValue({ recordId: role.recordId });
}
const onEnableRole = (role) => {
  BmbpHttp.post(RoleApi.enableUrl + role.recordId, {}).then((resp) => {
    if (resp.code == 0) {
      arco.Message.info(resp.msg);
      onQueryPageData({});
      onQueryTreeData({});
    } else {
      arco.Message.error(resp.msg);
    }
  });
}

const onDisableRole = (role) => {
  BmbpHttp.post(RoleApi.disableUrl + role.recordId, {}).then((resp) => {
    if (resp.code == 0) {
      arco.Message.info(resp.msg);
      onQueryPageData({});
      onQueryTreeData({});
    } else {
      arco.Message.error(resp.msg);
    }
  });
}

const onDeleteRole = (role) => {
  BmbpHttp.post(RoleApi.removeUrl + role.recordId, {}).then((resp) => {
    if (resp.code == 0) {
      onQueryPageData({});
      onQueryTreeData({});
    } else {
      arco.Message.error(resp.msg);
    }
  });
}


const onEditRoleInfo = (role) => {
  arco.Message.info("配置组织明细信息");
}

const onBatchDeleteRole = (roleIds) => {
  arco.Message.info("批量删除组织节点:" + JSON.stringify(roleIds));
}
const onToolBarImportBtnClick = () => {
  arco.Message.info("导入功能开发中...");
}
const onToolBarExportBtnClick = () => {
  arco.Message.info("导出功能开发中...");
}
const onToolBarPrintBtnClick = () => {
  arco.Message.info("打印功能开发中...");
}

const onSearchFormQueryBtnClick = () => {
  onQueryPageData({});
}

const onSearchFormRestBtnClick = () => {
  AppIns.formRef.current.resetFields();
}

/// 列表分页大小变化
const onGridPageChange = (page) => {
  AppIns.setPagination({ ...AppIns.pagination, pageSize: page.pageSize });
}

const onQueryTreeData = () => {
  BmbpHttp.post(RoleApi.queryTreeUrl, {}).then((resp) => {
    if (resp.code == 0) {
      AppIns.setRoleTreeData(resp.data);
    } else {
      arco.Message.error(resp.msg);
    }
  });
}

const onQueryPageData = (queryParams) => {
  queryParams = queryParams || {}
  queryParams.pageNo = AppIns.pagination.current;
  queryParams.pageSize = AppIns.pagination.pageSize;
  let searchFormData = AppIns.searchFormRef.current.getFieldsValue();
  if (AppIns.currentRoleCode) {
    queryParams.roleParentCode = AppIns.currentRoleCode;
  }
  Object.assign(queryParams, searchFormData);
  BmbpHttp.post(RoleApi.queryPageUrl, queryParams).then((resp) => {
    if (resp.code == 0) {
      let respData = resp.data;
      AppIns.setPagination({ ...AppIns.pagination, total: respData.rowTotal });
      AppIns.setRoleGridData(respData.data);
    } else {
      arco.Message.error(resp.msg);
    }
  });
}

const onQueryRoleInfo = (recordId, formRef) => {
  BmbpHttp.post(RoleApi.queryInfoUrl + recordId, {}).then((resp) => {
    if (resp.code == 0) {
      formRef.setFieldsValue(resp.data);
    } else {
      arco.Message.error(resp.msg);
    }
  });
}

const saveRoleInfo = (formData, set_model) => {
  BmbpHttp.post(RoleApi.saveUrl, formData).then((resp) => {
    if (resp.code == 0) {
      arco.Message.info(resp.msg);
      set_model(false);
      onQueryPageData({});
      onQueryTreeData({});
    } else {
      arco.Message.error(resp.msg);
    }
  });
}

/// 查询排除节点的组织结构数据
const onQueryTreeDataWithOutRecordId = () => {
  BmbpHttp.get(RoleApi.queryTreeWithOutRoleUrl + AppIns.initRoleValue.recordId, {}).then((resp) => {
    if (resp.code == 0) {
      AppIns.setTreeParentData(resp.data);
    } else {
      arco.Message.error(resp.msg);
    }
  });
}

const onSaveRoleParentChange = (recordId, roleParentCode) => {
  BmbpHttp.post(RoleApi.changeParentUrl + recordId + "/" + roleParentCode, {}).then((resp) => {
    if (resp.code == 0) {
      arco.Message.info(resp.msg);
      onQueryTreeData({});
      onQueryPageData({});
    } else {
      arco.Message.error(resp.msg);
    }
  });
}
