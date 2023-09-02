const PageApi = {
  queryOrganTreeUrl: '/rbac/v1/organ/find/tree',
  queryPageUrl: '/rbac/v1/user/find/page',
  queryInfoUrl: '/rbac/v1/user/find/info/id/',
  saveInfoUrl: '/rbac/v1/user/save',
  changeUserOrganUrl: '/rbac/v1/user/update/organ/',
  removeUrl: '/rbac/v1/user/remove/id/',
  disableUserUrl: '/rbac/v1/user/disable/id/',
  enableUserUrl: '/rbac/v1/user/enable/id/',
  resetUserPasswordUrl: '/rbac/v1/user/update/reset/password/id/'
}
const onQueryLeftTreeData = () => {
  BmbpHttp.post(PageApi.queryOrganTreeUrl, {}).then((resp) => {
    if (resp.code == 0) {
      PageContext.setLeftTreeData(resp.data);
    } else {
      arco.Message.error(resp.msg);
    }
  });
}
const onRefreshLeftTreeData = () => {
  onQueryLeftTreeData({});
}
const onLeftTreeNodeClick = (nodeData) => {
  PageContext.setLeftTreeSelectNode(nodeData);
}
const onQueryRightGridData = () => {
  let queryParams = {};
  queryParams.pageNo = PageContext.pageConfig.current;
  queryParams.pageSize = PageContext.pageConfig.pageSize;
  if (PageContext.leftTreeSelectNode) {
    queryParams.organId = PageContext.leftTreeSelectNode.recordId;
  }
  let searchFormData = PageContext.searchFormRef.current.getFieldsValue();
  Object.assign(queryParams, searchFormData);
  BmbpHttp.post(PageApi.queryPageUrl, queryParams).then((resp) => {
    if (resp.code == 0) {
      let respData = resp.data;
      PageContext.setPageConfig({ ...PageContext.pageConfig, total: respData.rowTotal });
      PageContext.setRightGridData(respData.data);
    } else {
      arco.Message.error(resp.msg);
    }
  });
}
const onGridPageConfigChange = (page) => {
  PageContext.setPagination({ ...PageContext.pageConfig, pageSize: page.pageSize });
}

const onSearchFormQueryEvent = () => {
  onQueryRightGridData();
}

const onSearchFormRestEvent = () => {
  PageContext.formRef.current.resetFields();
}
const onAddForm = () => {
  PageContext.setInitFormValue({ organId: PageContext.leftTreeSelectNode.recordId });
  PageContext.setFormTitle("新增用户");
  PageContext.setAddFormShow(true);
}
const onBatchDeleteEvent = (organIds) => {
  arco.Message.info("批量删除组织节点:" + JSON.stringify(organIds));
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
const onEditForm = (record) => {
  PageContext.setInitFormValue({ recordId: record.recordId });
  PageContext.setFormTitle("编辑用户");
  PageContext.setEditFormShow(true);
}
const onInfoForm = (record) => {
  PageContext.setInitFormValue({ recordId: record.recordId });
  PageContext.setFormTitle("查看用户");
  PageContext.setInfoFormShow(true);
}
const onConfigForm = (record) => {
  PageContext.setInitFormValue({ recordId: record.recordId });
  PageContext.setFormTitle("配置用户");
  PageContext.setConfigFormShow(true);
}
const onEnableEvent = (organ) => {
  BmbpHttp.post(PageApi.enableUserUrl + organ.recordId, {}).then((resp) => {
    if (resp.code == 0) {
      arco.Message.info(resp.msg);
      onQueryRightGridData({});
      onQueryLeftTreeData({});
    } else {
      arco.Message.error(resp.msg);
    }
  });
}
const onDisableEvent = (record) => {
  BmbpHttp.post(PageApi.disableUserUrl + record.recordId, {}).then((resp) => {
    if (resp.code == 0) {
      arco.Message.info(resp.msg);
      onQueryRightGridData();
      onQueryLeftTreeData();
    } else {
      arco.Message.error(resp.msg);
    }
  });
}
const onDeleteEvent = (record) => {
  BmbpHttp.post(PageApi.removeUrl + record.recordId, {}).then((resp) => {
    if (resp.code == 0) {
      onQueryRightGridData();
      onQueryLeftTreeData();
    } else {
      arco.Message.error(resp.msg);
    }
  });
}
const onResetPasswordEvent = (record) => {
  BmbpHttp.post(PageApi.resetUserPasswordUrl + record.recordId, {}).then((resp) => {
    if (resp.code == 0) {
      arco.Message.info(resp.msg);
    } else {
      arco.Message.error(resp.msg);
    }
  });
}
const onChangeOrganEvent = (record) => {
  PageContext.setInitFormValue({ recordId: record.recordId });
  PageContext.setFormTitle("变更组织");
  PageContext.setChangeOrganShow(true);
}

const onQueryFormInfo = (recordId, set_form_data) => {
  BmbpHttp.post(PageApi.queryInfoUrl + recordId, {}).then((resp) => {
    if (resp.code == 0) {
      set_form_data(resp.data);
    } else {
      arco.Message.error(resp.msg);
    }
  });
}

const onSaveFormInfo = (formData, callback) => {
  BmbpHttp.post(PageApi.saveInfoUrl, formData).then((resp) => {
    if (resp.code == 0) {
      arco.Message.info(resp.msg);
      onQueryRightGridData({});
      onQueryLeftTreeData({});
      callback();
    } else {
      arco.Message.error(resp.msg);
    }
  });
}

const onQueryChangeOrganTreeData = () => {
  BmbpHttp.post(PageApi.queryOrganTreeUrl, {}).then((resp) => {
    if (resp.code == 0) {
      PageContext.setChangeOrganTreeData(resp.data);
    } else {
      arco.Message.error(resp.msg);
    }
  });
}
const onSaveOrganChangeInfo = (recordId, organId) => {
  BmbpHttp.post(PageApi.changeUserOrganUrl + recordId + "/" + organId, {}).then((resp) => {
    if (resp.code == 0) {
      arco.Message.info(resp.msg);
      onQueryLeftTreeData({});
      onQueryRightGridData({});
    } else {
      arco.Message.error(resp.msg);
    }
  });
}
