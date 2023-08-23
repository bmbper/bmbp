const AppIns = {};
const OrganView = () => {
  //组织树数据
  const [organTreeData, setOrganTreeData] = React.useState([]);
  AppIns.organTreeData = organTreeData;
  AppIns.setOrganTreeData = setOrganTreeData;

  const [currentOrganCode, setCurrentOrganCode] = React.useState("");
  AppIns.currentOrganCode = currentOrganCode;
  AppIns.setCurrentOrganCode = setCurrentOrganCode;

  /// 组织列表数据
  const [organGridData, setOrganGridData] = React.useState([]);
  AppIns.organGridData = organGridData;
  AppIns.setOrganGridData = setOrganGridData;
  /// 组织分页数据
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

  /// 组织列表多选数据
  const [selectedRowKeys, setSelectedRowKeys] = React.useState([]);
  AppIns.selectedRowKeys = selectedRowKeys;
  AppIns.setSelectedRowKeys = setSelectedRowKeys;

  /// 组织新增窗口显示
  const [organFromDailogTitle, setOrganFromDailogTitle] = React.useState("");
  AppIns.organFromDailogTitle = organFromDailogTitle;
  AppIns.setOrganFromDailogTitle = setOrganFromDailogTitle;

  /// 组织新增窗口显示
  const [initOrganValue, setInitOrganValue] = React.useState({});
  AppIns.initOrganValue = initOrganValue;
  AppIns.setInitOrganValue = setInitOrganValue;

  const [addOrganFormShow, setAddOrganFormShow] = React.useState(false);
  AppIns.addOrganFormShow = addOrganFormShow;
  AppIns.setAddOrganFormShow = setAddOrganFormShow;

  /// 组织编辑窗口显示
  const [editOrganFormShow, setEditOrganFormShow] = React.useState(false);
  AppIns.editOrganFormShow = editOrganFormShow;
  AppIns.setEditOrganFormShow = setEditOrganFormShow;

  /// 组织配置窗口
  const [configOrganFormShow, setConfigOrganFormShow] = React.useState(false);
  AppIns.configOrganFormShow = configOrganFormShow;
  AppIns.setConfigOrganFormShow = setConfigOrganFormShow;

  /// 组织详情窗口显示
  const [infoOrganFormShow, setInfoOrganFormShow] = React.useState(false);
  AppIns.infoOrganFormShow = infoOrganFormShow;
  AppIns.setInfoOrganFormShow = setInfoOrganFormShow;

  // 组织新增、编辑表单
  const [organFormRef, setOrganFormRef] = React.useState(React.useRef());
  AppIns.organFormRef = organFormRef;
  AppIns.setOrganFormRef = setOrganFormRef;

  // 组织配置表单
  const organConfigRef = React.useRef();
  AppIns.organConfigRef = organConfigRef;

  // 组织详情表单
  const organFromInfoRef = React.useRef();
  AppIns.organFromInfoRef = organFromInfoRef;


  React.useEffect(() => {
    onQueryTreeData();
    onQueryPageData();
  }, []);
  return <OrganPage />
}

const OrganPage = () => {
  return <div className="bmbp-page-tree-grid-body">
    <OrganTreeLeft />
    <OrganGridRight />
    <AddRootOrganDialog title={AppIns.organFromDailogTitle} visible={AppIns.addOrganFormShow} />
  </div>;
}
const OrganTreeLeft = () => {
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
      <div style={titleStyle}><span>组织机构</span></div>
      <div style={addStyle}><arco.Button icon={<arcoicon.IconPlus style={{ color: '#165dff' }} />} onClick={() => onAddRootOrgan()} /></div>
      <div style={refreshStyle}><arco.Button icon={<arcoicon.IconRefresh style={{ color: '#165dff' }} />} onClick={() => onRefreshOrganTree()} /></div>
    </div>
    <div style={{ display: 'block', padding: '5px 2px' }}>
      <arco.Input.Search style={{ background: '#FFFFFF' }} />
    </div>
    <arco.Tree showLine blockNode onSelect={(keys, ext) => { onOrganTreeNodeClick(ext.node.props.dataRef) }}
      renderExtra={(node) => buildTreeNodeActionBar(node)}
    > {
        buildTreeData(AppIns.organTreeData)
      } </arco.Tree>
  </div>;
}
const buildTreeNodeActionBar = (node) => {
  return <div>
    <arco.Popover
      trigger='hover' position='right'
      content={
        <div className="bmbp-action-more">
          <arco.Button size={'mini'} onClick={() => onAddOrganChild(node.dataRef)}>新增</arco.Button>
          <arco.Button size={'mini'} onClick={() => onEditOrganInfo(node.dataRef)}>配置</arco.Button>
          <arco.Button size={'mini'} onClick={() => onEditOrgan(node.dataRef)}>编辑</arco.Button>
          <arco.Button size={'mini'} onClick={() => onInfoOrgan(node.dataRef)}>查看</arco.Button>
          <arco.Button size={'mini'} onClick={() => onChangeOrganParent(node.dataRef)}>变更上级</arco.Button>
          {node.recordStatus == '0' ? <arco.Button size={'mini'} onClick={() => onDisableOrgan(node.dataRef)}>停用</arco.Button> : <arco.Button size={'mini'} onClick={() => onEnableOrgan(node.dataRef)}>启用</arco.Button>}
          {node.childrenData && node.childrenData.length > 0 ? null : <arco.Popconfirm focusLock title='删除确认' content='数据删除之后，无法恢复，是否继续?' onOk={() => { onDeleteOrgan(node.dataRef) }}><arco.Button size={'mini'}>删除</arco.Button></arco.Popconfirm>
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
  let organTreeData = treeData || [];
  if (!organTreeData || organTreeData.length == 0) {
    return null;
  }
  return organTreeData.map((item) => {
    const { organChildren, organCode, organTitle, ...rest } = item;
    return (
      <arco.Tree.Node key={organCode} title={organTitle} {...rest} dataRef={item}>
        {organChildren ? buildTreeData(item.organChildren) : null}
      </arco.Tree.Node>
    );
  });
}


const OrganGridRight = () => {

  return <div className="bmbp-page-tree-grid-grid">
    <div>
      <div className="bmbp-page-serach-form">
        <SearchForm />
      </div>
      <div className="bmbp-page-serach-toolbar">
        <div className="bmbp-page-serach-toolbar-left">
          <arco.Button type='primary' onClick={() => { onAddRootOrgan() }}>新增</arco.Button>
          {
            AppIns.selectedRowKeys && AppIns.selectedRowKeys.length > 0 ? <arco.Popconfirm focusLock title='删除确认' content='数据删除之后，无法恢复，是否继续?' onOk={() => { onBatchDeleteOrgan(AppIns.selectedRowKeys) }}>
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
          <arco.Form.Item field="organTitle" label='组织名称'>
            <arco.Input placeholder='请输入组织名称' />
          </arco.Form.Item>
        </arco.Grid.Col>
        <arco.Grid.Col span={7}>
          <arco.Form.Item field="organType" label='组织类型' allowClear>
            <arco.Select placeholder='请选择组织类型'>
              <arco.Select.Option key={'units'} value={'units'}>分组</arco.Select.Option>
              <arco.Select.Option key={'unit'} value={'unit'}>单位</arco.Select.Option>
              <arco.Select.Option key={'dept'} value={'dept'}>部门</arco.Select.Option>
              <arco.Select.Option key={'post'} value={'post'}>岗位</arco.Select.Option>
              <arco.Select.Option key={'person'} value={'person'}>人员</arco.Select.Option>
            </arco.Select>
          </arco.Form.Item>
        </arco.Grid.Col>
        <arco.Grid.Col span={7}>
          <arco.Form.Item field="recordStatus" label='组织状态' allowClear>
            <arco.Select placeholder='请选择组织状态'>
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
  }, [AppIns.currentOrganCode]);
  const columns = [
    {
      title: '组织名称',
      dataIndex: 'organTitle',
    },
    {
      title: '组织类型',
      dataIndex: 'organType',
      width: 120,
      render: (_, record) => {
        switch (record.organType) {
          case 'units':
            return <arco.Tag>分组</arco.Tag>
          case 'unit':
            return <arco.Tag>单位</arco.Tag>
          case 'dept':
            return <arco.Tag>部门</arco.Tag>
          case 'post':
            return <arco.Tag>岗位</arco.Tag>
          case 'person':
            return <arco.Tag>人员</arco.Tag>
          default:
            return <arco.Tag>未知</arco.Tag>
        }

      }
    },
    {
      title: '组织路径',
      dataIndex: 'organTitlePath',
    },
    {
      title: '组织状态',
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
          <arco.Button type='text' size={'mini'} onClick={() => onAddOrganChild(record)}>新增</arco.Button>
          <arco.Button type='text' size={'mini'} onClick={() => onEditOrganInfo(record)}>配置</arco.Button>
          <arco.Popover
            trigger='hover' position='left'
            content={
              <div className="bmbp-action-more">
                <arco.Button size={'mini'} onClick={() => onEditOrgan(record)}>编辑</arco.Button>
                <arco.Button size={'mini'} onClick={() => onInfoOrgan(record)}>查看</arco.Button>
                <arco.Button size={'mini'} onClick={() => onChangeOrganParent(record)}>变更上级</arco.Button>
                {record.recordStatus == '0' ? <arco.Button size={'mini'} onClick={() => onDisableOrgan(record)}>停用</arco.Button> : <arco.Button size={'mini'} onClick={() => onEnableOrgan(record)}>启用</arco.Button>}
                <arco.Popconfirm focusLock title='删除确认' content='数据删除之后，无法恢复，是否继续?' onOk={() => { onDeleteOrgan(record) }}>
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
    rowKey={'recordId'} columns={columns} data={AppIns.organGridData} pagination={AppIns.pagination} onChange={onGridPageChange} />;
}
