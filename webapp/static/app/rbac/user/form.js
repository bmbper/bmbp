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
          <arco.Form.Item field="recordId" label='主键' hidden={true}>
            <arco.Input placeholder='' />
          </arco.Form.Item>
          <arco.Form.Item field="organId" label='组织ID' hidden={true}>
            <arco.Input placeholder='' />
          </arco.Form.Item>
        </arco.Grid.Col>
        <arco.Grid.Col span={24}>
          <arco.Form.Item field="userName" label='用户名称' rules={[{ required: true, message: '用户名称不能为空' }, { minLength: 4, maxLength: 32, message: '用户名称长度5到32' }]}>
            <arco.Input placeholder='用户名称：登录系统的惟一标识' />
          </arco.Form.Item>
        </arco.Grid.Col>
        <arco.Grid.Col span={24}>
          <arco.Form.Item field="userNickName" label='用户昵称' rules={[{ required: true, message: '用户昵称不能为空' }, { minLength: 2, maxLength: 32, message: '用户昵称长度3到32' }]}>
            <arco.Input placeholder='用户昵称：系统显示名称' />
          </arco.Form.Item>
        </arco.Grid.Col>
        <arco.Grid.Col span={24}>
          <arco.Form.Item field="userPassword" label='用户密码' rules={[{ required: true, message: '用户密码不能为空' }, { minLength: 10, maxLength: 10, message: '请设置10位用户密码' }]}>
            <arco.Input.Password placeholder='用户密码' />
          </arco.Form.Item>
        </arco.Grid.Col>
        <arco.Grid.Col span={24}>
          <arco.Form.Item field="userPasswordConfirm" label='确认密码' dependencies={['userPassword']}
            rules={[{ required: true, message: '确认密码不能为空' }, { minLength: 10, maxLength: 10, message: '请设置10确认密码' }, {
              validator: (value, msg) => {
                if (!value) {
                  return msg('请输入确认密码');
                } else if (PageContext.addFormRef.current.getFieldValue('userPassword') !== value) {
                  return msg('两次密码输入不一致');
                }
                return msg(null);
              }
            }]} >
            <arco.Input.Password placeholder='确认密码' />
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
          <arco.Form.Item field="recordId" label='主键' hidden={true}>
            <arco.Input placeholder='' />
          </arco.Form.Item>
          <arco.Form.Item field="organId" label='组织ID' hidden={true}>
            <arco.Input placeholder='' />
          </arco.Form.Item>
        </arco.Grid.Col>
        <arco.Grid.Col span={24}>
          <arco.Form.Item field="userName" label='用户名称' rules={[{ required: true, message: '用户名称不能为空' }, { minLength: 4, maxLength: 32, message: '用户名称长度5到32' }]}>
            <arco.Input placeholder='用户名称：登录系统的惟一标识' />
          </arco.Form.Item>
        </arco.Grid.Col>
        <arco.Grid.Col span={24}>
          <arco.Form.Item field="userNickName" label='用户昵称' rules={[{ required: true, message: '用户昵称不能为空' }, { minLength: 2, maxLength: 32, message: '用户昵称长度3到32' }]}>
            <arco.Input placeholder='用户昵称：系统显示名称' />
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
          <arco.Form.Item field="recordId" label='主键' hidden={true}>
            <arco.Input placeholder='' />
          </arco.Form.Item>
          <arco.Form.Item field="organId" label='组织ID' hidden={true}>
            <arco.Input placeholder='' />
          </arco.Form.Item>
        </arco.Grid.Col>
        <arco.Grid.Col span={24}>
          <arco.Form.Item field="userName" label='用户名称' rules={[{ required: true, message: '用户名称不能为空' }, { minLength: 4, maxLength: 32, message: '用户名称长度5到32' }]}>
            <arco.Input placeholder='用户名称：登录系统的惟一标识' />
          </arco.Form.Item>
        </arco.Grid.Col>
        <arco.Grid.Col span={24}>
          <arco.Form.Item field="userNickName" label='用户昵称' rules={[{ required: true, message: '用户昵称不能为空' }, { minLength: 2, maxLength: 32, message: '用户昵称长度3到32' }]}>
            <arco.Input placeholder='用户昵称：系统显示名称' />
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
        onSaveFormInfo(formData, () => {
          PageContext.configFormRef.current.resetFields();
          PageContext.setConfigFormShow(false);
        });

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
        onQueryFormInfo(PageContext.initFormValue.recordId, (data) => {
          PageContext.configFormRef.current.setFieldsValue(data);
        });
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
          <arco.Form.Item field="organId" label='组织ID' hidden={true}>
            <arco.Input placeholder='' />
          </arco.Form.Item>
        </arco.Grid.Col>
        <arco.Grid.Col span={24}>
          <arco.Form.Item field="userName" label='用户名称' rules={[{ required: true, message: '用户名称不能为空' }, { minLength: 4, maxLength: 32, message: '用户名称长度5到32' }]}>
            <arco.Input placeholder='用户名称：登录系统的惟一标识' />
          </arco.Form.Item>
        </arco.Grid.Col>
        <arco.Grid.Col span={24}>
          <arco.Form.Item field="userNickName" label='用户昵称' rules={[{ required: true, message: '用户昵称不能为空' }, { minLength: 2, maxLength: 32, message: '用户昵称长度3到32' }]}>
            <arco.Input placeholder='用户昵称：系统显示名称' />
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
      let organId = PageContext.initFormValue.organId;
      onSaveOrganChangeInfo(recordId, organId);
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
    key: 'recordId',
    title: 'organTitle',
    children: 'organChildren',
  };
  return <arco.Tree ref={PageContext.changeOrganTreeRef} showLine blockNode multiple={false} checkStrictly={true} fieldNames={treeFiledNames} treeData={PageContext.changeOrganTreeData}
    onSelect={(keys, _) => {
      let key = keys[0];
      PageContext.setInitFormValue({ organId: key, recordId: PageContext.initFormValue.recordId });
    }}
  ></arco.Tree>;
}
