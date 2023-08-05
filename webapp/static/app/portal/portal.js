
const logoClick = () => {
  alert("/login.click")
}

const onNavMenuClick = (menu) => {
  console.log(menu);
  console.log(menu.menuUrl);
  if (menu.menuUrl) {
    AppIns.setCurrentMenuPath(menu.menuUrl);
    let menuTitlePath = menu.menuTitlePath || "//";
    menuTitlePath = menuTitlePath.substring(1, menuTitlePath.length - 1);
    AppIns.setBreadItem(menuTitlePath.split("/"));
  }
}

function subMenuCom(data) {
  return data.children.map((subMenu, index) => {
    <MenuItem key={subMenu.recordId}>{subMenu.menuTitle}</MenuItem>
  });
}

function BmbpMenu() {
  const Menu = arco.Menu;
  const SubMenu = arco.Menu.SubMenu;
  const MenuItem = arco.Menu.Item;
  return <Menu>
    {
      /// 三级菜单渲染
      AppIns.menuData && AppIns.menuData.map((menu) => (
        menu.menuType == 'func' ? <MenuItem onClick={() => onNavMenuClick(menu)} key={menu.recordId}>{menu.menuTitle}</MenuItem> : <SubMenu onClick={() => onNavMenuClick(menu)} key={menu.recordId} title={menu.menuTitle}>{
          menu.children ? menu.children.map((sub) => (
            sub.menuType == 'func' ? <MenuItem onClick={() => onNavMenuClick(sub)} key={sub.recordId}>{sub.menuTitle}</MenuItem> : <SubMenu onClick={() => onNavMenuClick(sub)} key={sub.recordId} title={sub.menuTitle}>{
              sub.children ? sub.children.map((third) => (
                <MenuItem onClick={() => onNavMenuClick(third)} key={third.recordId}>
                  {third.menuTitle}
                </MenuItem>
              )) : <div></div>

            }
            </SubMenu>
          )) : <div></div>}
        </SubMenu>

      ))
    }
  </Menu >
}

const SettingMenu = (
  <arco.Menu>
    <arco.Menu.Item key='1'>应用设置</arco.Menu.Item>
    <arco.Menu.Item key='2'>参数设置</arco.Menu.Item>
  </arco.Menu>
);

const UserMenu = (
  <arco.Menu>
    <arco.Menu.Item key='1'>切换角色</arco.Menu.Item>
    <arco.Menu.Item key='2'>个人信息</arco.Menu.Item>
    <arco.Menu.Item key='2'>修改密码</arco.Menu.Item>
    <arco.Menu.Item key='4'>退出登录</arco.Menu.Item>
  </arco.Menu>
);

function HeaderPage() {
  const dotStyle = { width: 10, height: 10 };
  return <div className="bmbp-layout-header">
    <div className="bmbp-layout-header-logo">
      <div onClick={logoClick}></div>
    </div>
    <div className="bmbp-layout-header-title">
      <span>
        教学案例
      </span>
    </div>
    <div className="bmbp-layout-header-nav">
      <div className="bmbp-layout-header-nav-app">
      </div>
    </div>
    <div className="bmbp-layout-header-setting">
      <div>
        <arco.Tooltip content='帮助文档'>
          <arco.Avatar>
            <arcoicon.IconQuestion />
          </arco.Avatar>
        </arco.Tooltip>
      </div>
      <div>
        <arco.Tooltip content='源码仓库'>
          <arco.Avatar>
            <arcoicon.IconGithub />
          </arco.Avatar>
        </arco.Tooltip>
      </div>
      <div>
        <arco.Badge count={9} dot dotStyle={dotStyle}>
          <arco.Avatar>
            <arcoicon.IconNotification />
          </arco.Avatar>
        </arco.Badge>
      </div>
      <div>
        <arco.Popover position='br' content={SettingMenu}>
          <arco.Avatar><arcoicon.IconSettings /></arco.Avatar>
        </arco.Popover>
      </div>
      <div>
        <arco.Popover position='br' content={UserMenu}>
          <arco.Avatar className="bmbp-layout-header-setting-user">
            <arcoicon.IconUser />
          </arco.Avatar>
        </arco.Popover>

      </div>
    </div>
  </div >;
}
function CenterPage() {
  return <div className="bmbp-layout-center">
    <div className="bmbp-layout-center-header">
      <arco.Breadcrumb>
        {
          AppIns.breadItem && AppIns.breadItem.map(
            (item) => (
              <arco.Breadcrumb.Item>{item}</arco.Breadcrumb.Item>
            )
          )
        }
      </arco.Breadcrumb>
    </div>
    <div className="bmbp-layout-center-body">
      <iframe className="bmbp-layout-center-body-content" src={AppIns.currentMenuPath} />
    </div>
  </div>;
}

function SiderPage() {
  return <div className="bmbp-layout-sider">
    <BmbpMenu />
  </div>;
}

function FooterPage() {
  return <div className="bmbp-layout-footer">
    Copyright 2023 Bmbp
  </div>;
}

function FullPage() {
  return <div className="bmbp-layout-full">
    <HeaderPage />
    <div className="bmbp-layout-flex-row">
      <SiderPage />
      <div className="bmbp-layout-flex-column">
        <CenterPage />
        <FooterPage />
      </div>
    </div>
  </div>;
}
const AppIns = {};
// 主页面的APPView
function PortalView() {
  const [currentMenuPath, setCurrentMenuPath] = React.useState("");
  const [menuData, setMenuData] = React.useState([]);
  const [breadItem, setBreadItem] = React.useState([]);
  AppIns.currentMenuPath = currentMenuPath;
  AppIns.setCurrentMenuPath = setCurrentMenuPath;
  AppIns.menuData = menuData;
  AppIns.setMenuData = setMenuData;
  AppIns.breadItem = breadItem;
  AppIns.setBreadItem = setBreadItem;

  React.useEffect(() => {
    let tempData = queryMenuData()
    setMenuData(tempData);
    loadFirstFunc(tempData);
  }, []);

  return <FullPage />;
}

const loadFirstFunc = (menuData) => {
  if (menuData && menuData.length > 0) {
    for (let item of menuData) {
      if (item.menuType == 'func' && item.menuUrl) {
        onNavMenuClick(item);
        return true;
      } else {
        if (item.children && item.children.length > 0) {
          if (loadFirstFunc(item.children)) {
            return true;
          }
        }
      }
    }
  }
  return false;
}
