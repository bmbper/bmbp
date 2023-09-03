// 页面上下文，用于页面全局操作
// 作用相当于Redux的简化版
const PageContext = {};
// 页面视图
const PageView = () => {
  //左侧树数据
  const [leftTreeData, setLeftTreeData] = React.useState([]);
  PageContext.leftTreeData = leftTreeData;
  PageContext.setLeftTreeData = setLeftTreeData;
  // 左侧树选中节点数据
  const [leftTreeSelectNode, setLeftTreeSelectNode] = React.useState({});
  PageContext.leftTreeSelectNode = leftTreeSelectNode;
  PageContext.setLeftTreeSelectNode = setLeftTreeSelectNode;
  /// 右侧列表数据
  const [rightGridData, setRightGridData] = React.useState([]);
  PageContext.rightGridData = rightGridData;
  PageContext.setRightGridData = setRightGridData;
  // 右侧查询表单
  const [searchFormRef, setSearchFormRef] = React.useState(React.useRef());
  PageContext.searchFormRef = searchFormRef;
  PageContext.setSearchFormRef = setSearchFormRef;
  /// 右侧列表多选数据
  const [gridSelectedKeys, setGridSelectedKeys] = React.useState([]);
  PageContext.gridSelectedKeys = gridSelectedKeys;
  PageContext.setGridSelectedKeys = setGridSelectedKeys;
  /// 分页配置
  const [pageConfig, setPageConfig] = React.useState({
    sizeCanChange: true,
    showTotal: true,
    total: 0,
    pageSize: 10,
    current: 1,
    pageSizeChangeResetCurrent: true,
  });
  PageContext.pageConfig = pageConfig;
  PageContext.setPageConfig = setPageConfig;

  /// 窗口标题
  const [formTitle, setFormTitle] = React.useState("");
  PageContext.formTitle = formTitle;
  PageContext.setFormTitle = setFormTitle;

  /// 表单数据
  const [initFormValue, setInitFormValue] = React.useState({});
  PageContext.initFormValue = initFormValue;
  PageContext.setInitFormValue = setInitFormValue;

  // 新增表单
  const [addFormShow, setAddFormShow] = React.useState(false);
  PageContext.addFormShow = addFormShow;
  PageContext.setAddFormShow = setAddFormShow;
  const [addFormRef, setAddFormRef] = React.useState(React.useRef());
  PageContext.addFormRef = addFormRef;
  PageContext.setAddFormRef = setAddFormRef;
  /// 编辑表单
  const [editFormShow, setEditFormShow] = React.useState(false);
  PageContext.editFormShow = editFormShow;
  PageContext.setEditFormShow = setEditFormShow;
  const [editFormRef, setEditFormRef] = React.useState(React.useRef());
  PageContext.editFormRef = editFormRef;
  PageContext.setEditFormRef = setEditFormRef;
  /// 详情表单
  const [infoFormShow, setInfoFormShow] = React.useState(false);
  PageContext.infoFormShow = infoFormShow;
  PageContext.setInfoFormShow = setInfoFormShow;
  const [infoFormRef, setInfoFormRef] = React.useState(React.useRef());
  PageContext.infoFormRef = infoFormRef;
  PageContext.setInfoFormRef = setInfoFormRef;
  /// 配置表单
  const [configFormShow, setConfigFormShow] = React.useState(false);
  PageContext.configFormShow = configFormShow;
  PageContext.setConfigFormShow = setConfigFormShow;
  const [configFormRef, setConfigFormRef] = React.useState(React.useRef());
  PageContext.configFormRef = configFormRef;
  PageContext.setConfigFormRef = setConfigFormRef;

  /// 变更组织-弹出树
  const [changeOrganShow, setChangeOrganShow] = React.useState(false);
  PageContext.changeOrganShow = changeOrganShow;
  PageContext.setChangeOrganShow = setChangeOrganShow;
  const [changeOrganTreeRef, setChangeOrganTreeRef] = React.useState(React.useRef());
  PageContext.changeOrganTreeRef = changeOrganTreeRef;
  PageContext.setChangeOrganTreeRef = setChangeOrganTreeRef;
  const [changeOrganTreeData, setChangeOrganTreeData] = React.useState([]);
  PageContext.changeOrganTreeData = changeOrganTreeData;
  PageContext.setChangeOrganTreeData = setChangeOrganTreeData;

  React.useEffect(() => {
    // 查询左侧树
    onQueryLeftTreeData();
    onQueryRightGridData();
  }, []);
  return <TreeGridPage />
}

const TreeGridPage = () => {
  return <div className="bmbp-page-tree-grid-body">
    <LeftTreePanel />
    <RightGridPanel />
    <AddFormDialog title={PageContext.formTitle} visible={PageContext.addFormShow} />
    <EditFormDialog title={PageContext.formTitle} visible={PageContext.editFormShow} />
    <InfoFormDialog title={PageContext.formTitle} visible={PageContext.infoFormShow} />
    <ConfigFormDialog title={PageContext.formTitle} visible={PageContext.configFormShow} />
    <ChangeOrganDialog title={PageContext.formTitle} visible={PageContext.changeOrganShow} />
  </div>;
}
const LeftTreePanel = () => {
  const titleStyle = {
    with: '100%', lineHeight: '32px', textAlign: 'left', height: '32px', padding: '2px 5px',
    fontWeight: '500'
  };
  const refreshStyle = {
    position: 'absolute',
    top: '0px',
    right: '0px',
    width: '36px',
    height: '36px',
    lineHeight: '36px',
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'center',
  }
  return <div className="bmbp-page-tree-grid-tree">
    <div className="bmbp-page-tree-grid-tree-title">
      <div style={titleStyle}><span>组织机构</span></div>
      <div style={refreshStyle}><arco.Button icon={<arcoicon.IconRefresh style={{ color: '#165dff' }} />} onClick={() => onRefreshLeftTreeData()} /></div>
    </div>
    <div style={{ display: 'block', padding: '5px 2px' }}>
      <arco.Input.Search style={{ background: '#FFFFFF' }} />
    </div>
    <arco.Tree showLine blockNode onSelect={(_, ext) => { onLeftTreeNodeClick(ext.node.props.dataRef) }}
    > {
        buildLeftTreeNode(PageContext.leftTreeData)
      } </arco.Tree>
  </div>;
}
const buildLeftTreeNode = (treeData) => {
  let tempData = treeData || [];
  if (!tempData || tempData.length == 0) {
    return null;
  }

  return tempData.map((item) => {
    const { organChildren, organCode, organTitle, ...rest } = item;
    return (
      <arco.Tree.Node key={organCode} title={organTitle} {...rest} dataRef={item}>
        {organChildren ? buildLeftTreeNode(item.organChildren) : null}
      </arco.Tree.Node>
    );
  });
}


const RightGridPanel = () => {
  return <div className="bmbp-page-tree-grid-grid">
    <div>
      <div className="bmbp-page-serach-form">
        <SearchForm />
      </div>
      <div className="bmbp-page-serach-toolbar">
        <div className="bmbp-page-serach-toolbar-left">
          {
            PageContext.leftTreeSelectNode.recordId ? <arco.Button type='primary' onClick={() => { onAddForm() }}>新增</arco.Button> : null
          }
          {
            PageContext.gridSelectedKeys && PageContext.gridSelectedKeys.length > 0 ? <arco.Popconfirm focusLock title='删除确认' content='数据删除之后，无法恢复，是否继续?' onOk={() => { onBatchDelete(PageContext.gridSelectedKeys) }}>
              <arco.Button type='secondary' >删除</arco.Button>
            </arco.Popconfirm> : null
          }

        </div>
        <div className="bmbp-page-serach-toolbar-right">
          <arco.Button type='secondary' icon={<arcoicon.IconImport />} onClick={() => { onImportEvent() }}></arco.Button>
          <arco.Button type='secondary' icon={<arcoicon.IconExport />} onClick={() => { onExportEvent() }}></arco.Button>
          <arco.Button type='secondary' icon={<arcoicon.IconPrinter />} onClick={() => { onPrintEvent() }}></arco.Button>
        </div>
      </div>
      <div className="bmbp-page-serach-grid">
        <GridTable />
      </div>
    </div>
  </div>;
}


const GridTable = () => {
  React.useEffect(() => {
    onQueryRightGridData();
  }, [PageContext.leftTreeSelectNode, PageContext.pageConfig.current, PageContext.pageConfig.pageSize]);
  const gridColumns = [
    {
      title: '用户名称',
      dataIndex: 'userName',
    },
    {
      title: '显示名称',
      dataIndex: 'userNickName',
    },
    {
      title: '所属组织',
      dataIndex: 'organTitlePath',
    },
    {
      title: '用户状态',
      dataIndex: 'recordStatus',
      width: 120,
      render: (_, record) => {
        if (record.recordStatus == '0') {
          return <arco.Tag style={{ color: '#165dff' }}>正常</arco.Tag>
        }
        if (record.recordStatus == '-1') {
          return <arco.Tag style={{ color: '#7bc616' }}> 已停用</arco.Tag >
        }
      }
    },
    {
      title: '操作',
      dataIndex: 'op',
      width: '120px',
      render: (_, record) => {
        return <div className="bmbp-grid-row-action">
          <arco.Button type='text' size={'mini'} onClick={() => onEditForm(record)}>编辑</arco.Button>
          <arco.Button type='text' size={'mini'} onClick={() => onConfigForm(record)}>配置</arco.Button>
          <arco.Popover
            trigger='hover' position='left'
            content={
              <div className="bmbp-action-more">
                <arco.Button size={'mini'} onClick={() => onInfoForm(record)}>查看</arco.Button>
                <arco.Button size={'mini'} onClick={() => onResetPasswordEvent(record)}>重置密码</arco.Button>
                <arco.Button size={'mini'} onClick={() => onChangeOrganEvent(record)}>变更组织</arco.Button>
                {record.recordStatus == '0' ? <arco.Button size={'mini'} onClick={() => onDisableEvent(record)}>停用</arco.Button> : <arco.Button size={'mini'} onClick={() => onEnableEvent(record)}>启用</arco.Button>}
                <arco.Popconfirm focusLock title='删除确认' content='数据删除之后，无法恢复，是否继续?' onOk={() => { onDeleteEvent(record) }}>
                  <arco.Button size={'mini'} status='danger'>删除</arco.Button>
                </arco.Popconfirm>
              </div>
            }
          >
            <arco.Button type='text' size={'mini'} icon={<arcoicon.IconMore />}></arco.Button>
          </arco.Popover>

        </div>
      }
    },
  ];

  return <arco.Table
    rowSelection={{
      type: 'checkbox', checkAll: true, fixed: true, selectedRowKeys: PageContext.gridSelectedKeys,
      onChange: (selectedRowKeys, _) => {
        PageContext.setGridSelectedKeys(selectedRowKeys);
      },
    }}
    rowKey={'recordId'} columns={gridColumns} data={PageContext.rightGridData} pagination={PageContext.pageConfig} onChange={() => { onGridPageConfigChange() }} />;
}
