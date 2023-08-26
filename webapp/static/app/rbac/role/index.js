const AppIns = {};
const PageView = () => {
  //角色树数据
  const [roleTreeData, setRoleTreeData] = React.useState([]);
  AppIns.roleTreeData = roleTreeData;
  AppIns.setRoleTreeData = setRoleTreeData;

  const [currentRoleCode, setCurrentRoleCode] = React.useState("");
  AppIns.currentRoleCode = currentRoleCode;
  AppIns.setCurrentRoleCode = setCurrentRoleCode;

  /// 角色列表数据
  const [roleGridData, setRoleGridData] = React.useState([]);
  AppIns.roleGridData = roleGridData;
  AppIns.setRoleGridData = setRoleGridData;
  /// 角色分页数据
  const [pagination, setPagination] = React.useState({
    sizeCanChange: true,
    showTotal: true,
    total: 0,
    pageSize: 10,
    current: 1,
    pageSizeChangeResetCurrent: true,
  });
  AppIns.setPagination = setPagination;
  AppIns.pagination = pagination;

  /// 角色列表多选数据
  const [selectedRowKeys, setSelectedRowKeys] = React.useState([]);
  AppIns.selectedRowKeys = selectedRowKeys;
  AppIns.setSelectedRowKeys = setSelectedRowKeys;

  /// 角色新增窗口显示
  const [roleFromDailogTitle, setRoleFromDailogTitle] = React.useState("");
  AppIns.roleFromDailogTitle = roleFromDailogTitle;
  AppIns.setRoleFromDailogTitle = setRoleFromDailogTitle;

  /// 角色新增窗口显示
  const [initRoleValue, setInitRoleValue] = React.useState({});
  AppIns.initRoleValue = initRoleValue;
  AppIns.setInitRoleValue = setInitRoleValue;

  const [addRoleFormShow, setAddRoleFormShow] = React.useState(false);
  AppIns.addRoleFormShow = addRoleFormShow;
  AppIns.setAddRoleFormShow = setAddRoleFormShow;

  /// 角色编辑窗口显示
  const [editRoleFormShow, setEditRoleFormShow] = React.useState(false);
  AppIns.editRoleFormShow = editRoleFormShow;
  AppIns.setEditRoleFormShow = setEditRoleFormShow;

  /// 角色配置窗口
  const [configRoleFormShow, setConfigRoleFormShow] = React.useState(false);
  AppIns.configRoleFormShow = configRoleFormShow;
  AppIns.setConfigRoleFormShow = setConfigRoleFormShow;

  /// 角色详情窗口显示
  const [infoRoleFormShow, setInfoRoleFormShow] = React.useState(false);
  AppIns.infoRoleFormShow = infoRoleFormShow;
  AppIns.setInfoRoleFormShow = setInfoRoleFormShow;
  /// 角色选择框显示
  const [changeParentRoleShow, setChangeParentRoleShow] = React.useState(false);
  AppIns.changeParentRoleShow = changeParentRoleShow;
  AppIns.setChangeParentRoleShow = setChangeParentRoleShow;
  /// 角色弹窗选择树
  const [parentRoleTreeRef, setParentRoleTreeRef] = React.useState(React.useRef());
  AppIns.parentRoleTreeRef = parentRoleTreeRef;
  AppIns.setParentRoleTreeRef = setParentRoleTreeRef;
  const [treeParentData, setTreeParentData] = React.useState([]);
  AppIns.treeParentData = treeParentData;
  AppIns.setTreeParentData = setTreeParentData;

  // 角色新增、编辑表单
  const [roleFormRef, setRoleFormRef] = React.useState(React.useRef());
  AppIns.roleFormRef = roleFormRef;
  AppIns.setRoleFormRef = setRoleFormRef;

  const [roleEditFormRef, setRoleEditFormRef] = React.useState(React.useRef());
  AppIns.roleEditFormRef = roleEditFormRef;
  AppIns.setRoleEditFormRef = setRoleEditFormRef;

  const [roleInfoFormRef, setRoleInfoFormRef] = React.useState(React.useRef());
  AppIns.roleInfoFormRef = roleInfoFormRef;
  AppIns.setRoleInfoFormRef = setRoleInfoFormRef;



  // 角色配置表单
  const roleConfigRef = React.useRef();
  AppIns.roleConfigRef = roleConfigRef;

  // 角色详情表单
  const roleFromInfoRef = React.useRef();
  AppIns.roleFromInfoRef = roleFromInfoRef;


  React.useEffect(() => {
    onQueryTreeData();
    onQueryPageData();
  }, []);
  return <RolePage />
}

const RolePage = () => {
  return <div className="bmbp-page-tree-grid-body">
    <RoleTreeLeft />
    <RoleGridRight />
    <AddRootRoleDialog title={AppIns.roleFromDailogTitle} visible={AppIns.addRoleFormShow} />
    <EditRoleDialog title={AppIns.roleFromDailogTitle} visible={AppIns.editRoleFormShow} />
    <InfoRoleDialog title={AppIns.roleFromDailogTitle} visible={AppIns.infoRoleFormShow} />
    <ChangeParentRoleDialog title={AppIns.roleFromDailogTitle} visible={AppIns.changeParentRoleShow} />
  </div>;
}
const RoleTreeLeft = () => {
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
  const addStyle = {
    position: 'absolute',
    top: '0px',
    right: '36px',
    width: '36px',
    height: '36px',
    lineHeight: '36px',
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'center',
    padding: 0
  }
  return <div className="bmbp-page-tree-grid-tree">
    <div className="bmbp-page-tree-grid-tree-title">
      <div style={titleStyle}><span>角色树</span></div>
      <div style={addStyle}><arco.Button icon={<arcoicon.IconPlus style={{ color: '#165dff' }} />} onClick={() => onAddRootRole()} /></div>
      <div style={refreshStyle}><arco.Button icon={<arcoicon.IconRefresh style={{ color: '#165dff' }} />} onClick={() => onRefreshRoleTree()} /></div>
    </div>
    <div style={{ display: 'block', padding: '5px 2px' }}>
      <arco.Input.Search style={{ background: '#FFFFFF' }} />
    </div>
    <arco.Tree showLine blockNode onSelect={(keys, ext) => { onRoleTreeNodeClick(ext.node.props.dataRef) }}
      renderExtra={(node) => buildTreeNodeActionBar(node)}
    > {
        buildTreeData(AppIns.roleTreeData)
      } </arco.Tree>
  </div>;
}
const buildTreeNodeActionBar = (node) => {
  return <div>
    <arco.Popover
      trigger='hover' position='right'
      content={
        <div className="bmbp-action-more">
          <arco.Button size={'mini'} onClick={() => onAddRoleChild(node.dataRef)}>新增</arco.Button>
          <arco.Button size={'mini'} onClick={() => onEditRoleInfo(node.dataRef)}>配置</arco.Button>
          <arco.Button size={'mini'} onClick={() => onEditRole(node.dataRef)}>编辑</arco.Button>
          <arco.Button size={'mini'} onClick={() => onInfoRole(node.dataRef)}>查看</arco.Button>
          <arco.Button size={'mini'} onClick={() => onChangeRoleParent(node.dataRef)}>变更上级</arco.Button>
          {node.recordStatus == '0' ? <arco.Button size={'mini'} onClick={() => onDisableRole(node.dataRef)}>停用</arco.Button> : <arco.Button size={'mini'} onClick={() => onEnableRole(node.dataRef)}>启用</arco.Button>}
          {node.childrenData && node.childrenData.length > 0 ? null : <arco.Popconfirm focusLock title='删除确认' content='数据删除之后，无法恢复，是否继续?' onOk={() => { onDeleteRole(node.dataRef) }}><arco.Button size={'mini'}>删除</arco.Button></arco.Popconfirm>
          }
        </div>
      }>
      <arcoicon.IconMore
        style={{
          position: 'absolute',
          right: 8,
          fontSize: 12,
          top: 10,
          color: '#165dff',
        }}
      />
    </arco.Popover >
  </div >
}
const buildTreeData = (treeData) => {
  let roleTreeData = treeData || [];
  if (!roleTreeData || roleTreeData.length == 0) {
    return null;
  }
  return roleTreeData.map((item) => {
    const { roleChildren, roleCode, roleTitle, ...rest } = item;
    return (
      <arco.Tree.Node key={roleCode} title={roleTitle} {...rest} dataRef={item}>
        {roleChildren ? buildTreeData(item.roleChildren) : null}
      </arco.Tree.Node>
    );
  });
}


const RoleGridRight = () => {

  return <div className="bmbp-page-tree-grid-grid">
    <div>
      <div className="bmbp-page-serach-form">
        <SearchForm />
      </div>
      <div className="bmbp-page-serach-toolbar">
        <div className="bmbp-page-serach-toolbar-left">
          <arco.Button type='primary' onClick={() => { onAddRootRole() }}>新增</arco.Button>
          {
            AppIns.selectedRowKeys && AppIns.selectedRowKeys.length > 0 ? <arco.Popconfirm focusLock title='删除确认' content='数据删除之后，无法恢复，是否继续?' onOk={() => { onBatchDeleteRole(AppIns.selectedRowKeys) }}>
              <arco.Button type='secondary' >删除</arco.Button>
            </arco.Popconfirm> : null
          }

        </div>
        <div className="bmbp-page-serach-toolbar-right">
          <arco.Button type='secondary' icon={<arcoicon.IconImport />} onClick={() => { onToolBarImportBtnClick() }}></arco.Button>
          <arco.Button type='secondary' icon={<arcoicon.IconExport />} onClick={() => { onToolBarExportBtnClick() }}></arco.Button>
          <arco.Button type='secondary' icon={<arcoicon.IconPrinter />} onClick={() => { onToolBarPrintBtnClick() }}></arco.Button>
        </div>
      </div>
      <div className="bmbp-page-serach-grid">
        <GridTable />
      </div>
    </div>
  </div>;
}

const SearchForm = () => {
  AppIns.searchFormRef = React.useRef();

  const searchBtnStyle = {
    marginRight: "4px",
    padding: "0 5px"
  };
  const formItemLayout = {
    labelCol: {
      span: 8,
    },
    wrapperCol: {
      span: 16,
    },
    labelAlign: 'right'
  };
  return <div>
    <arco.Form ref={AppIns.searchFormRef} {...formItemLayout}>
      <arco.Grid.Row gutter={24}>
        <arco.Grid.Col span={7}>
          <arco.Form.Item field="roleTitle" label='角色名称'>
            <arco.Input placeholder='请输入角色名称' />
          </arco.Form.Item>
        </arco.Grid.Col>
        <arco.Grid.Col span={7}>
          <arco.Form.Item field="roleType" label='角色类型' allowClear>
            <arco.Select placeholder='请选择角色类型'>
              <arco.Select.Option key={'1'} value={'1'}>管理角色</arco.Select.Option>
              <arco.Select.Option key={'2'} value={'2'}>业务角色</arco.Select.Option>
            </arco.Select>
          </arco.Form.Item>
        </arco.Grid.Col>
        <arco.Grid.Col span={7}>
          <arco.Form.Item field="recordStatus" label='角色状态' allowClear>
            <arco.Select placeholder='请选择角色状态'>
              <arco.Select.Option key={'1'} value={'1'}>正常</arco.Select.Option>
              <arco.Select.Option key={'0'} value={'0'}>已停用</arco.Select.Option>
            </arco.Select>
          </arco.Form.Item>
        </arco.Grid.Col>
        <arco.Grid.Col span={3}>
          <arco.Button type='primary' style={searchBtnStyle} onClick={() => { onSearchFormQueryBtnClick() }}>查询</arco.Button>
          <arco.Button type='secondary' style={searchBtnStyle} onClick={() => { onSearchFormRestBtnClick() }}>重置</arco.Button>
        </arco.Grid.Col>
      </arco.Grid.Row>
    </arco.Form>
  </div >;
}

const GridTable = () => {
  React.useEffect(() => {
    onQueryPageData();
  }, [AppIns.currentRoleCode]);
  const columns = [
    {
      title: '角色名称',
      dataIndex: 'roleTitle',
    },
    {
      title: '角色类型',
      dataIndex: 'roleType',
      width: 120,
      render: (_, record) => {
        switch (record.roleType) {
          case '1':
            return <arco.Tag>管理角色</arco.Tag>
          case '2':
            return <arco.Tag>业务角色</arco.Tag>
          default:
            return <arco.Tag>未知</arco.Tag>
        }

      }
    },
    {
      title: '角色路径',
      dataIndex: 'roleTitlePath',
    },
    {
      title: '角色状态',
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
          <arco.Button type='text' size={'mini'} onClick={() => onAddRoleChild(record)}>新增</arco.Button>
          <arco.Button type='text' size={'mini'} onClick={() => onEditRoleInfo(record)}>配置</arco.Button>
          <arco.Popover
            trigger='hover' position='left'
            content={
              <div className="bmbp-action-more">
                <arco.Button size={'mini'} onClick={() => onEditRole(record)}>编辑</arco.Button>
                <arco.Button size={'mini'} onClick={() => onInfoRole(record)}>查看</arco.Button>
                <arco.Button size={'mini'} onClick={() => onChangeRoleParent(record)}>变更上级</arco.Button>
                {record.recordStatus == '0' ? <arco.Button size={'mini'} onClick={() => onDisableRole(record)}>停用</arco.Button> : <arco.Button size={'mini'} onClick={() => onEnableRole(record)}>启用</arco.Button>}
                <arco.Popconfirm focusLock title='删除确认' content='数据删除之后，无法恢复，是否继续?' onOk={() => { onDeleteRole(record) }}>
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
      type: 'checkbox', checkAll: true, fixed: true, selectedRowKeys: AppIns.selectedRowKeys,
      onChange: (selectedRowKeys, _) => {
        AppIns.setSelectedRowKeys(selectedRowKeys);
      },
    }}
    rowKey={'recordId'} columns={columns} data={AppIns.roleGridData} pagination={AppIns.pagination} onChange={onGridPageChange} />;
}
