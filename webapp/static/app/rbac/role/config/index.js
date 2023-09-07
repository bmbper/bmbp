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
    <arco.Tabs.TabPane key='1' title='应用菜单资源'>
      <div>分配应用菜单</div>
    </arco.Tabs.TabPane>
    <arco.Tabs.TabPane key='2' title='应用接口资源'>
      <div>分配应用接口</div>
    </arco.Tabs.TabPane>
    <arco.Tabs.TabPane key='3' title='应用数据资源'>
      <div>分配应用数据</div>
    </arco.Tabs.TabPane>
    <arco.Tabs.TabPane key='4' title='分配用户' >
      <div>分配用户</div>
    </arco.Tabs.TabPane>
    <arco.Tabs.TabPane key='5' title='互斥角色' >
      <div>互斥角色</div>
    </arco.Tabs.TabPane>
    <arco.Tabs.TabPane key='6' title='角色基本信息'>
      <div>角色基本信息</div>
    </arco.Tabs.TabPane>
  </arco.Tabs>
}
