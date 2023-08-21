const AddRootOrganDialog = (props) => {
  return <arco.Modal
    title={props.title}
    visible={props.visible}
    autoFocus={false}
    focusLock={true}
    onOk={() => {
      AppIns.organFormRef.current.validate().then((formData) => {
        saveOrganInfo(formData, AppIns.setAddOrganFormShow);
        AppIns.organFormRef.current.resetFields();
      }).catch((_) => {
      });
    }}
    onCancel={() => {
      AppIns.organFormRef.current.resetFields();
      AppIns.setAddOrganFormShow(false);
    }}
  >
    <OrganFormView />
  </arco.Modal>
}

const OrganFormView = () => {
  React.useEffect(() => {
    AppIns.organFormRef.current.setFieldsValue(AppIns.initOrganValue);
  }, [AppIns.initOrganValue]);
  return <div>
    <arco.Form ref={AppIns.organFormRef}>
      <arco.Grid.Row gutter={24}>
        <arco.Grid.Col span={24}>
          <arco.Form.Item field="recordId" label='主键' hidden={true}>
            <arco.Input placeholder='' />
          </arco.Form.Item>
          <arco.Form.Item field="organParentCode" label='上级组织编码' hidden={true}>
            <arco.Input placeholder='' />
          </arco.Form.Item>
        </arco.Grid.Col>
        <arco.Grid.Col span={24}>
          <arco.Form.Item field="organParentTitle" label='上级组织'>
            <arco.Input disabled={true} />
          </arco.Form.Item>
        </arco.Grid.Col>
        <arco.Grid.Col span={24}>
          <arco.Form.Item field="organTitle" label='组织名称' rules={[{ required: true, message: '组织名称不能为空' }, { minLength: 2, maxLength: 32, message: '应用编码长度2到32' }]}>
            <arco.Input placeholder='请输入组织名称' />
          </arco.Form.Item>
        </arco.Grid.Col>
        <arco.Grid.Col span={24}>
          <arco.Form.Item field="organType" label='组织类型' allowClear rules={[{ required: true, message: '组织类型不能为空' }]} >
            <arco.Select placeholder='请选择组织类型'>
              <arco.Select.Option key={'1'} value={'units'}>分组</arco.Select.Option>
              <arco.Select.Option key={'2'} value={'unit'}>单位</arco.Select.Option>
              <arco.Select.Option key={'3'} value={'dept'}>部门</arco.Select.Option>
              <arco.Select.Option key={'4'} value={'post'}>岗位</arco.Select.Option>
              <arco.Select.Option key={'5'} value={'person'}>人员</arco.Select.Option>
            </arco.Select>
          </arco.Form.Item>
        </arco.Grid.Col>
        <arco.Grid.Col span={24}>
          <arco.Form.Item field="recordNum" label='显示顺序' allowClear rules={[{ required: true, message: '显示顺序不能为空' }]}>
            <arco.InputNumber placeholder='顺序' />
          </arco.Form.Item>
        </arco.Grid.Col>
      </arco.Grid.Row>
    </arco.Form>
  </div >;
}
