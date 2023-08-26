const AddRootRoleDialog = (props) => {
  return <arco.Modal
    title={props.title}
    visible={props.visible}
    autoFocus={false}
    focusLock={true}
    onOk={() => {
      AppIns.roleFormRef.current.validate().then((formData) => {
        saveRoleInfo(formData, AppIns.setAddRoleFormShow);
        AppIns.roleFormRef.current.resetFields();
      }).catch((_) => {
      });
    }}
    onCancel={() => {
      AppIns.roleFormRef.current.resetFields();
      AppIns.setAddRoleFormShow(false);
    }}
  >
    <RoleFormView />
  </arco.Modal>
}

const RoleFormView = () => {
  React.useEffect(() => {
    if (AppIns.initRoleValue) {
      if (AppIns.initRoleValue.recordId) {
        onQueryRoleInfo(AppIns.initRoleValue.recordId);
      } else {
        AppIns.roleFormRef.current.setFieldsValue(AppIns.initRoleValue);
      }
    }


  }, [AppIns.initRoleValue]);
  return <div>
    <arco.Form ref={AppIns.roleFormRef}>
      <arco.Grid.Row gutter={24}>
        <arco.Grid.Col span={24}>
          <arco.Form.Item field="recordId" label='主键' hidden={true}>
            <arco.Input placeholder='' />
          </arco.Form.Item>
          <arco.Form.Item field="roleParentCode" label='上级角色编码' hidden={true}>
            <arco.Input placeholder='' />
          </arco.Form.Item>
        </arco.Grid.Col>
        <arco.Grid.Col span={24}>
          <arco.Form.Item field="roleParentTitle" label='上级角色'>
            <arco.Input disabled={true} />
          </arco.Form.Item>
        </arco.Grid.Col>
        <arco.Grid.Col span={24}>
          <arco.Form.Item field="roleTitle" label='角色名称' rules={[{ required: true, message: '角色名称不能为空' }, { minLength: 2, maxLength: 32, message: '应用编码长度2到32' }]}>
            <arco.Input placeholder='请输入角色名称' />
          </arco.Form.Item>
        </arco.Grid.Col>
        <arco.Grid.Col span={24}>
          <arco.Form.Item field="roleType" label='角色类型' allowClear rules={[{ required: true, message: '角色类型不能为空' }]} >
            <arco.Select placeholder='请选择角色类型'>
              <arco.Select.Option key={'1'} value={'1'}>管理角色</arco.Select.Option>
              <arco.Select.Option key={'2'} value={'2'}>业务角色</arco.Select.Option>
            </arco.Select>
          </arco.Form.Item>
        </arco.Grid.Col>
        <arco.Grid.Col span={24}>
          <arco.Form.Item field="recordNum" label='显示顺序' allowClear rules={[{ required: true, message: '显示顺序不能为空' }]}>
            <arco.InputNumber placeholder='顺序' />
          </arco.Form.Item>
        </arco.Grid.Col>
        <arco.Grid.Col span={24}>
          <arco.Form.Item field="recordRemark" label='角色说明' allowClear rules={[{ maxLength: 255, message: '角色说明最长255' }]}>
            <arco.Input.TextArea placeholder='输入角色说明' style={{ minHeight: 64 }} />
          </arco.Form.Item>
        </arco.Grid.Col>
      </arco.Grid.Row>
    </arco.Form>
  </div >;
}

const EditRoleDialog = (props) => {
  return <arco.Modal
    title={props.title}
    visible={props.visible}
    autoFocus={false}
    focusLock={true}
    onOk={() => {
      AppIns.roleEditFormRef.current.validate().then((formData) => {
        saveRoleInfo(formData, AppIns.setEditRoleFormShow);
        AppIns.roleEditFormRef.current.resetFields();
      }).catch((_) => {
      });
    }}
    onCancel={() => {
      AppIns.roleEditFormRef.current.resetFields();
      AppIns.setEditRoleFormShow(false);
    }}
  >
    <EditRoleFormView />
  </arco.Modal>
}

const EditRoleFormView = () => {
  React.useEffect(() => {
    if (AppIns.initRoleValue) {
      if (AppIns.initRoleValue.recordId) {
        onQueryRoleInfo(AppIns.initRoleValue.recordId, AppIns.roleEditFormRef.current);
      } else {
        AppIns.roleEditFormRef.current.setFieldsValue(AppIns.initRoleValue);
      }
    }
  }, [AppIns.initRoleValue]);
  return <div>
    <arco.Form ref={AppIns.roleEditFormRef}>
      <arco.Grid.Row gutter={24}>
        <arco.Grid.Col span={24}>
          <arco.Form.Item field="recordId" label='主键' hidden={true}>
            <arco.Input placeholder='' />
          </arco.Form.Item>
          <arco.Form.Item field="roleParentCode" label='上级角色编码' hidden={true}>
            <arco.Input placeholder='' />
          </arco.Form.Item>
        </arco.Grid.Col>
        <arco.Grid.Col span={24}>
          <arco.Form.Item field="roleTitle" label='角色名称' rules={[{ required: true, message: '角色名称不能为空' }, { minLength: 2, maxLength: 32, message: '应用编码长度2到32' }]}>
            <arco.Input placeholder='请输入角色名称' />
          </arco.Form.Item>
        </arco.Grid.Col>
        <arco.Grid.Col span={24}>
          <arco.Form.Item disabled={true} field="roleType" label='角色类型' allowClear rules={[{ required: true, message: '角色类型不能为空' }]} >
            <arco.Select placeholder='请选择角色类型'>
              <arco.Select.Option key={'1'} value={'1'}>管理角色</arco.Select.Option>
              <arco.Select.Option key={'2'} value={'2'}>业务角色</arco.Select.Option>
            </arco.Select>
          </arco.Form.Item>
        </arco.Grid.Col>
        <arco.Grid.Col span={24}>
          <arco.Form.Item field="recordNum" label='显示顺序' allowClear rules={[{ required: true, message: '显示顺序不能为空' }]}>
            <arco.InputNumber placeholder='顺序' />
          </arco.Form.Item>
        </arco.Grid.Col>
        <arco.Grid.Col span={24}>
          <arco.Form.Item field="recordRemark" label='角色说明' allowClear rules={[{ maxLength: 255, message: '角色说明最长255' }]}>
            <arco.Input.TextArea placeholder='输入角色说明' style={{ minHeight: 64 }} />
          </arco.Form.Item>
        </arco.Grid.Col>
      </arco.Grid.Row>
    </arco.Form>
  </div>;
}

const InfoRoleDialog = (props) => {
  return <arco.Modal
    title={props.title}
    visible={props.visible}
    autoFocus={false}
    focusLock={true}
    footer={null}
    onCancel={() => {
      AppIns.roleInfoFormRef.current.resetFields();
      AppIns.setInfoRoleFormShow(false);
    }}
  >
    <InfoRoleFormView />
  </arco.Modal>
}

const InfoRoleFormView = () => {
  React.useEffect(() => {
    if (AppIns.initRoleValue) {
      if (AppIns.initRoleValue.recordId) {
        onQueryRoleInfo(AppIns.initRoleValue.recordId, AppIns.roleInfoFormRef.current);
      } else {
        AppIns.roleInfoFormRef.current.setFieldsValue(AppIns.initRoleValue);
      }
    }
  }, [AppIns.initRoleValue]);
  return <div>
    <arco.Form ref={AppIns.roleInfoFormRef} disabled={true}>
      <arco.Grid.Row gutter={24}>
        <arco.Grid.Col span={24}>
          <arco.Form.Item field="recordId" label='主键' hidden={true}>
            <arco.Input placeholder='' />
          </arco.Form.Item>
          <arco.Form.Item field="roleParentCode" label='上级角色编码' hidden={true}>
            <arco.Input placeholder='' />
          </arco.Form.Item>
        </arco.Grid.Col>
        <arco.Grid.Col span={24}>
          <arco.Form.Item field="roleTitle" label='角色名称' rules={[{ required: true, message: '角色名称不能为空' }, { minLength: 2, maxLength: 32, message: '应用编码长度2到32' }]}>
            <arco.Input placeholder='请输入角色名称' />
          </arco.Form.Item>
        </arco.Grid.Col>
        <arco.Grid.Col span={24}>
          <arco.Form.Item field="roleType" label='角色类型' allowClear rules={[{ required: true, message: '角色类型不能为空' }]} >
            <arco.Select placeholder='请选择角色类型'>
              <arco.Select.Option key={'1'} value={'1'}>管理角色</arco.Select.Option>
              <arco.Select.Option key={'2'} value={'2'}>业务角色</arco.Select.Option>
            </arco.Select>
          </arco.Form.Item>
        </arco.Grid.Col>
        <arco.Grid.Col span={24}>
          <arco.Form.Item field="recordNum" label='显示顺序' allowClear rules={[{ required: true, message: '显示顺序不能为空' }]}>
            <arco.InputNumber placeholder='顺序' />
          </arco.Form.Item>
        </arco.Grid.Col>
        <arco.Grid.Col span={24}>
          <arco.Form.Item field="recordRemark" label='角色说明' allowClear rules={[{ maxLength: 255, message: '角色说明最长255' }]}>
            <arco.Input.TextArea placeholder='输入角色说明' style={{ minHeight: 64 }} />
          </arco.Form.Item>
        </arco.Grid.Col>
      </arco.Grid.Row>
    </arco.Form>
  </div>;
}


const ChangeParentRoleDialog = (props) => {
  return <arco.Modal
    title={props.title}
    visible={props.visible}
    autoFocus={false}
    focusLock={true}
    onOk={() => {
      let recordId = AppIns.initRoleValue.recordId;
      let roleParentCode = AppIns.initRoleValue.roleParentCode;
      onSaveRoleParentChange(recordId, roleParentCode);
      AppIns.setInitRoleValue({});
      AppIns.setTreeParentData([]);
      AppIns.setChangeParentRoleShow(false);
    }}
    onCancel={() => {
      // 清空初始值
      AppIns.setInitRoleValue({});
      AppIns.setTreeParentData([]);
      AppIns.setChangeParentRoleShow(false);
    }}
  >
    <ParentRoleTree />
  </arco.Modal>
}
const ParentRoleTree = () => {
  React.useEffect(() => {
    if (AppIns.changeParentRoleShow) {
      onQueryTreeDataWithOutRecordId();
    }
  }, [AppIns.changeParentRoleShow]);
  const treeFiledNames = {
    key: 'roleCode',
    title: 'roleTitle',
    children: 'roleChildren',
  };
  return <arco.Tree ref={AppIns.parentRoleTreeRef} showLine blockNode multiple={false} checkStrictly={true} fieldNames={treeFiledNames} treeData={AppIns.treeParentData}
    onSelect={(keys, evet) => {
      let key = keys[0];
      AppIns.setInitRoleValue({ roleParentCode: key, recordId: AppIns.initRoleValue.recordId });
    }}
  ></arco.Tree>;
}
