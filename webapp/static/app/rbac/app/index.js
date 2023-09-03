const PageContext = {
};
function PageView() {
  /// 右侧列表数据
  const [gridData, setGridData] = React.useState([]);
  PageContext.gridData = gridData;
  PageContext.setGridData = setGridData;
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
  return <AppView />;
}

const AppView = () => {
  React.useEffect(() => {
    onQueryGridData();
  }, []);
  return <div className="bmbp-page-body">
    <div className="bmbp-page-serach-form">
      <SearchForm />
    </div>
    <div className="bmbp-page-serach-toolbar">
      <div className="bmbp-page-serach-toolbar-left">
        <arco.Button type='primary' onClick={onAddForm}>新增</arco.Button>
        {
          PageContext.selectedRowKeys && PageContext.selectedRowKeys.length > 0 ? <arco.Popconfirm focusLock title='删除确认' content='数据删除之后，无法恢复，是否继续?' onOk={() => { onToolBarDelBtnClick() }}>
            <arco.Button type='secondary' >删除</arco.Button>
          </arco.Popconfirm> : null
        }

      </div>
      <div className="bmbp-page-serach-toolbar-right">
        <arco.Button type='secondary' icon={<arcoicon.IconImport />} onClick={onImportEvent}></arco.Button>
        <arco.Button type='secondary' icon={<arcoicon.IconExport />} onClick={onExportEvent}></arco.Button>
        <arco.Button type='secondary' icon={<arcoicon.IconPrinter />} onClick={onPrintEvent}></arco.Button>
      </div>
    </div>
    <div className="bmbp-page-serach-grid">
      <GridTable />
    </div>
    <AddFormDialog title={PageContext.formTitle} visible={PageContext.addFormShow} />
    <EditFormDialog title={PageContext.formTitle} visible={PageContext.editFormShow} />
    <InfoFormDialog title={PageContext.formTitle} visible={PageContext.infoFormShow} />
    <ConfigFormDialog title={PageContext.formTitle} visible={PageContext.configFormShow} />
  </div >;
}

const GridTable = () => {
  const gridColumns = [
    {
      title: '应用编码',
      dataIndex: 'appCode',
    },
    {
      title: '应用名称',
      dataIndex: 'appTitle',
    },
    {
      title: '应用标识',
      dataIndex: 'appKey',
    },
    {
      title: '应用密钥',
      dataIndex: 'appSecrectKey',
      width: '320px',
    },
    {
      title: '应用类型',
      dataIndex: 'appType',
      width: 80,
      render: (_, record) => {
        if (record.appType == 'module') {
          return <arco.Tag style={{ color: '#165dff' }}>平台应用</arco.Tag>
        }
        if (record.recordStatus == 'sso') {
          return <arco.Tag style={{ color: '#7bc616' }}> 单点应用</arco.Tag >
        }
        if (record.recordStatus == 'link') {
          return <arco.Tag style={{ color: '#ff5722' }} > 连接应用</arco.Tag >
        }
        return <arco.Tag > 其它应用</arco.Tag >
      }
    },
    {
      title: '应用状态',
      dataIndex: 'recordStatus',
      width: 100,
      render: (_, record) => {
        if (record.recordStatus == '0') {
          return <arco.Tag color={'#165dff'} >开发中</arco.Tag>
        }
        if (record.recordStatus == '1') {
          return <arco.Tag color={'#00b42a'} > 已上线</arco.Tag >
        }
        if (record.recordStatus == '2') {
          return <arco.Tag color={'#ff5722'} > 已下线</arco.Tag >
        }
        return <arco.Tag >未知</arco.Tag >
      }
    },
    {
      title: '顺序',
      dataIndex: 'recordNum',
      width: 80,
      align: 'center'
    },
    {
      title: '操作',
      dataIndex: 'op',
      width: '120px',
      render: (_, record) => {
        return <div className="bmbp-grid-row-action">
          <arco.Button type='text' size={'mini'} onClick={() => onConfigForm(record)}>配置</arco.Button>
          <arco.Button type='text' size={'mini'} onClick={() => onInfoForm(record)}>查看</arco.Button>
          <arco.Popover
            trigger='hover' position='left'
            content={
              <div className="bmbp-action-more">
                {buildRowStatusAction(record)}
                <arco.Popconfirm focusLock title='删除确认' content='数据删除之后，无法恢复，是否继续?' onOk={() => { onDeleteEvent(record) }}>
                  <arco.Button type='text' size={'mini'} status='danger'>删除</arco.Button>
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
    rowKey={'recordId'} columns={gridColumns} data={PageContext.gridData} pagination={PageContext.pageConfig} onChange={() => { onGridPageConfigChange() }} />;
}


const buildRowStatusAction = (record) => {
  if (record.recordStatus == "0") {
    return <React.Fragment>
      <arco.Button type='text' size={'mini'} onClick={() => onEditForm(record)}>编辑</arco.Button>
      <arco.Button type='text' size={'mini'} onClick={() => onEnableEvent(record)}>上线</arco.Button>
    </React.Fragment>

  } else if (record.recordStatus == "1") {
    return <arco.Button type='text' size={'mini'} onClick={() => onDisableEvent(record)}>下线</arco.Button>;
  } else {
    return <arco.Button type='text' size={'mini'} onClick={() => onRestartEvent(record)}>重启开发</arco.Button>;
  }

}
