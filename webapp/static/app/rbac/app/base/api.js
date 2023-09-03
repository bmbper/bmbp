const PageApi = {
  queryPageUrl: "/rbac/v1/app/find/page",
  queryInfoUrl: "/rbac/v1/app/find/info/",
  enableUrl: "/rbac/v1/app/enable/",
  disableUrl: "/rbac/v1/app/disable/",
  reStartUrl: "/rbac/v1/app/restart/",
  removeUrl: "/rbac/v1/app/delete/",
  saveInfoUrl: "/rbac/v1/app/save"
}

const onQueryGridData = () => {
  let queryParams = {};
  queryParams.pageNo = PageContext.pageConfig.current;
  queryParams.pageSize = PageContext.pageConfig.pageSize;
  let searchFormData = PageContext.searchFormRef.current.getFieldsValue();
  Object.assign(queryParams, searchFormData);
  BmbpHttp.post(PageApi.queryPageUrl, queryParams).then((resp) => {
    if (resp.code == 0) {
      let respData = resp.data;
      PageContext.setPageConfig({ ...PageContext.pageConfig, total: respData.rowTotal });
      PageContext.setGridData(respData.data);
    } else {
      arco.Message.error(resp.msg);
    }
  });
}
const onGridPageConfigChange = (page) => {
  PageContext.setPagination({ ...PageContext.pageConfig, pageSize: page.pageSize });
}
const onSearchFormQueryEvent = () => {
  onQueryGridData();
}
const onSearchFormRestEvent = () => {
  PageContext.formRef.current.resetFields();
}
const onAddForm = () => {
  PageContext.setInitFormValue({});
  PageContext.setFormTitle("新增应用");
  PageContext.setAddFormShow(true);
}
const onBatchDeleteEvent = (organIds) => {
  arco.Message.info("批量删除应用:" + JSON.stringify(organIds));
}
const onImportEvent = () => {
  arco.Message.info("导入功能开发中...");
}
const onExportEvent = () => {
  arco.Message.info("导出功能开发中...");
}
const onPrintEvent = () => {
  arco.Message.info("打印功能开发中...");
}
const onEditForm = (record) => {
  PageContext.setInitFormValue({ recordId: record.recordId });
  PageContext.setFormTitle("编辑应用");
  PageContext.setEditFormShow(true);
}
const onInfoForm = (record) => {
  PageContext.setInitFormValue({ recordId: record.recordId });
  PageContext.setFormTitle("查看应用");
  PageContext.setInfoFormShow(true);
}
const onConfigForm = (record) => {
  window.parent.postMessage({ from: 'app', recordId: record.recordId }, "*");
}
const onEnableEvent = (organ) => {
  BmbpHttp.post(PageApi.enableUrl + organ.recordId, {}).then((resp) => {
    if (resp.code == 0) {
      arco.Message.info(resp.msg);
      onQueryGridData();
    } else {
      arco.Message.error(resp.msg);
    }
  });
}
const onDisableEvent = (record) => {
  BmbpHttp.post(PageApi.disableUrl + record.recordId, {}).then((resp) => {
    if (resp.code == 0) {
      arco.Message.info(resp.msg);
      onQueryGridData();
    } else {
      arco.Message.error(resp.msg);
    }
  });
}
const onDeleteEvent = (record) => {
  BmbpHttp.post(PageApi.removeUrl + record.recordId, {}).then((resp) => {
    if (resp.code == 0) {
      onQueryGridData();
    } else {
      arco.Message.error(resp.msg);
    }
  });
}

const onRestartEvent = (record) => {
  BmbpHttp.post(PageApi.reStartUrl + record.recordId, {}).then((resp) => {
    if (resp.code == 0) {
      onQueryGridData();
    } else {
      arco.Message.error(resp.msg);
    }
  });
}


const onQueryFormInfo = (recordId, callback) => {
  BmbpHttp.post(PageApi.queryInfoUrl + recordId, {}).then((resp) => {
    if (resp.code == 0) {
      callback(resp.data);
    } else {
      arco.Message.error(resp.msg);
    }
  });
}

const onSaveFormInfo = (formData, callback) => {
  BmbpHttp.post(PageApi.saveInfoUrl, formData).then((resp) => {
    if (resp.code == 0) {
      arco.Message.info(resp.msg);
      onQueryGridData();
      callback();
    } else {
      arco.Message.error(resp.msg);
    }
  });
}
