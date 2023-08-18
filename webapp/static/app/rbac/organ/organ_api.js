const OrganApi = {
  queryTreeUrl: '/rbac/v1/organ/find/tree',
  queryPageUrl: '/rbac/v1/organ/find/page',
  saveUrl: '/rbac/v1/organ/save'
}

const onAddRootOrgan = () => {
  AppIns.setAddRootOrganFormShow(true);
}
const onRefreshOrganTree = () => {
  arco.Message.info("树刷新事件");
}
const onOrganTreeNodeClick = (organ) => {
  arco.Message.info("组织节点点击");
}
const onChangeOrganParent = (organ) => {
  arco.Message.info("变更组织上级");
}
const onAddOrganChild = (organ) => {
  arco.Message.info("增加下级节点");
}
const onEditOrgan = (organ) => {
  arco.Message.info("编辑组织节点");
}
const onInfoOrgan = (organ) => {
  arco.Message.info("查看组织节点");
}
const onEnableOrgan = (organ) => {
  arco.Message.info("启用组织节点");
}

const onDisableOrgan = (organ) => {
  arco.Message.info("停用组织节点");
}

const onDeleteOrgan = (organ) => {
  arco.Message.info("删除组织节点");
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
