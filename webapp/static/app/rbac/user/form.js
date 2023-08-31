const SearchForm = () => {
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
    <arco.Form ref={PageContext.searchFormRef} {...formItemLayout}>
      <arco.Grid.Row gutter={24}>
        <arco.Grid.Col span={7}>
          <arco.Form.Item field="userName" label='用户名称'>
            <arco.Input placeholder='请输入用户名称' />
          </arco.Form.Item>
        </arco.Grid.Col>
        <arco.Grid.Col span={7}>
          <arco.Form.Item field="userNickName" label='显示名称'>
            <arco.Input placeholder='请输入用户显示名称' />
          </arco.Form.Item>
        </arco.Grid.Col>
        <arco.Grid.Col span={7}>
          <arco.Form.Item field="recordStatus" label='用户状态' allowClear>
            <arco.Select placeholder='请选择用户状态'>
              <arco.Select.Option key={'1'} value={'1'}>正常</arco.Select.Option>
              <arco.Select.Option key={'0'} value={'0'}>已停用</arco.Select.Option>
            </arco.Select>
          </arco.Form.Item>
        </arco.Grid.Col>
        <arco.Grid.Col span={3}>
          <arco.Button type='primary' style={searchBtnStyle} onClick={() => { onSearchFormQueryEvent() }}>查询</arco.Button>
          <arco.Button type='secondary' style={searchBtnStyle} onClick={() => { onSearchFormRestEvent() }}>重置</arco.Button>
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
        onSaveFormInfo(formData, PageContext.setAddFormShow);
        PageContext.addFormRef.current.resetFields();
      }).catch((_) => {
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
          <arco.Form.Item field="recordId" label='主键' hidden={true}>
            <arco.Input placeholder='' />
          </arco.Form.Item>
          <arco.Form.Item field="organCode" label='组织编码' hidden={true}>
            <arco.Input placeholder='' />
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

const EditFormDialog = (props) => {
  return <arco.Modal
    title={props.title}
    visible={props.visible}
    autoFocus={false}
    focusLock={true}
    onOk={() => {
      PageContext.editFormRef.current.validate().then((formData) => {
        onSaveFormInfo(formData, PageContext.setEditFormShow);
        PageContext.editFormRef.current.resetFields();
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
        onQueryFormInfo(PageContext.initFormValue.recordId, PageContext.editFormRef.current);
      } else {
        PageContext.editFormRef.current.setFieldsValue(PageContext.initFormValue);
      }
    }
  }, [PageContext.initFormValue]);
  return <div>
    <arco.Form ref={PageContext.editFormRef}>
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
          <arco.Form.Item field="organTitle" label='组织名称' rules={[{ required: true, message: '组织名称不能为空' }, { minLength: 2, maxLength: 32, message: '应用编码长度2到32' }]}>
            <arco.Input placeholder='请输入组织名称' />
          </arco.Form.Item>
        </arco.Grid.Col>
        <arco.Grid.Col span={24}>
          <arco.Form.Item disabled={true} field="organType" label='组织类型' allowClear rules={[{ required: true, message: '组织类型不能为空' }]} >
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
        onQueryFormInfo(PageContext.initFormValue.recordId, PageContext.infoFormRef.current);
      } else {
        PageContext.infoFormRef.current.setFieldsValue(PageContext.initFormValue);
      }
    }
  }, [PageContext.initFormValue]);
  return <div>
    <arco.Form ref={PageContext.infoFormRef} disabled={true}>
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
  </div>;
}


const ConfigFormDialog = (props) => {
  return <arco.Modal
    title={props.title}
    visible={props.visible}
    autoFocus={false}
    focusLock={true}
    onOk={() => {
      PageContext.configFormRef.current.validate().then((formData) => {
        onSaveFormInfo(formData, PageContext.setConfigFormShow);
        PageContext.configFormRef.current.resetFields();
      }).catch((_) => {
      });
    }}
    onCancel={() => {
      PageContext.configFormRef.current.resetFields();
      PageContext.setConfigFormShow(false);
    }}
  >
    <ConfigFormView />
  </arco.Modal>
}

const ConfigFormView = () => {
  React.useEffect(() => {
    if (PageContext.initFormValue) {
      if (PageContext.initFormValue.recordId) {
        onQueryFormInfo(PageContext.initFormValue.recordId, PageContext.configFormRef.current);
      } else {
        PageContext.configFormRef.current.setFieldsValue(PageContext.initFormValue);
      }
    }
  }, [PageContext.initFormValue]);
  return <div>
    <arco.Form ref={PageContext.configFormRef}>
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
          <arco.Form.Item field="organTitle" label='组织名称' rules={[{ required: true, message: '组织名称不能为空' }, { minLength: 2, maxLength: 32, message: '应用编码长度2到32' }]}>
            <arco.Input placeholder='请输入组织名称' />
          </arco.Form.Item>
        </arco.Grid.Col>
        <arco.Grid.Col span={24}>
          <arco.Form.Item disabled={true} field="organType" label='组织类型' allowClear rules={[{ required: true, message: '组织类型不能为空' }]} >
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
  </div>;
}


const ChangeOrganDialog = (props) => {
  return <arco.Modal
    title={props.title}
    visible={props.visible}
    autoFocus={false}
    focusLock={true}
    onOk={() => {
      let recordId = PageContext.initFormValue.recordId;
      let organCode = PageContext.initFormValue.organCode;
      onSaveOrganChangeInfo(recordId, organCode);
      PageContext.setInitFormValue({});
      PageContext.setChangeOrganTreeData([]);
      PageContext.setChangeOrganShow(false);
    }}
    onCancel={() => {
      // 清空初始值
      PageContext.setInitFormValue({});
      PageContext.setChangeOrganTreeData([]);
      PageContext.setChangeOrganShow(false);
    }}
  >
    <ChangeOranTreeView />
  </arco.Modal>
}
const ChangeOranTreeView = () => {
  React.useEffect(() => {
    if (PageContext.changeOrganShow) {
      onQueryChangeOrganTreeData();
    }
  }, [PageContext.setChangeOrganShow]);
  const treeFiledNames = {
    key: 'organCode',
    title: 'organTitle',
    children: 'organChildren',
  };
  return <arco.Tree ref={PageContext.changeOrganTreeRef} showLine blockNode multiple={false} checkStrictly={true} fieldNames={treeFiledNames} treeData={PageContext.changeOrganTreeData}
    onSelect={(keys, _) => {
      let key = keys[0];
      PageContext.setInitFromValue({ organCode: key, recordId: PageContext.initFormValue.recordId });
    }}
  ></arco.Tree>;
}
