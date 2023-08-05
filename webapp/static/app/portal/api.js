const queryMenuData = () => {

  return [
    {
      recordId: '0',
      menuTitle: "工作台",
      menuUrl: '/portal/index.view',
      menuType: 'func',
      menuTitlePath: '/工作台/'
    },
    {
      recordId: '1',
      menuTitle: "权限管理",
      menuUrl: '',
      children: [{
        recordId: '1.1',
        menuTitle: "应用管理",
        menuUrl: '/rbac/v1/app/index.view',
        menuType: 'func',
        menuTitlePath: '/权限管理/应用管理/'
      },
      {
        recordId: '1.2',
        menuTitle: "菜单管理",
        menuUrl: '/rbac/v1/res/menu/index.view',
        menuType: 'func',
        menuTitlePath: '/权限管理/菜单管理/'
      }, {
        recordId: '1.3',
        menuTitle: "组织管理",
        menuUrl: '/rbac/v1/organ/index.view',
        menuType: 'func',
        menuTitlePath: '/权限管理/组织管理/'
      }, {
        recordId: '1.4',
        menuTitle: "用户管理",
        menuUrl: '/rbac/v1/user/index.view',
        menuType: 'func',
        menuTitlePath: '/权限管理/用户管理/'
      }, {
        recordId: '1.5',
        menuTitle: "角色管理",
        menuUrl: '/rbac/v1/role/index.view',
        menuType: 'func',
        menuTitlePath: '/权限管理/角色管理/'
      },

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
