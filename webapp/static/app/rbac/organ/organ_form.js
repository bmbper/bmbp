const AddRootOrganDialog = (props) => {
  return <arco.Modal
    title={props.title}
    visible={props.visible}
    autoFocus={false}
    focusLock={true}
    onOk={() => {
    }}
    onCancel={() => {
      AppIns.setAddRootOrganFormShow(false);
    }}
  >
    <OrganFormView />
  </arco.Modal>
}

const OrganFormView = () => {
  return <div>FORM</div>
}
