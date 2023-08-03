const AppAddForm = (props) => {
  return <arco.Modal
    title={props.title}
    visible={props.visible}
    autoFocus={false}
    focusLock={true}
    onOk={() => AppPageIns.setAddFormVisible(false)}
    onCancel={() => AppPageIns.setAddFormVisible(false)}
  >
    <div> Add Form:{props.recordId}</div>
  </arco.Modal>
}

const AppEditForm = (props) => {
  return <arco.Modal
    title={props.title}
    visible={props.visible}
    autoFocus={false}
    focusLock={true}
    onOk={() => AppPageIns.setEditFormVisible(false)}
    onCancel={() => AppPageIns.setEditFormVisible(false)}
  >
    <div> edit Form:{props.recordId}</div>
  </arco.Modal>
}


const AppInfoForm = (props) => {
  return <arco.Modal
    title={props.title}
    visible={props.visible}
    autoFocus={false}
    focusLock={true}
    onOk={() => AppPageIns.setInfoFormVisible(false)}
    onCancel={() => AppPageIns.setInfoFormVisible(false)}
  >
    <div> Info Form:{props.recordId}</div>
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
