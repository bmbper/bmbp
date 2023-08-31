const PageApi = {
  queryOrganTreeUrl: '/rbac/v1/organ/find/tree',
  queryPageUrl: '/rbac/v1/user/find/page',
  queryUserInfoUrl: '/rbac/v1/user/find/info/id/',
  saveUserUrl: '/rbac/v1/user/save',
  changeUserOrganUrl: '/rbac/v1/user/update/organ/',
  removeUserUrl: '/rbac/v1/user/remove/id/',
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
const onQueryRightGridData = (queryParams) => {
  queryParams = queryParams || {}
  queryParams.pageNo = PageContext.pageConfig.current;
  queryParams.pageSize = PageContext.pageConfig.pageSize;
  let searchFormData = PageContext.searchFormRef.current.getFieldsValue();
  if (PageContext.leftTreeSelectNode) {
    queryParams.organCode = PageContext.leftTreeSelectNode.organCode;
  }
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
  PageContext.setPagination({ ...PageContext.pagination, pageSize: page.pageSize });
}

const onSearchFormQueryEvent = () => {
  onQueryRightGridData({});
}

const onSearchFormRestEvent = () => {
  PageContext.formRef.current.resetFields();
}
const onAddForm = () => {
  PageContext.setOrganFromDailogTitle("新增组织");
  PageContext.setAddOrganFormShow(true);
  PageContext.setInitOrganValue({ organParentTitle: "", organParentCode: "0" });
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
const onEditForm = (formValue) => {
  PageContext.setOrganFromDailogTitle("编辑组织");
  PageContext.setEditOrganFormShow(true);
  PageContext.setInitOrganValue({ recordId: formValue.recordId });
}
const onInfoForm = (formValue) => {
  PageContext.setOrganFromDailogTitle("查看组织");
  PageContext.setInfoOrganFormShow(true);
  PageContext.setInitOrganValue({ recordId: formValue.recordId });
}
const onConfigForm = (formValue) => {
  PageContext.setOrganFromDailogTitle("查看组织");
  PageContext.setInfoOrganFormShow(true);
  PageContext.setInitOrganValue({ recordId: formValue.recordId });
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
const onDisableEvent = (organ) => {
  BmbpHttp.post(PageApi.disableUserUrl + organ.recordId, {}).then((resp) => {
    if (resp.code == 0) {
      arco.Message.info(resp.msg);
      onQueryRightGridData({});
      onQueryLeftTreeData({});
    } else {
      arco.Message.error(resp.msg);
    }
  });
}
const onDeleteEvent = (organ) => {
  BmbpHttp.post(PageApi.removeUserUrl + organ.recordId, {}).then((resp) => {
    if (resp.code == 0) {
      onQueryRightGridData({});
      onQueryLeftTreeData({});
    } else {
      arco.Message.error(resp.msg);
    }
  });
}
const onResetPasswordEvent = (organ) => {
  BmbpHttp.post(PageApi.removeUserUrl + organ.recordId, {}).then((resp) => {
    if (resp.code == 0) {
      onQueryRightGridData({});
      onQueryLeftTreeData({});
    } else {
      arco.Message.error(resp.msg);
    }
  });
}
const onChangeOrganEvent = (organ) => {
  PageContext.setOrganFromDailogTitle("选择上级");
  PageContext.setInitOrganValue({ recordId: organ.recordId });
  PageContext.setChangeParentOrganShow(true);
}

const onQueryFormInfo = (recordId, formRef) => {
  BmbpHttp.post(PageApi.queryUserInfoUrl + recordId, {}).then((resp) => {
    if (resp.code == 0) {
      formRef.setFieldsValue(resp.data);
    } else {
      arco.Message.error(resp.msg);
    }
  });
}

const onSaveFormInfo = (formData, set_model) => {
  BmbpHttp.post(PageApi.saveUserUrl, formData).then((resp) => {
    if (resp.code == 0) {
      arco.Message.info(resp.msg);
      set_model(false);
      onQueryRightGridData({});
      onQueryLeftTreeData({});
    } else {
      arco.Message.error(resp.msg);
    }
  });
}

const onQueryChangeOrganTreeData = () => {
  BmbpHttp.get(PageApi.queryTreeWithOutOrganUrl + PageContext.initOrganValue.recordId, {}).then((resp) => {
    if (resp.code == 0) {
      PageContext.setTreeParentData(resp.data);
    } else {
      arco.Message.error(resp.msg);
    }
  });
}
const onSaveOrganChangeInfo = (recordId, organParentCode) => {
  BmbpHttp.post(PageApi.changeUserOrganUrl + recordId + "/" + organParentCode, {}).then((resp) => {
    if (resp.code == 0) {
      arco.Message.info(resp.msg);
      onQueryLeftTreeData({});
      onQueryRightGridData({});
    } else {
      arco.Message.error(resp.msg);
    }
  });
}
