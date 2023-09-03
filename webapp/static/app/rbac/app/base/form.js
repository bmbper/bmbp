const SearchForm = () => {
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
    <arco.Form ref={PageContext.searchFormRef} {...formItemLayout}>
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
          <arco.Form.Item field="appType" label='应用类型' allowClear>
            <arco.Select placeholder='请选择应用类型'>
              <arco.Select.Option key={'1'} value={'module'}>内置应用</arco.Select.Option>
              <arco.Select.Option key={'2'} value={'sso'}>单点应用</arco.Select.Option>
              <arco.Select.Option key={'3'} value={'link'}>链接应用</arco.Select.Option>
            </arco.Select>
          </arco.Form.Item>
        </arco.Grid.Col>
        <arco.Grid.Col span={7}>
          <arco.Form.Item field="recordStatus" label='应用状态' allowClear>
            <arco.Select placeholder='请选择应用状态'>
              <arco.Select.Option key={'1'} value={'1'}>开发中</arco.Select.Option>
              <arco.Select.Option key={'2'} value={'2'}>已发布</arco.Select.Option>
              <arco.Select.Option key={'3'} value={'3'}>已下线</arco.Select.Option>
            </arco.Select>
          </arco.Form.Item>
        </arco.Grid.Col>
        <arco.Grid.Col span={3}>
          <arco.Button type='primary' style={searchBtnStyle} onClick={onSearchFormQueryEvent}>查询</arco.Button>
          <arco.Button type='secondary' style={searchBtnStyle} onClick={onSearchFormRestEvent}>重置</arco.Button>
        </arco.Grid.Col>
      </arco.Grid.Row>
    </arco.Form>
  </div >;
}


const AddFormDialog = (props) => {
  return <arco.Modal
    title={props.title}
    visible={props.visible}
    autoFocus={false}
    focusLock={true}
    onOk={() => {
      PageContext.addFormRef.current.validate().then((formData) => {
        onSaveFormInfo(formData, () => {
          PageContext.setAddFormShow(false);
          PageContext.addFormRef.current.resetFields();
        });
      }).catch((_) => {
        PageContext.addFormRef.current.resetFields();
      });
    }}
    onCancel={() => {
      PageContext.addFormRef.current.resetFields();
      PageContext.setAddFormShow(false);
    }}
  >
    <AddFormView />
  </arco.Modal>
}

const AddFormView = () => {
  React.useEffect(() => {
    if (PageContext.initFormValue) {
      PageContext.addFormRef.current.setFieldsValue(PageContext.initFormValue);
    }
  }, [PageContext.initFormValue]);
  return <div>
    <arco.Form ref={PageContext.addFormRef}>
      <arco.Grid.Row gutter={24}>
        <arco.Grid.Col span={24}>
          <arco.Form.Item field="recordId" label='应用主键' hidden={true}>
            <arco.Input placeholder='' />
          </arco.Form.Item>
        </arco.Grid.Col>
        <arco.Grid.Col span={24}>
          <arco.Form.Item field="appCode" label='应用编码' rules={[{ required: true, message: '应用编码不能为空' }, { minLength: 4, maxLength: 32, message: '应用编码长度4到32' }]}>
            <arco.Input placeholder='请输入应用编码' />
          </arco.Form.Item>
        </arco.Grid.Col>
        <arco.Grid.Col span={24}>
          <arco.Form.Item field="appTitle" label='应用名称' rules={[{ required: true, message: '应用名称不能为空' }, { minLength: 4, maxLength: 32, message: '应用名称长度4到32' }]}>
            <arco.Input placeholder='请输入应用名称' />
          </arco.Form.Item>
        </arco.Grid.Col>
        <arco.Grid.Col span={24}>
          <arco.Form.Item field="appKey" label='应用标识' rules={[{ required: true, message: '应用标识不能为空' }, { minLength: 4, maxLength: 32, message: '应用标识长度4到32' }]}>
            <arco.Input placeholder='请输入应用标识' />
          </arco.Form.Item>
        </arco.Grid.Col>
        <arco.Grid.Col span={24}>
          <arco.Form.Item field="appSecrectKey" label='应用密钥' rules={[{ required: true, message: '应用密钥不能为空' }, { minLength: 32, maxLength: 32, message: '应用密钥为32位加密值' }]} >
            <arco.Input placeholder='请输入应用密钥' />
          </arco.Form.Item>
        </arco.Grid.Col>
        <arco.Grid.Col span={24}>
          <arco.Form.Item field="appType" label='应用类型' allowClear rules={[{ required: true, message: '应用类型不能为空' }]} >
            <arco.Select placeholder='请选择应用类型'>
              <arco.Select.Option key={'1'} value={'module'}>内置应用</arco.Select.Option>
              <arco.Select.Option key={'2'} value={'sso'}>单点应用</arco.Select.Option>
              <arco.Select.Option key={'3'} value={'link'}>链接应用</arco.Select.Option>
            </arco.Select>
          </arco.Form.Item>
        </arco.Grid.Col>
        <arco.Grid.Col span={24}>
          <arco.Form.Item field="recordNum" label='显示顺序' allowClear rules={[{ required: true, message: '显示顺序不能为空' }]}>
            <arco.InputNumber placeholder='顺序' />
          </arco.Form.Item>
        </arco.Grid.Col>
        <arco.Grid.Col span={24}>
          <arco.Form.Item field="recordRemark" label='应用说明' allowClear rules={[{ maxLength: 255, message: '应用说明最长255' }]}>
            <arco.Input.TextArea placeholder='输入应用说明' style={{ minHeight: 64 }} />
          </arco.Form.Item>
        </arco.Grid.Col>
      </arco.Grid.Row>
    </arco.Form>
  </div >;
}


const EditFormDialog = (props) => {
  return <arco.Modal
    title={props.title}
    visible={props.visible}
    autoFocus={false}
    focusLock={true}
    onOk={() => {
      PageContext.editFormRef.current.validate().then((formData) => {
        onSaveFormInfo(formData, () => {
          PageContext.setEditFormShow(false);
          PageContext.editFormRef.current.resetFields();
        });
      }).catch((_) => {
      });
    }}
    onCancel={() => {
      PageContext.editFormRef.current.resetFields();
      PageContext.setEditFormShow(false);
    }}
  >
    <EditFormView />
  </arco.Modal>
}

const EditFormView = () => {
  React.useEffect(() => {
    if (PageContext.initFormValue) {
      if (PageContext.initFormValue.recordId) {
        onQueryFormInfo(PageContext.initFormValue.recordId, (data) => {
          PageContext.editFormRef.current.setFieldsValue(data);
        });
      } else {
        PageContext.editFormRef.current.setFieldsValue(PageContext.initFormValue);
      }
    }
  }, [PageContext.initFormValue]);
  return <div>
    <arco.Form ref={PageContext.editFormRef}>
      <arco.Grid.Row gutter={24}>
        <arco.Grid.Col span={24}>
          <arco.Form.Item field="recordId" label='应用主键' hidden={true}>
            <arco.Input placeholder='' />
          </arco.Form.Item>
        </arco.Grid.Col>
        <arco.Grid.Col span={24}>
          <arco.Form.Item field="appCode" label='应用编码' rules={[{ required: true, message: '应用编码不能为空' }, { minLength: 4, maxLength: 32, message: '应用编码长度4到32' }]}>
            <arco.Input placeholder='请输入应用编码' />
          </arco.Form.Item>
        </arco.Grid.Col>
        <arco.Grid.Col span={24}>
          <arco.Form.Item field="appTitle" label='应用名称' rules={[{ required: true, message: '应用名称不能为空' }, { minLength: 4, maxLength: 32, message: '应用名称长度4到32' }]}>
            <arco.Input placeholder='请输入应用名称' />
          </arco.Form.Item>
        </arco.Grid.Col>
        <arco.Grid.Col span={24}>
          <arco.Form.Item field="appKey" label='应用标识' rules={[{ required: true, message: '应用标识不能为空' }, { minLength: 4, maxLength: 32, message: '应用标识长度4到32' }]}>
            <arco.Input placeholder='请输入应用标识' />
          </arco.Form.Item>
        </arco.Grid.Col>
        <arco.Grid.Col span={24}>
          <arco.Form.Item field="appSecrectKey" label='应用密钥' rules={[{ required: true, message: '应用密钥不能为空' }, { minLength: 32, maxLength: 32, message: '应用密钥为32位加密值' }]} >
            <arco.Input placeholder='请输入应用密钥' />
          </arco.Form.Item>
        </arco.Grid.Col>
        <arco.Grid.Col span={24}>
          <arco.Form.Item field="appType" label='应用类型' allowClear rules={[{ required: true, message: '应用类型不能为空' }]} >
            <arco.Select placeholder='请选择应用类型'>
              <arco.Select.Option key={'1'} value={'module'}>内置应用</arco.Select.Option>
              <arco.Select.Option key={'2'} value={'sso'}>单点应用</arco.Select.Option>
              <arco.Select.Option key={'3'} value={'link'}>链接应用</arco.Select.Option>
            </arco.Select>
          </arco.Form.Item>
        </arco.Grid.Col>
        <arco.Grid.Col span={24}>
          <arco.Form.Item field="recordNum" label='显示顺序' allowClear rules={[{ required: true, message: '显示顺序不能为空' }]}>
            <arco.InputNumber placeholder='顺序' />
          </arco.Form.Item>
        </arco.Grid.Col>
        <arco.Grid.Col span={24}>
          <arco.Form.Item field="recordRemark" label='应用说明' allowClear rules={[{ maxLength: 255, message: '应用说明最长255' }]}>
            <arco.Input.TextArea placeholder='输入应用说明' style={{ minHeight: 64 }} />
          </arco.Form.Item>
        </arco.Grid.Col>
      </arco.Grid.Row>
    </arco.Form>
  </div>;
}

const InfoFormDialog = (props) => {
  return <arco.Modal
    title={props.title}
    visible={props.visible}
    autoFocus={false}
    focusLock={true}
    footer={null}
    onCancel={() => {
      PageContext.infoFormRef.current.resetFields();
      PageContext.setInfoFormShow(false);
    }}
  >
    <InfoFormView />
  </arco.Modal>
}

const InfoFormView = () => {
  React.useEffect(() => {
    if (PageContext.initFormValue) {
      if (PageContext.initFormValue.recordId) {
        onQueryFormInfo(PageContext.initFormValue.recordId, (data) => {
          PageContext.infoFormRef.current.setFieldsValue(data);
        });
      } else {
        PageContext.infoFormRef.current.setFieldsValue(PageContext.initFormValue);
      }
    }
  }, [PageContext.initFormValue]);
  return <div>
    <arco.Form ref={PageContext.infoFormRef} disabled={true}>
      <arco.Grid.Row gutter={24}>
        <arco.Grid.Col span={24}>
          <arco.Form.Item field="recordId" label='应用主键' hidden={true}>
            <arco.Input placeholder='' />
          </arco.Form.Item>
        </arco.Grid.Col>
        <arco.Grid.Col span={24}>
          <arco.Form.Item field="appCode" label='应用编码' rules={[{ required: true, message: '应用编码不能为空' }, { minLength: 4, maxLength: 32, message: '应用编码长度4到32' }]}>
            <arco.Input placeholder='请输入应用编码' />
          </arco.Form.Item>
        </arco.Grid.Col>
        <arco.Grid.Col span={24}>
          <arco.Form.Item field="appTitle" label='应用名称' rules={[{ required: true, message: '应用名称不能为空' }, { minLength: 4, maxLength: 32, message: '应用名称长度4到32' }]}>
            <arco.Input placeholder='请输入应用名称' />
          </arco.Form.Item>
        </arco.Grid.Col>
        <arco.Grid.Col span={24}>
          <arco.Form.Item field="appKey" label='应用标识' rules={[{ required: true, message: '应用标识不能为空' }, { minLength: 4, maxLength: 32, message: '应用标识长度4到32' }]}>
            <arco.Input placeholder='请输入应用标识' />
          </arco.Form.Item>
        </arco.Grid.Col>
        <arco.Grid.Col span={24}>
          <arco.Form.Item field="appSecrectKey" label='应用密钥' rules={[{ required: true, message: '应用密钥不能为空' }, { minLength: 32, maxLength: 32, message: '应用密钥为32位加密值' }]} >
            <arco.Input placeholder='请输入应用密钥' />
          </arco.Form.Item>
        </arco.Grid.Col>
        <arco.Grid.Col span={24}>
          <arco.Form.Item field="appType" label='应用类型' allowClear rules={[{ required: true, message: '应用类型不能为空' }]} >
            <arco.Select placeholder='请选择应用类型'>
              <arco.Select.Option key={'1'} value={'module'}>内置应用</arco.Select.Option>
              <arco.Select.Option key={'2'} value={'sso'}>单点应用</arco.Select.Option>
              <arco.Select.Option key={'3'} value={'link'}>链接应用</arco.Select.Option>
            </arco.Select>
          </arco.Form.Item>
        </arco.Grid.Col>
        <arco.Grid.Col span={24}>
          <arco.Form.Item field="recordNum" label='显示顺序' allowClear rules={[{ required: true, message: '显示顺序不能为空' }]}>
            <arco.InputNumber placeholder='顺序' />
          </arco.Form.Item>
        </arco.Grid.Col>
        <arco.Grid.Col span={24}>
          <arco.Form.Item field="recordRemark" label='应用说明' allowClear rules={[{ maxLength: 255, message: '应用说明最长255' }]}>
            <arco.Input.TextArea placeholder='输入应用说明' style={{ minHeight: 64 }} />
          </arco.Form.Item>
        </arco.Grid.Col>
      </arco.Grid.Row>
    </arco.Form>
  </div>;
}
