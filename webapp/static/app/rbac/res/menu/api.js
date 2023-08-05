const onAddRootOrgan = () => {
  arco.Message.info("增加根节点");
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

const onQueryGridData = (queryData) => {
  queryData = queryData || {}
  let data = [
    {
      recordId: '1',
      organTitle: '集团公司',
      organTitlePath: '/集团公司/',
      organType: 'unit',
      recordStatus: '1',
    },
    {
      recordId: '2',
      organTitle: '部门',
      organTitlePath: '/集团公司/部门',
      organType: 'dept',
      recordStatus: '1',
    },
    {
      recordId: '3',
      organTitle: '部门2',
      organTitlePath: '/集团公司/部门2',
      organType: 'dept',
      recordStatus: '0',
    },
    {
      recordId: '4',
      organTitle: '部门4',
      organTitlePath: '/集团公司/部门4',
      organType: 'dept',
      recordStatus: '0',
    },
    {
      recordId: '5',
      organTitle: '员工',
      organTitlePath: '/集团公司/部门2/员工',
      organType: 'post',
      recordStatus: '0',
    }
  ];
  AppIns.setPagination({ ...AppIns.pagination, total: data.length });
  AppIns.setGridData(data);
  arco.Message.info("查询表结构数据：" + JSON.stringify(queryData));
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
  onQueryGridData(queryData);
}

const onSearchFormRestBtnClick = () => {
  AppIns.formRef.current.resetFields();
}

/// 列表分页大小变化
const onGridPageChange = (page) => {
  AppIns.setPagination({ ...AppIns.pagination, pageSize: page.pageSize });
}

const queryTreeData = () => {
  return [{ organTitle: '配置中心', organCode: '1', resType: 'app', children: [{ organTitle: '部门1', organCode: '1.1' }, { organTitle: '部门3', organCode: '1.2' }] }]
}
