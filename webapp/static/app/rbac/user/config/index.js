const PageContext = {
};
function PageView() {

  const [infoFormRef, setInfoFormRef] = React.useState(React.useRef());
  PageContext.infoFormRef = infoFormRef;
  PageContext.setInfoFormRef = setInfoFormRef;

  return <arco.Tabs defaultActiveTab='1' extra={
    <arco.Button size='mini' type='secondary' onClick={goBackBaseView}>
      返回
    </arco.Button>
  }>
    <arco.Tabs.TabPane key='1' title='分配角色'>
      <div>分配角色</div>
    </arco.Tabs.TabPane>
    <arco.Tabs.TabPane key='2' title='认证信息'>
      <div>认证信息</div>
    </arco.Tabs.TabPane>
    <arco.Tabs.TabPane key='3' title='人员信息'>
      <div>关联人员信息</div>
    </arco.Tabs.TabPane>
    <arco.Tabs.TabPane key='4' title='基本信息'>
      <div>角色基本信息</div>
    </arco.Tabs.TabPane>
  </arco.Tabs>
}
