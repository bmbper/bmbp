
const AppAddFormDialog = (props) => {
  return <arco.Modal
    title={props.title}
    visible={props.visible}
    autoFocus={false}
    focusLock={true}
    onOk={() => {
      AppPageIns.appFormRef.current.validate().then((formData) => {
        saveAppInfo(formData);
      }).catch((_) => {
      });
    }}
    onCancel={() => {
      AppPageIns.appFormRef.current.resetFields();
      AppPageIns.setAddFormVisible(false);
    }}
  >
    <AppFormView />
  </arco.Modal>
}

const AppFormView = () => {
  return <div>
    <arco.Form ref={AppPageIns.appFormRef}>
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


const AppEditFormDialog = (props) => {
  return <arco.Modal
    title={props.title}
    visible={props.visible}
    autoFocus={false}
    focusLock={true}
    onOk={() => {
      AppPageIns.appFormRef.current.validate().then((formData) => {
        saveAppInfo(formData, AppPageIns.setEditFormVisible);
      }).catch((_) => {
      });
    }}
    onCancel={() => {
      AppPageIns.appFormRef.current.resetFields();
      AppPageIns.setEditFormVisible(false);
    }}
  >
    <AppFormView />
  </arco.Modal>
}


const AppInfoFormDialog = (props) => {
  return <arco.Modal
    title={props.title}
    visible={props.visible}
    autoFocus={false}
    focusLock={true}
    footer={null}
    onCancel={() => {
      AppPageIns.appFormRef.current.resetFields();
      AppPageIns.setInfoFormVisible(false);
    }}
  >
    <AppFormView />
  </arco.Modal>
}

const AppConfigForm = (props) => {
  return <arco.Modal
    title={props.title}
    visible={props.visible}
    autoFocus={false}
    focusLock={true}
    onOk={() => AppPageIns.setConfigFormVisible(false)}
    onCancel={() => AppPageIns.setConfigFormVisible(false)}
  >
    <div> Config Form:{props.recordId}</div>
  </arco.Modal>
}
