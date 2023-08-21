const OrganApi = {
  queryTreeUrl: '/rbac/v1/organ/find/tree',
  queryPageUrl: '/rbac/v1/organ/find/page',
  saveUrl: '/rbac/v1/organ/save',
  removeUrl: '/rbac/v1/organ/remove/id/',
  disableUrl: '/rbac/v1/organ/disable/id/',
  enableUrl: '/rbac/v1/organ/enable/id/'
}

const onAddRootOrgan = () => {
  AppIns.setOrganFromDailogTitle("新增组织");
  AppIns.setAddOrganFormShow(true);
  AppIns.setInitOrganValue({ organParentTitle: "", organParentCode: "0" });
}
const onRefreshOrganTree = () => {
  onQueryTreeData({});
}
const onOrganTreeNodeClick = (organ) => {
  onQueryPageData({ organParentCode: organ.organCode });
}
const onChangeOrganParent = (organ) => {
  arco.Message.info("变更组织上级");
}
const onAddOrganChild = (organ) => {
  AppIns.setOrganFromDailogTitle("新增下级组织");
  AppIns.setAddOrganFormShow(true);
  AppIns.setInitOrganValue({ organParentTitle: organ.organTitle, organParentCode: organ.organCode });
}
const onEditOrgan = (organ) => {
  arco.Message.info("编辑组织节点");
}
const onInfoOrgan = (organ) => {
  arco.Message.info("查看组织节点");
}
const onEnableOrgan = (organ) => {
  BmbpHttp.post(OrganApi.enableUrl + organ.recordId, {}).then((resp) => {
    if (resp.code == 0) {
      onQueryPageData({});
    } else {
      arco.Message.error(resp.msg);
    }
  });
}

const onDisableOrgan = (organ) => {
  BmbpHttp.post(OrganApi.disableUrl + organ.recordId, {}).then((resp) => {
    if (resp.code == 0) {
      onQueryPageData({});
    } else {
      arco.Message.error(resp.msg);
    }
  });
}

const onDeleteOrgan = (organ) => {
  BmbpHttp.post(OrganApi.removeUrl + organ.recordId, {}).then((resp) => {
    if (resp.code == 0) {
      onQueryPageData({});
      onQueryTreeData({});
    } else {
      arco.Message.error(resp.msg);
    }
  });
}


const onEditOrganInfo = (organ) => {
  arco.Message.info("配置组织明细信息");
}

const onBatchDeleteOrgan = (organIds) => {
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

const onSearchFormQueryBtnClick = () => {
  var queryData = AppIns.formRef.current.getFieldsValue();
  onQueryPageData(queryData);
}

const onSearchFormRestBtnClick = () => {
  AppIns.formRef.current.resetFields();
}

/// 列表分页大小变化
const onGridPageChange = (page) => {
  AppIns.setPagination({ ...AppIns.pagination, pageSize: page.pageSize });
}

const onQueryTreeData = () => {
  BmbpHttp.post(OrganApi.queryTreeUrl, {}).then((resp) => {
    if (resp.code == 0) {
      AppIns.setOrganTreeData(resp.data);
    } else {
      arco.Message.error(resp.msg);
    }
  });
}

const onQueryPageData = (queryParams) => {
  queryParams = queryParams || {}
  queryParams.pageNo = AppIns.pagination.current;
  queryParams.pageSize = AppIns.pagination.pageSize;
  BmbpHttp.post(OrganApi.queryPageUrl, queryParams).then((resp) => {
    if (resp.code == 0) {
      let respData = resp.data;
      AppIns.setPagination({ ...AppIns.pagination, total: respData.rowTotal });
      AppIns.setOrganGridData(respData.data);
    } else {
      arco.Message.error(resp.msg);
    }
  });
}


const saveOrganInfo = (formData, set_model) => {
  BmbpHttp.post(OrganApi.saveUrl, formData).then((resp) => {
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
