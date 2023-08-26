const AddRootMenuDialog = (props) => {
  return <arco.Modal
    title={props.title}
    visible={props.visible}
    autoFocus={false}
    focusLock={true}
    onOk={() => {
      AppIns.menuFormRef.current.validate().then((formData) => {
        saveMenuInfo(formData, AppIns.setAddMenuFormShow);
        AppIns.menuFormRef.current.resetFields();
      }).catch((_) => {
      });
    }}
    onCancel={() => {
      AppIns.menuFormRef.current.resetFields();
      AppIns.setAddMenuFormShow(false);
    }}
  >
    <MenuFormView />
  </arco.Modal>
}

const MenuFormView = () => {
  React.useEffect(() => {
    if (AppIns.initMenuValue) {
      if (AppIns.initMenuValue.recordId) {
        onQueryMenuInfo(AppIns.initMenuValue.recordId);
      } else {
        AppIns.menuFormRef.current.setFieldsValue(AppIns.initMenuValue);
      }
    }


  }, [AppIns.initMenuValue]);
  return <div>
    <arco.Form ref={AppIns.menuFormRef}>
      <arco.Grid.Row gutter={24}>
        <arco.Grid.Col span={24}>
          <arco.Form.Item field="recordId" label='主键' hidden={true}>
            <arco.Input placeholder='' />
          </arco.Form.Item>
          <arco.Form.Item field="menuParentCode" label='上级菜单编码' hidden={true}>
            <arco.Input placeholder='' />
          </arco.Form.Item>
        </arco.Grid.Col>
        <arco.Grid.Col span={24}>
          <arco.Form.Item field="menuParentTitle" label='上级菜单'>
            <arco.Input disabled={true} />
          </arco.Form.Item>
        </arco.Grid.Col>
        <arco.Grid.Col span={24}>
          <arco.Form.Item field="menuTitle" label='菜单名称' rules={[{ required: true, message: '菜单名称不能为空' }, { minLength: 2, maxLength: 32, message: '应用编码长度2到32' }]}>
            <arco.Input placeholder='请输入菜单名称' />
          </arco.Form.Item>
        </arco.Grid.Col>
        <arco.Grid.Col span={24}>
          <arco.Form.Item field="menuType" label='菜单类型' allowClear rules={[{ required: true, message: '菜单类型不能为空' }]} >
            <arco.Select placeholder='请选择菜单类型'>
              <arco.Select.Option key={'1'} value={'module}'}>模块</arco.Select.Option>
              <arco.Select.Option key={'2'} value={'func'}>功能</arco.Select.Option>
            </arco.Select>
          </arco.Form.Item>
        </arco.Grid.Col>
        <arco.Grid.Col span={24}>
          <arco.Form.Item field="menuUrl" label='访问地址' rules={[{ required: true, message: '访问地址不能为空' }, { maxLength: 256, message: '访问地址最长256' }]}>
            <arco.Input placeholder='请输入访问地址' />
          </arco.Form.Item>
        </arco.Grid.Col>
        <arco.Grid.Col span={24}>
          <arco.Form.Item field="recordNum" label='显示顺序' allowClear rules={[{ required: true, message: '显示顺序不能为空' }]}>
            <arco.InputNumber placeholder='顺序' />
          </arco.Form.Item>
        </arco.Grid.Col>
        <arco.Grid.Col span={24}>
          <arco.Form.Item field="recordRemark" label='菜单说明' allowClear rules={[{ maxLength: 255, message: '菜单说明最长255' }]}>
            <arco.Input.TextArea placeholder='输入菜单说明' style={{ minHeight: 64 }} />
          </arco.Form.Item>
        </arco.Grid.Col>
      </arco.Grid.Row>
    </arco.Form>
  </div >;
}

const EditMenuDialog = (props) => {
  return <arco.Modal
    title={props.title}
    visible={props.visible}
    autoFocus={false}
    focusLock={true}
    onOk={() => {
      AppIns.menuEditFormRef.current.validate().then((formData) => {
        saveMenuInfo(formData, AppIns.setEditMenuFormShow);
        AppIns.menuEditFormRef.current.resetFields();
      }).catch((_) => {
      });
    }}
    onCancel={() => {
      AppIns.menuEditFormRef.current.resetFields();
      AppIns.setEditMenuFormShow(false);
    }}
  >
    <EditMenuFormView />
  </arco.Modal>
}

const EditMenuFormView = () => {
  React.useEffect(() => {
    if (AppIns.initMenuValue) {
      if (AppIns.initMenuValue.recordId) {
        onQueryMenuInfo(AppIns.initMenuValue.recordId, AppIns.menuEditFormRef.current);
      } else {
        AppIns.menuEditFormRef.current.setFieldsValue(AppIns.initMenuValue);
      }
    }
  }, [AppIns.initMenuValue]);
  return <div>
    <arco.Form ref={AppIns.menuEditFormRef}>
      <arco.Grid.Row gutter={24}>
        <arco.Grid.Col span={24}>
          <arco.Form.Item field="recordId" label='主键' hidden={true}>
            <arco.Input placeholder='' />
          </arco.Form.Item>
          <arco.Form.Item field="menuParentCode" label='上级菜单编码' hidden={true}>
            <arco.Input placeholder='' />
          </arco.Form.Item>
        </arco.Grid.Col>
        <arco.Grid.Col span={24}>
          <arco.Form.Item field="menuTitle" label='菜单名称' rules={[{ required: true, message: '菜单名称不能为空' }, { minLength: 2, maxLength: 32, message: '应用编码长度2到32' }]}>
            <arco.Input placeholder='请输入菜单名称' />
          </arco.Form.Item>
        </arco.Grid.Col>
        <arco.Grid.Col span={24}>
          <arco.Form.Item field="menuType" label='菜单类型' allowClear rules={[{ required: true, message: '菜单类型不能为空' }]} >
            <arco.Select placeholder='请选择菜单类型'>
              <arco.Select.Option key={'1'} value={'module}'}>模块</arco.Select.Option>
              <arco.Select.Option key={'2'} value={'func'}>功能</arco.Select.Option>
            </arco.Select>
          </arco.Form.Item>
        </arco.Grid.Col>
        <arco.Grid.Col span={24}>
          <arco.Form.Item field="menuUrl" label='访问地址' rules={[{ required: true, message: '访问地址不能为空' }, { maxLength: 256, message: '访问地址最长256' }]}>
            <arco.Input placeholder='请输入访问地址' />
          </arco.Form.Item>
        </arco.Grid.Col>
        <arco.Grid.Col span={24}>
          <arco.Form.Item field="recordNum" label='显示顺序' allowClear rules={[{ required: true, message: '显示顺序不能为空' }]}>
            <arco.InputNumber placeholder='顺序' />
          </arco.Form.Item>
        </arco.Grid.Col>
        <arco.Grid.Col span={24}>
          <arco.Form.Item field="recordRemark" label='菜单说明' allowClear rules={[{ maxLength: 255, message: '菜单说明最长255' }]}>
            <arco.Input.TextArea placeholder='输入菜单说明' style={{ minHeight: 64 }} />
          </arco.Form.Item>
        </arco.Grid.Col>
      </arco.Grid.Row>
    </arco.Form>
  </div>;
}

const InfoMenuDialog = (props) => {
  return <arco.Modal
    title={props.title}
    visible={props.visible}
    autoFocus={false}
    focusLock={true}
    footer={null}
    onCancel={() => {
      AppIns.menuInfoFormRef.current.resetFields();
      AppIns.setInfoMenuFormShow(false);
    }}
  >
    <InfoMenuFormView />
  </arco.Modal>
}

const InfoMenuFormView = () => {
  React.useEffect(() => {
    if (AppIns.initMenuValue) {
      if (AppIns.initMenuValue.recordId) {
        onQueryMenuInfo(AppIns.initMenuValue.recordId, AppIns.menuInfoFormRef.current);
      } else {
        AppIns.menuInfoFormRef.current.setFieldsValue(AppIns.initMenuValue);
      }
    }
  }, [AppIns.initMenuValue]);
  return <div>
    <arco.Form ref={AppIns.menuInfoFormRef} disabled={true}>
      <arco.Grid.Row gutter={24}>
        <arco.Grid.Col span={24}>
          <arco.Form.Item field="recordId" label='主键' hidden={true}>
            <arco.Input placeholder='' />
          </arco.Form.Item>
          <arco.Form.Item field="menuParentCode" label='上级菜单编码' hidden={true}>
            <arco.Input placeholder='' />
          </arco.Form.Item>
        </arco.Grid.Col>
        <arco.Grid.Col span={24}>
          <arco.Form.Item field="menuTitle" label='菜单名称' rules={[{ required: true, message: '菜单名称不能为空' }, { minLength: 2, maxLength: 32, message: '应用编码长度2到32' }]}>
            <arco.Input placeholder='请输入菜单名称' />
          </arco.Form.Item>
        </arco.Grid.Col>
        <arco.Grid.Col span={24}>
          <arco.Form.Item field="menuType" label='菜单类型' allowClear rules={[{ required: true, message: '菜单类型不能为空' }]} >
            <arco.Select placeholder='请选择菜单类型'>
              <arco.Select.Option key={'1'} value={'module}'}>模块</arco.Select.Option>
              <arco.Select.Option key={'2'} value={'func'}>功能</arco.Select.Option>
            </arco.Select>
          </arco.Form.Item>
        </arco.Grid.Col>
        <arco.Grid.Col span={24}>
          <arco.Form.Item field="menuUrl" label='访问地址' rules={[{ required: true, message: '访问地址不能为空' }, { maxLength: 256, message: '访问地址最长256' }]}>
            <arco.Input placeholder='请输入访问地址' />
          </arco.Form.Item>
        </arco.Grid.Col>
        <arco.Grid.Col span={24}>
          <arco.Form.Item field="recordNum" label='显示顺序' allowClear rules={[{ required: true, message: '显示顺序不能为空' }]}>
            <arco.InputNumber placeholder='顺序' />
          </arco.Form.Item>
        </arco.Grid.Col>
        <arco.Grid.Col span={24}>
          <arco.Form.Item field="recordRemark" label='菜单说明' allowClear rules={[{ maxLength: 255, message: '菜单说明最长255' }]}>
            <arco.Input.TextArea placeholder='输入菜单说明' style={{ minHeight: 64 }} />
          </arco.Form.Item>
        </arco.Grid.Col>
      </arco.Grid.Row>
    </arco.Form>
  </div>;
}


const ChangeParentMenuDialog = (props) => {
  return <arco.Modal
    title={props.title}
    visible={props.visible}
    autoFocus={false}
    focusLock={true}
    onOk={() => {
      let recordId = AppIns.initMenuValue.recordId;
      let menuParentCode = AppIns.initMenuValue.menuParentCode;
      onSaveMenuParentChange(recordId, menuParentCode);
      AppIns.setInitMenuValue({});
      AppIns.setTreeParentData([]);
      AppIns.setChangeParentMenuShow(false);
    }}
    onCancel={() => {
      // 清空初始值
      AppIns.setInitMenuValue({});
      AppIns.setTreeParentData([]);
      AppIns.setChangeParentMenuShow(false);
    }}
  >
    <ParentMenuTree />
  </arco.Modal>
}
const ParentMenuTree = () => {
  React.useEffect(() => {
    if (AppIns.changeParentMenuShow) {
      onQueryTreeDataWithOutRecordId();
    }
  }, [AppIns.changeParentMenuShow]);
  const treeFiledNames = {
    key: 'menuCode',
    title: 'menuTitle',
    children: 'menuChildren',
  };
  return <arco.Tree ref={AppIns.parentMenuTreeRef} showLine blockNode multiple={false} checkStrictly={true} fieldNames={treeFiledNames} treeData={AppIns.treeParentData}
    onSelect={(keys, evet) => {
      let key = keys[0];
      AppIns.setInitMenuValue({ menuParentCode: key, recordId: AppIns.initMenuValue.recordId });
    }}
  ></arco.Tree>;
}
