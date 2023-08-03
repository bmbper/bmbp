
const logoClick = () => {
  alert("/login.click")
}


const queryMenuData = () => {

  return [
    {
      recordId: '0',
      menuTitle: "首页",
      menuUrl: '/index.view',
      menuType: 'func',
    },
    {
      recordId: '1',
      menuTitle: "权限管理",
      menuUrl: '',
      children: [{
        recordId: '1.1',
        menuTitle: "应用管理",
        menuUrl: '/rbac/v1/app/index.view',
        menuType: 'func'
      }, {
        recordId: '1.2',
        menuTitle: "组织管理",
        menuUrl: '',
      }, {
        recordId: '1.3',
        menuTitle: "用户管理",
        menuUrl: '',
      }
      ]
    },
    {
      recordId: '2',
      menuTitle: "XX管理",
      menuUrl: '',
      children: [{
        recordId: '2.1',
        menuTitle: "CC管理",
        menuUrl: '',
        children: [
          {
            recordId: '2.1.1',
            menuTitle: "CC管理-子菜单",
            menuUrl: '',
          }

        ]
      }, {
        recordId: '2.2',
        menuTitle: "二级功能",
        menuUrl: '',
        menuType: 'func'
      }
      ]
    },
    {
      recordId: '3',
      menuTitle: "功能管理",
      menuUrl: '',
      menuType: 'func',
      children: []
    }
  ]

}

function subMenuCom(data) {
  debugger;
  return data.children.map((subMenu, index) => {
    <MenuItem key={subMenu.recordId}>{subMenu.menuTitle}</MenuItem>
  });
}

function BmbpMenu() {
  const Menu = arco.Menu;
  const SubMenu = arco.Menu.SubMenu;
  const MenuItem = arco.Menu.Item;
  const menuData = queryMenuData();
  return <Menu>
    {
      /// 三级菜单渲染
      menuData.map((menu) => (
        menu.menuType == 'func' ? <MenuItem key={menu.recordId}>{menu.menuTitle}</MenuItem> : <SubMenu key={menu.recordId} title={menu.menuTitle}>{
          menu.children ? menu.children.map((sub) => (
            sub.menuType == 'func' ? <MenuItem key={sub.recordId}>{sub.menuTitle}</MenuItem> : <SubMenu key={sub.recordId} title={sub.menuTitle}>{
              sub.children ? sub.children.map((third) => (
                <MenuItem key={third.recordId}>
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
        Bmbp开发运维护系统
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
        <arco.Breadcrumb.Item>系统管理</arco.Breadcrumb.Item>
        <arco.Breadcrumb.Item>
          权限管理
        </arco.Breadcrumb.Item>
        <arco.Breadcrumb.Item>应用管理</arco.Breadcrumb.Item>
      </arco.Breadcrumb>
    </div>
    <div className="bmbp-layout-center-body">
      <iframe className="bmbp-layout-center-body-content" src="/rbac/v1/app/index.view" />
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
// 主页面的APPView
function PortalView() {
  return <FullPage />;
}
