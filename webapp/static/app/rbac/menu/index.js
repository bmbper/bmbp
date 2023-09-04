const AppIns = {};
const PageView = () => {
  //菜单树数据
  const [menuTreeData, setMenuTreeData] = React.useState([]);
  AppIns.menuTreeData = menuTreeData;
  AppIns.setMenuTreeData = setMenuTreeData;

  const [currentMenuCode, setCurrentMenuCode] = React.useState("");
  AppIns.currentMenuCode = currentMenuCode;
  AppIns.setCurrentMenuCode = setCurrentMenuCode;

  /// 菜单列表数据
  const [menuGridData, setMenuGridData] = React.useState([]);
  AppIns.menuGridData = menuGridData;
  AppIns.setMenuGridData = setMenuGridData;
  /// 菜单分页数据
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

  /// 菜单列表多选数据
  const [selectedRowKeys, setSelectedRowKeys] = React.useState([]);
  AppIns.selectedRowKeys = selectedRowKeys;
  AppIns.setSelectedRowKeys = setSelectedRowKeys;

  /// 菜单新增窗口显示
  const [menuFromDailogTitle, setMenuFromDailogTitle] = React.useState("");
  AppIns.menuFromDailogTitle = menuFromDailogTitle;
  AppIns.setMenuFromDailogTitle = setMenuFromDailogTitle;

  /// 菜单新增窗口显示
  const [initMenuValue, setInitMenuValue] = React.useState({});
  AppIns.initMenuValue = initMenuValue;
  AppIns.setInitMenuValue = setInitMenuValue;

  const [addMenuFormShow, setAddMenuFormShow] = React.useState(false);
  AppIns.addMenuFormShow = addMenuFormShow;
  AppIns.setAddMenuFormShow = setAddMenuFormShow;

  /// 菜单编辑窗口显示
  const [editMenuFormShow, setEditMenuFormShow] = React.useState(false);
  AppIns.editMenuFormShow = editMenuFormShow;
  AppIns.setEditMenuFormShow = setEditMenuFormShow;

  /// 菜单配置窗口
  const [configMenuFormShow, setConfigMenuFormShow] = React.useState(false);
  AppIns.configMenuFormShow = configMenuFormShow;
  AppIns.setConfigMenuFormShow = setConfigMenuFormShow;

  /// 菜单详情窗口显示
  const [infoMenuFormShow, setInfoMenuFormShow] = React.useState(false);
  AppIns.infoMenuFormShow = infoMenuFormShow;
  AppIns.setInfoMenuFormShow = setInfoMenuFormShow;
  /// 菜单选择框显示
  const [changeParentMenuShow, setChangeParentMenuShow] = React.useState(false);
  AppIns.changeParentMenuShow = changeParentMenuShow;
  AppIns.setChangeParentMenuShow = setChangeParentMenuShow;
  /// 菜单弹窗选择树
  const [parentMenuTreeRef, setParentMenuTreeRef] = React.useState(React.useRef());
  AppIns.parentMenuTreeRef = parentMenuTreeRef;
  AppIns.setParentMenuTreeRef = setParentMenuTreeRef;
  const [treeParentData, setTreeParentData] = React.useState([]);
  AppIns.treeParentData = treeParentData;
  AppIns.setTreeParentData = setTreeParentData;

  // 菜单新增、编辑表单
  const [menuFormRef, setMenuFormRef] = React.useState(React.useRef());
  AppIns.menuFormRef = menuFormRef;
  AppIns.setMenuFormRef = setMenuFormRef;

  const [menuEditFormRef, setMenuEditFormRef] = React.useState(React.useRef());
  AppIns.menuEditFormRef = menuEditFormRef;
  AppIns.setMenuEditFormRef = setMenuEditFormRef;

  const [menuInfoFormRef, setMenuInfoFormRef] = React.useState(React.useRef());
  AppIns.menuInfoFormRef = menuInfoFormRef;
  AppIns.setMenuInfoFormRef = setMenuInfoFormRef;



  // 菜单配置表单
  const menuConfigRef = React.useRef();
  AppIns.menuConfigRef = menuConfigRef;

  // 菜单详情表单
  const menuFromInfoRef = React.useRef();
  AppIns.menuFromInfoRef = menuFromInfoRef;


  React.useEffect(() => {
    onQueryTreeData();
    onQueryPageData();
  }, []);
  return <MenuPage />
}

const MenuPage = () => {
  return <div className="bmbp-page-tree-grid-body">
    <MenuTreeLeft />
    <MenuGridRight />
    <AddMenuDialog title={AppIns.menuFromDailogTitle} visible={AppIns.addMenuFormShow} />
    <EditMenuDialog title={AppIns.menuFromDailogTitle} visible={AppIns.editMenuFormShow} />
    <InfoMenuDialog title={AppIns.menuFromDailogTitle} visible={AppIns.infoMenuFormShow} />
    <ChangeParentMenuDialog title={AppIns.menuFromDailogTitle} visible={AppIns.changeParentMenuShow} />
  </div>;
}
const MenuTreeLeft = () => {
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
      <div style={titleStyle}><span>应用菜单</span></div>
      <div style={addStyle}><arco.Button icon={<arcoicon.IconPlus style={{ color: '#165dff' }} />} onClick={() => onAddRootMenu()} /></div>
      <div style={refreshStyle}><arco.Button icon={<arcoicon.IconRefresh style={{ color: '#165dff' }} />} onClick={() => onRefreshMenuTree()} /></div>
    </div>
    <div style={{ display: 'block', padding: '5px 2px' }}>
      <arco.Input.Search style={{ background: '#FFFFFF' }} />
    </div>
    <arco.Tree showLine blockNode onSelect={(keys, ext) => { onMenuTreeNodeClick(ext.node.props.dataRef) }}
      renderExtra={(node) => buildTreeNodeActionBar(node)}
    > {
        buildTreeData(AppIns.menuTreeData)
      } </arco.Tree>
  </div>;
}
const buildTreeNodeActionBar = (node) => {
  return <div>
    <arco.Popover
      trigger='hover' position='right'
      content={
        <div className="bmbp-action-more">
          <arco.Button size={'mini'} onClick={() => onAddMenuChild(node.dataRef)}>新增</arco.Button>
          <arco.Button size={'mini'} onClick={() => onEditMenu(node.dataRef)}>编辑</arco.Button>
          <arco.Button size={'mini'} onClick={() => onInfoMenu(node.dataRef)}>查看</arco.Button>
          <arco.Button size={'mini'} onClick={() => onChangeMenuParent(node.dataRef)}>变更上级</arco.Button>
          {node.recordStatus == '0' ? <arco.Button size={'mini'} onClick={() => onDisableMenu(node.dataRef)}>停用</arco.Button> : <arco.Button size={'mini'} onClick={() => onEnableMenu(node.dataRef)}>启用</arco.Button>}
          {node.childrenData && node.childrenData.length > 0 ? null : <arco.Popconfirm focusLock title='删除确认' content='数据删除之后，无法恢复，是否继续?' onOk={() => { onDeleteMenu(node.dataRef) }}><arco.Button size={'mini'}>删除</arco.Button></arco.Popconfirm>
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
  let menuTreeData = treeData || [];
  if (!menuTreeData || menuTreeData.length == 0) {
    return null;
  }
  return menuTreeData.map((item) => {
    const { menuChildren, menuCode, menuTitle, ...rest } = item;
    return (
      <arco.Tree.Node key={menuCode} title={menuTitle} {...rest} dataRef={item}>
        {menuChildren ? buildTreeData(item.menuChildren) : null}
      </arco.Tree.Node>
    );
  });
}


const MenuGridRight = () => {

  return <div className="bmbp-page-tree-grid-grid">
    <div>
      <div className="bmbp-page-serach-form">
        <SearchForm />
      </div>
      <div className="bmbp-page-serach-toolbar">
        <div className="bmbp-page-serach-toolbar-left">
          <arco.Button type='primary' onClick={() => { onAddRootMenu() }}>新增</arco.Button>
          {
            AppIns.selectedRowKeys && AppIns.selectedRowKeys.length > 0 ? <arco.Popconfirm focusLock title='删除确认' content='数据删除之后，无法恢复，是否继续?' onOk={() => { onBatchDeleteMenu(AppIns.selectedRowKeys) }}>
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
          <arco.Form.Item field="menuTitle" label='菜单名称'>
            <arco.Input placeholder='请输入菜单名称' />
          </arco.Form.Item>
        </arco.Grid.Col>
        <arco.Grid.Col span={7}>
          <arco.Form.Item field="menuUrl" label='菜单路径'>
            <arco.Input placeholder='请输入菜单路径' />
          </arco.Form.Item>
        </arco.Grid.Col>
        <arco.Grid.Col span={7}>
          <arco.Form.Item field="recordStatus" label='菜单状态' allowClear>
            <arco.Select placeholder='请选择菜单状态'>
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
  }, [AppIns.currentMenuCode]);
  const columns = [
    {
      title: '菜单名称',
      dataIndex: 'menuTitle',
    },
    {
      title: '菜单类型',
      dataIndex: 'menuType',
      width: 120,
      render: (_, record) => {
        switch (record.menuType) {
          case 'module':
            return <arco.Tag>模块</arco.Tag>
          case 'func':
            return <arco.Tag>功能</arco.Tag>
          default:
            return <arco.Tag>未知</arco.Tag>
        }

      }
    },
    {
      title: '菜单路径',
      dataIndex: 'menuTitlePath',
    },
    {
      title: '菜单状态',
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
          <arco.Button type='text' size={'mini'} onClick={() => onAddMenuChild(record)}>新增</arco.Button>
          <arco.Button type='text' size={'mini'} onClick={() => onEditMenu(record)}>编辑</arco.Button>
          <arco.Popover
            trigger='hover' position='left'
            content={
              <div className="bmbp-action-more">
                <arco.Button size={'mini'} onClick={() => onEditMenu(record)}>编辑</arco.Button>
                <arco.Button size={'mini'} onClick={() => onInfoMenu(record)}>查看</arco.Button>
                <arco.Button size={'mini'} onClick={() => onChangeMenuParent(record)}>变更上级</arco.Button>
                {record.recordStatus == '0' ? <arco.Button size={'mini'} onClick={() => onDisableMenu(record)}>停用</arco.Button> : <arco.Button size={'mini'} onClick={() => onEnableMenu(record)}>启用</arco.Button>}
                <arco.Popconfirm focusLock title='删除确认' content='数据删除之后，无法恢复，是否继续?' onOk={() => { onDeleteMenu(record) }}>
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
    rowKey={'recordId'} columns={columns} data={AppIns.menuGridData} pagination={AppIns.pagination} onChange={onGridPageChange} />;
}
