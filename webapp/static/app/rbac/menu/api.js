const MenuApi = {
  queryTreeUrl: '/rbac/v1/menu/find/tree',
  queryTreeWithOutMenuUrl: '/rbac/v1/menu/find/tree/with/out/',
  queryPageUrl: '/rbac/v1/menu/find/page',
  queryInfoUrl: '/rbac/v1/menu/find/info/id/',
  saveUrl: '/rbac/v1/menu/save',
  changeParentUrl: '/rbac/v1/menu/update/parent/',
  removeUrl: '/rbac/v1/menu/remove/id/',
  disableUrl: '/rbac/v1/menu/disable/id/',
  enableUrl: '/rbac/v1/menu/enable/id/'
}

const onAddRootMenu = () => {
  AppIns.setMenuFromDailogTitle("新增菜单");
  AppIns.setAddMenuFormShow(true);
  AppIns.setInitMenuValue({ menuParentTitle: "", menuParentCode: "0", appId: PageVars.appId });
}
const onRefreshMenuTree = () => {
  AppIns.setCurrentMenuCode("");
}
const onMenuTreeNodeClick = (menu) => {
  AppIns.setCurrentMenuCode(menu.menuCode);
}
const onChangeMenuParent = (menu) => {
  AppIns.setMenuFromDailogTitle("选择上级");
  AppIns.setInitMenuValue({ recordId: menu.recordId });
  AppIns.setChangeParentMenuShow(true);
}
const onAddMenuChild = (menu) => {
  AppIns.setMenuFromDailogTitle("新增下级菜单");
  AppIns.setAddMenuFormShow(true);
  AppIns.setInitMenuValue({ menuParentTitle: menu.menuTitle, menuParentCode: menu.menuCode, appId: menu.appId });
}
const onEditMenu = (menu) => {
  AppIns.setMenuFromDailogTitle("编辑菜单");
  AppIns.setEditMenuFormShow(true);
  AppIns.setInitMenuValue({ recordId: menu.recordId });
}
const onInfoMenu = (menu) => {
  AppIns.setMenuFromDailogTitle("查看菜单");
  AppIns.setInfoMenuFormShow(true);
  AppIns.setInitMenuValue({ recordId: menu.recordId });
}
const onEnableMenu = (menu) => {
  BmbpHttp.post(MenuApi.enableUrl + menu.recordId, {}).then((resp) => {
    if (resp.code == 0) {
      arco.Message.info(resp.msg);
      onQueryPageData({});
      onQueryTreeData({});
    } else {
      arco.Message.error(resp.msg);
    }
  });
}

const onDisableMenu = (menu) => {
  BmbpHttp.post(MenuApi.disableUrl + menu.recordId, {}).then((resp) => {
    if (resp.code == 0) {
      arco.Message.info(resp.msg);
      onQueryPageData({});
      onQueryTreeData({});
    } else {
      arco.Message.error(resp.msg);
    }
  });
}

const onDeleteMenu = (menu) => {
  BmbpHttp.post(MenuApi.removeUrl + menu.recordId, {}).then((resp) => {
    if (resp.code == 0) {
      onQueryPageData({});
      onQueryTreeData({});
    } else {
      arco.Message.error(resp.msg);
    }
  });
}


const onEditMenuInfo = (menu) => {
  arco.Message.info("配置菜单明细信息");
}

const onBatchDeleteMenu = (menuIds) => {
  arco.Message.info("批量删除菜单节点:" + JSON.stringify(menuIds));
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
  BmbpHttp.post(MenuApi.queryTreeUrl, { appId: PageVars.appId }).then((resp) => {
    if (resp.code == 0) {
      AppIns.setMenuTreeData(resp.data);
    } else {
      arco.Message.error(resp.msg);
    }
  });
}

const onQueryPageData = (queryParams) => {
  queryParams = queryParams || {}
  queryParams.pageNo = AppIns.pagination.current;
  queryParams.pageSize = AppIns.pagination.pageSize;
  queryParams.appId = PageVars.appId;
  let searchFormData = AppIns.searchFormRef.current.getFieldsValue();
  if (AppIns.currentMenuCode) {
    queryParams.menuParentCode = AppIns.currentMenuCode;
  }
  Object.assign(queryParams, searchFormData);
  BmbpHttp.post(MenuApi.queryPageUrl, queryParams).then((resp) => {
    if (resp.code == 0) {
      let respData = resp.data;
      AppIns.setPagination({ ...AppIns.pagination, total: respData.rowTotal });
      AppIns.setMenuGridData(respData.data);
    } else {
      arco.Message.error(resp.msg);
    }
  });
}

const onQueryMenuInfo = (recordId, formRef) => {
  BmbpHttp.post(MenuApi.queryInfoUrl + recordId, {}).then((resp) => {
    if (resp.code == 0) {
      formRef.setFieldsValue(resp.data);
    } else {
      arco.Message.error(resp.msg);
    }
  });
}

const saveMenuInfo = (formData, set_model) => {
  BmbpHttp.post(MenuApi.saveUrl, formData).then((resp) => {
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

/// 查询排除节点的菜单结构数据
const onQueryTreeDataWithOutRecordId = () => {
  BmbpHttp.get(MenuApi.queryTreeWithOutMenuUrl + "/app/" + PageVars.appId + "/id/" + AppIns.initMenuValue.recordId, {}).then((resp) => {
    if (resp.code == 0) {
      AppIns.setTreeParentData(resp.data);
    } else {
      arco.Message.error(resp.msg);
    }
  });
}

const onSaveMenuParentChange = (recordId, menuParentCode) => {
  BmbpHttp.post(MenuApi.changeParentUrl + recordId + "/" + menuParentCode, {}).then((resp) => {
    if (resp.code == 0) {
      arco.Message.info(resp.msg);
      onQueryTreeData({});
      onQueryPageData({});
    } else {
      arco.Message.error(resp.msg);
    }
  });
}
