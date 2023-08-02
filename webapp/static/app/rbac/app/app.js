
const onToolBarAddBtnClick = () => {
  arco.Message.info("新增功能开发中...");
}
const onToolBarDelBtnClick = () => {
  arco.Message.info("删除功能开发中...");
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
  var queryData = pageData.formRef.current.getFieldsValue();
  onQueryGridData(queryData);
}

const onQueryGridData = (queryData) => {
  queryData = queryData || {}
  let data = [
    {
      recordId: '1',
      appCode: '1',
      appTitle: 'Jane Doe',
      appKey: 23000,
      appType: '32 Park Road, London',
      recordStatus: '1',
    },
    {
      recordId: '2',
      appCode: '2',
      appTitle: 'Jane Doe',
      appKey: 23000,
      appType: '32 Park Road, London',
      recordStatus: '0',
    },
    {
      recordId: '3',
      appCode: '2',
      appTitle: 'Jane Doe',
      appKey: 23000,
      appType: '32 Park Road, London',
      recordStatus: '0',
    },
    {
      recordId: '4',
      appCode: '2',
      appTitle: 'Jane Doe',
      appKey: 23000,
      appType: '32 Park Road, London',
      recordStatus: '0',
    },
    {
      recordId: '5',
      appCode: '2',
      appTitle: 'Jane Doe',
      appKey: 23000,
      appType: '32 Park Road, London',
      recordStatus: '0',
    },
    {
      recordId: '6',
      appCode: '2',
      appTitle: 'Jane Doe',
      appKey: 23000,
      appType: '32 Park Road, London',
      recordStatus: '0',
    },
  ];
  pageData.setPagination({ ...pageData.pagination, total: data.length });
  pageData.setGridData(data);

  arco.Message.info("查询表结构数据：" + JSON.stringify(queryData));
}

const onGridPageChange = (page) => {
  pageData.setPagination({ ...pageData.pagination, pageSize: page.pageSize });
}

const onSearchFormRestBtnClick = () => {
  pageData.formRef.current.resetFields();
}

const onRowEditBtnClick = (record) => {
  arco.Message.info("编辑功能开发中..." + record.appTitle);
}

const onRowConfigBtnClick = (record) => {
  arco.Message.info("配置功能开发中..." + record.appTitle);
}

const onRowInfoBtnClick = (record) => {
  arco.Message.info("查看功能开发中..." + record.appTitle);
}

const onRowDelBtnClick = (record) => {
  arco.Message.info("删除功能开发中..." + record.appTitle);
}

const onRowEnableBtnClick = (record) => {
  arco.Message.info("启用功能开发中..." + record.appTitle);
}

const onRowDisableBtnClick = (record) => {
  arco.Message.info("停用功能开发中..." + record.appTitle);
}

const pageData = {
  formRef: null,
  setPagination: null,
  pagination: null,
};

function SearchForm() {
  pageData.formRef = React.useRef();
  const searchBtnStyle = {
    marginRight: "8px",
    padding: "0 10px"
  };
  const formItemLayout = {
    labelCol: {
      span: 6,
    },
    wrapperCol: {
      span: 18,
    },
    labelAlign: 'right'
  };
  return <div>
    <arco.Form ref={pageData.formRef} {...formItemLayout}>
      <arco.Grid.Row gutter={24}>
        <arco.Grid.Col span={7}>
          <arco.Form.Item field="appCode" label='应用编码'>
            <arco.Input placeholder='请输入应用编码' />
          </arco.Form.Item>
        </arco.Grid.Col>
        <arco.Grid.Col span={7}>
          <arco.Form.Item field="appTitle" label='应用名称'>
            <arco.Input placeholder='请输入应用名称' />
          </arco.Form.Item>
        </arco.Grid.Col>
        <arco.Grid.Col span={7}>
          <arco.Form.Item field="appType" label='应用类型'>
            <arco.Select placeholder='请选择应用类型'>
              <arco.Select.Option key={'1'} value={'1'}>内置应用</arco.Select.Option>
              <arco.Select.Option key={'2'} value={'2'}>集成应用</arco.Select.Option>
            </arco.Select>
          </arco.Form.Item>
        </arco.Grid.Col>
        <arco.Grid.Col span={7}>
          <arco.Form.Item field="recordStatus" label='应用状态'>
            <arco.Select placeholder='请选择应用状态'>
              <arco.Select.Option key={'1'} value={'1'}>开发中</arco.Select.Option>
              <arco.Select.Option key={'2'} value={'2'}>已发布</arco.Select.Option>
              <arco.Select.Option key={'3'} value={'3'}>已下线</arco.Select.Option>
            </arco.Select>
          </arco.Form.Item>
        </arco.Grid.Col>
        <arco.Grid.Col span={3}>
          <arco.Button type='primary' style={searchBtnStyle} onClick={onSearchFormQueryBtnClick}>查询</arco.Button>
          <arco.Button type='secondary' style={searchBtnStyle} onClick={onSearchFormRestBtnClick}>重置</arco.Button>
        </arco.Grid.Col>
      </arco.Grid.Row>
    </arco.Form>
  </div >;
}

function GridTable() {
  // 初始化查询函数
  React.useEffect(() => {
    onQueryGridData({});
  }, []);
  const [selectedRowKeys, setSelectedRowKeys] = React.useState([]);
  const [pagination, setPagination] = React.useState({
    sizeCanChange: true,
    showTotal: true,
    total: 0,
    pageSize: 10,
    current: 1,
    pageSizeChangeResetCurrent: true,
  });
  pageData.setPagination = setPagination;
  pageData.pagination = pagination;
  const [gridData, setGridData] = React.useState([]);
  pageData.setGridData = setGridData;

  const columns = [
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
      dataIndex: 'appSeecretKey',
    },
    {
      title: '应用类型',
      dataIndex: 'appType',
    },
    {
      title: '应用状态',
      dataIndex: 'recordStatus',
      render: (_, record) => {
        if (record.recordStatus == '0') {
          return <arco.Tag style={{ color: '#165dff' }}>开发中</arco.Tag>
        }
        if (record.recordStatus == '1') {
          return <arco.Tag style={{ color: '#7bc616' }}> 已发布</arco.Tag >
        }
        if (record.recordStatus == '2') {
          return <arco.Tag style={{ color: '#ff5722' }} > 已下线</arco.Tag >
        }
      }
    },
    {
      title: '操作',
      dataIndex: 'op',
      width: '120px',
      render: (_, record) => {
        return <div className="bmbp-grid-row-action">
          <arco.Button type='text' size={'mini'} onClick={() => onRowConfigBtnClick(record)}>配置</arco.Button>
          <arco.Button type='text' size={'mini'} onClick={() => onRowEditBtnClick(record)}>编辑</arco.Button>
          <arco.Popover
            trigger='hover' position='left'
            content={
              <div className="bmbp-grid-row-action-more">
                {record.recordStatus == '0' ? <arco.Button type='text' size={'mini'} onClick={() => onRowDisableBtnClick(record)}>上线</arco.Button> : <arco.Button type='text' size={'mini'} onClick={() => onRowEnableBtnClick(record)}>下线</arco.Button>}
                <arco.Button type='text' size={'mini'} onClick={() => onRowInfoBtnClick(record)}>查看</arco.Button>
                <arco.Button type='text' size={'mini'} onClick={() => onRowDelBtnClick(record)} status='danger'>删除</arco.Button>
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
      type: 'checkbox', checkAll: true, fixed: true, selectedRowKeys: selectedRowKeys,
      onChange: (selectedRowKeys, _) => {
        setSelectedRowKeys(selectedRowKeys);
      },
    }}
    rowKey={'recordId'} columns={columns} data={gridData} pagination={pagination} onChange={onGridPageChange} />;
}


function AppPage() {
  return <div className="bmbp-page-body">
    <div className="bmbp-page-serach-form">
      <SearchForm />
    </div>
    <div className="bmbp-page-serach-toolbar">
      <div className="bmbp-page-serach-toolbar-left">
        <arco.Button type='primary' onClick={onToolBarAddBtnClick}>新增</arco.Button>
        <arco.Button type='secondary' onClick={onToolBarDelBtnClick}>删除</arco.Button>
      </div>
      <div className="bmbp-page-serach-toolbar-right">
        <arco.Button type='secondary' icon={<arcoicon.IconImport />} onClick={onToolBarImportBtnClick}></arco.Button>
        <arco.Button type='secondary' icon={<arcoicon.IconExport />} onClick={onToolBarExportBtnClick}></arco.Button>
        <arco.Button type='secondary' icon={<arcoicon.IconPrinter />} onClick={onToolBarPrintBtnClick}></arco.Button>
      </div>
    </div>
    <div className="bmbp-page-serach-grid">
      <GridTable />
    </div>
  </div>;
}


// 登录界面的APPView
function RbacAppView() {

  return <AppPage />;
}
