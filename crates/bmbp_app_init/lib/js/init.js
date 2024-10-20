//DOM 加载完成后执行 初始化方法
window.onload = () => {
  const root = ReactDOM.createRoot(document.getElementById("app"));
  const app = React.createElement(InitView, {}, null);
  root.render(app);
};

// 应用初始化视图
const InitView = () => {
  let formTitle = React.createElement(FormTitle, {}, null);
  let formBody = React.createElement(FormBody, {}, null);
  return React.createElement(
    "div",
    {
      class: "bmbp-app-container",
    },
    formTitle,
    formBody,
  );
};

const FormTitle = () => {
  const h1 = React.createElement("h3", {}, "系统初化配置");
  return React.createElement(
    "div",
    {
      style: {
        textAlign: "center",
      },
    },
    h1,
  );
};
const FormBody = () => {
  let formCard = [
    React.createElement(BaseFormCard, {}, null),
    React.createElement(ServerFormCard, {}, null),
    React.createElement(DataSourceFormCard, {}, null),
    React.createElement(FormBottomBar, { className: "form-card" }, null),
  ];
  return React.createElement("div", {}, ...formCard);
};

const BaseFormCard = () => {
  const code = React.createElement(
    arco.Form.Item,
    {
      label: "应用编码",
      field: "code",
      rules: [
        {
          required: true,
          message: "应用编码不能为空",
        },
        {
          maxLength: 16,
          message: "应用编码最长16字符",
        },
        {
          minLength: 2,
          message: "应用编码最少2字符",
        },
      ],
    },
    React.createElement(arco.Input, { palaceHolder: "请输入应用编码" }, null),
  );
  const name = React.createElement(
    arco.Form.Item,
    {
      label: "应用名称",
      field: "name",
      rules: [
        {
          required: true,
          message: "应用名称不能为空",
        },
        {
          maxLength: 32,
          message: "应用编码最长32字符",
        },
        {
          minLength: 2,
          message: "应用编码最少2字符",
        },
      ],
    },
    React.createElement(arco.Input, { palaceHolder: "请输入应用名称" }, null),
  );
  const loginName = React.createElement(
    arco.Form.Item,
    {
      label: "登录标题",
      field: "login_name",
      rules: [
        {
          required: true,
          message: "登录标题不能为空",
        },
        {
          maxLength: 16,
          message: "登录标题最长16字符",
        },
        {
          minLength: 2,
          message: "应用编码最少2字符",
        },
      ],
    },
    React.createElement(arco.Input, { palaceHolder: "请输入登录标题" }, null),
  );
  const navName = React.createElement(
    arco.Form.Item,
    {
      label: "导航标题",
      field: "nav_name",
      rules: [
        {
          required: true,
          message: "导航标题不能为空",
        },
        {
          maxLength: 8,
          message: "登录标题最长8字符",
        },
        {
          minLength: 2,
          message: "应用编码最少2字符",
        },
      ],
    },
    React.createElement(arco.Input, { palaceHolder: "请输入导航标题" }, null),
  );
  const copyRight = React.createElement(
    arco.Form.Item,
    {
      field: "copy_right",
      label: "版权信息",
      rules: [
        {
          required: true,
          message: "版权信息不能为空",
        },
        {
          maxLength: 64,
          message: "版权信息最长64字符",
        },
        {
          minLength: 16,
          message: "应用编码最少16字符",
        },
      ],
    },
    React.createElement(arco.Input, { palaceHolder: "请输入版权信息" }, null),
  );
  let cols = [
    React.createElement(arco.Grid.Col, { span: 12 }, code),
    React.createElement(arco.Grid.Col, { span: 12 }, name),
    React.createElement(arco.Grid.Col, { span: 12 }, loginName),
    React.createElement(arco.Grid.Col, { span: 12 }, navName),
    React.createElement(arco.Grid.Col, { span: 24 }, copyRight),
  ];
  let rows = [React.createElement(arco.Grid.Row, { gutter: 12 }, ...cols)];

  let baseCard = React.createElement(
    arco.Card,
    { className: "form-card", title: "应用基本信息" },
    ...rows,
  );
  window.baseFormRef = React.useRef();
  return React.createElement(
    arco.Form,
    {
      ref: window.baseFormRef,
      layout: "vertical",
      initialValues: getAppValue()
    },
    baseCard,
  );
};

const ServerFormCard = () => {
  const envSelect = React.createElement(
    arco.Select,
    { allowClear: true, palaceHolder: "请选择运行环境" },
    ...createOptions(envOptions),
  );
  const logLevelSelect = React.createElement(
    arco.Select,
    { allowClear: true, palaceHolder: "请选择日志级别" },
    ...createOptions(logLevelOptions),
  );

  const serverHost = React.createElement(
    arco.Form.Item,
    {
      label: "服务Host",
      field: "host",
      rules: [
        {
          required: true,
          message: "服务Host不能为空",
        },
        {
          maxLength: 64,
          message: "版权信息最长64字符",
        },
        {
          minLength: 8,
          message: "应用编码最少7字符",
        },
      ],
    },
    React.createElement(arco.Input, { palaceHolder: "请输入服务IP" }, null),
  );
  const serverPort = React.createElement(
    arco.Form.Item,
    {
      label: "服务端口",
      field: "port",
      rules: [
        {
          required: true,
          message: "服务端口不能为空",
        }
      ],
    },
    React.createElement(
      arco.InputNumber,
      { palaceHolder: "请输入服务端口", min: 10000, max: 40000 },
      null,
    ),
  );
  const serverEnv = React.createElement(
    arco.Form.Item,
    {
      label: "运行环境",
      field: "env",
      rules: [
        {
          required: true,
          message: "运行环境不能为空",
        },
      ],
    },
    envSelect,
  );
  const serverLogLevel = React.createElement(
    arco.Form.Item,
    {
      label: "日志级别",
      field: "log_level",
      rules: [
        {
          required: true,
          message: "日志级别不能为空",
        },
      ],
    },
    logLevelSelect,
  );
  let cols = [
    React.createElement(arco.Grid.Col, { span: 6 }, serverHost),
    React.createElement(arco.Grid.Col, { span: 6 }, serverPort),
    React.createElement(arco.Grid.Col, { span: 6 }, serverEnv),
    React.createElement(arco.Grid.Col, { span: 6 }, serverLogLevel),
  ];
  let rows = [React.createElement(arco.Grid.Row, { gutter: 12 }, ...cols)];
  let serverCard = React.createElement(
    arco.Card,
    { className: "form-card", title: "应用服务信息" },
    ...rows,
  );
  window.serverFormRef = React.useRef();
  return React.createElement(
    arco.Form,
    {
      ref: window.serverFormRef,
      layout: "vertical",
      initialValues: getServerValue()
    },
    serverCard,
  );
};

const DataSourceFormCard = () => {
  const dataBaseType = React.createElement(
    arco.Form.Item,
    {
      label: "数据库类型",
      field: "driver",
      rules: [
        {
          required: true,
          message: "数据库类型不能为空",
        },
      ],
    },
    React.createElement(
      arco.Select,
      { palaceHolder: "请选择数据库类型" },
      ...createOptions(dataBaseOptions),
    ),
  );
  const dataBaseHost = React.createElement(
    arco.Form.Item,
    {
      label: "数据库地址",
      field: "host",
      rules: [
        {
          required: true,
          message: "数据库地址不能为空",
        },
        {
          maxLength: 128,
          message: "数据库地址最长128字符",
        },
        {
          minLength: 8,
          message: "数据库地址最少7字符",
        },
      ],
    },
    React.createElement(arco.Input, { palaceHolder: "请输入数据库IP" }, null),
  );
  const dataBasePort = React.createElement(
    arco.Form.Item,
    {
      label: "数据库端口",
      field: "port",
      rules: [
        {
          required: true,
          message: "数据库端口不能为空",
        }
      ],
    },
    React.createElement(
      arco.InputNumber,
      { palaceHolder: "请输入数据库端口", min: 1000, max: 65535 },
      null,
    ),
  );
  const dataBaseName = React.createElement(
    arco.Form.Item,
    {
      label: "数据库名称",
      field: "database",
      rules: [
        {
          minLength: 1,
          message: "数据库名称长度大于1",
        },
        {
          maxLength: 32,
          message: "数据库名称长度小于32",
        },
      ],
    },
    React.createElement(arco.Input, { palaceHolder: "请输入数据库" }, null),
  );
  const dataBaseChema = React.createElement(
    arco.Form.Item,
    {
      label: "模式",
      field: "schema",
      rules: [
        {
          minLength: 1,
          message: "模式名称长度大于1",
        },
        {
          maxLength: 32,
          message: "模式名称长度小于32",
        },
      ],
    },
    React.createElement(arco.Input, { palaceHolder: "请输入数据库模式" }, null),
  );
  const dataBaseUsername = React.createElement(
    arco.Form.Item,
    {
      label: "用户名",
      field: "username",
      rules: [
        {
          required: true,
          message: "用户名不能为空",
        },
        {
          maxLength: 128,
          message: "用户名最长128字符",
        },
        {
          minLength: 4,
          message: "用户名最少7字符",
        },
      ],
    },
    React.createElement(arco.Input, { palaceHolder: "请输入数据库用户" }, null),
  );
  const dataBasePassword = React.createElement(
    arco.Form.Item,
    {
      label: "密码",
      field: "password",
      rules: [
        {
          required: true,
          message: "密码不能为空",
        },
        {
          maxLength: 128,
          message: "密码最长128字符",
        },
        {
          minLength: 8,
          message: "密码最少7字符",
        },
      ],
    },
    React.createElement(
      arco.Input.Password,
      { palaceHolder: "请输入数据库密码" },
      null,
    ),
  );
  const dataBaseCaseSensitve = React.createElement(
    arco.Form.Item,
    { label: "忽略大小写", field: "ignore_case" },
    React.createElement(
      arco.Checkbox,
      { palaceHolder: "请选择忽略大小写" },
      "是",
    ),
  );
  const connInit = React.createElement(
    arco.Form.Item,
    {
      label: "初始连接数",
      field: "init_size",
      rules: [
        {
          required: true,
          message: "初始连接数不能为空",
        },
      ],
    },
    React.createElement(
      arco.InputNumber,
      { palaceHolder: "请输入初始连接数", min: 1, max: 10000 },
      null,
    ),
  );
  const maxSize = React.createElement(
    arco.Form.Item,
    {
      label: "最大连接数",
      field: "max_size",

    },
    React.createElement(
      arco.InputNumber,
      { palaceHolder: "请输入最大连接数", min: 1, max: 10000 },
      null,
    ),
  );
  const maxIdel = React.createElement(
    arco.Form.Item,
    {
      label: "最大空闲连接数",
      field: "max_idle",

    },
    React.createElement(
      arco.InputNumber,
      { palaceHolder: "请输入最大空闲连接数", min: 1, max: 10000 },
      null,
    ),
  );
  const minIdel = React.createElement(
    arco.Form.Item,
    {
      label: "最小空闲连接数",
      field: "min_idle",

    },
    React.createElement(
      arco.InputNumber,
      { palaceHolder: "请输入最小空闲连接数", min: 1, max: 10000 },
      null,
    ),
  );
  let cols = [
    React.createElement(arco.Grid.Col, { span: 6 }, dataBaseType),
    React.createElement(arco.Grid.Col, { span: 6 }, dataBaseHost),
    React.createElement(arco.Grid.Col, { span: 6 }, dataBasePort),
    React.createElement(arco.Grid.Col, { span: 6 }, dataBaseName),
    React.createElement(arco.Grid.Col, { span: 6 }, dataBaseChema),
    React.createElement(arco.Grid.Col, { span: 6 }, dataBaseUsername),
    React.createElement(arco.Grid.Col, { span: 6 }, dataBasePassword),
    React.createElement(arco.Grid.Col, { span: 6 }, dataBaseCaseSensitve),
    React.createElement(arco.Grid.Col, { span: 6 }, connInit),
    React.createElement(arco.Grid.Col, { span: 6 }, maxSize),
    React.createElement(arco.Grid.Col, { span: 6 }, minIdel),
    React.createElement(arco.Grid.Col, { span: 6 }, maxIdel),
  ];
  let rows = [React.createElement(arco.Grid.Row, { gutter: 12 }, ...cols)];
  let datasourceCard = React.createElement(
    arco.Card,
    { className: "form-card", title: "数据源信息" },
    ...rows,
  );
  window.datasourceFormRef = React.useRef();
  return React.createElement(
    arco.Form,
    {
      ref: window.datasourceFormRef,
      layout: "vertical",
      initialValues: getDatasourceValue()
    },
    datasourceCard,
  );
};

const FormBottomBar = () => {
  const okBtn = React.createElement(
    arco.Button,
    {
      type: "primary",
      onClick: (e) => {
        saveFormData();
      },
    },
    "保存",
  );
  const cancelBtn = React.createElement(
    arco.Button,
    {
      onClick: () => {
        clearFormData();
      },
    },
    "取消",
  );
  const space = React.createElement(arco.Space, {}, okBtn, cancelBtn);
  return React.createElement(
    "div",
    {
      style: {
        textAlign: "center",
      },
    },
    space,
  );
};

/// 字典数据
const envOptions = [
  {
    label: "生产环境",
    value: "prod",
  },
  {
    label: "开发环境",
    value: "dev",
  },
  {
    label: "测试环境",
    value: "test",
  },
];

const logLevelOptions = [
  {
    label: "调试",
    value: "debug",
  },
  {
    label: "日志",
    value: "info",
  },
  {
    label: "警告",
    value: "warn",
  },
  {
    label: "错误",
    value: "error",
  },
];
const dataBaseOptions = [
  {
    label: "MYSQL",
    value: "mysql",
  },
  {
    label: "Postgres",
    value: "postgres",
  },
];

const createOptions = (options) => {
  let view = [];
  for (item of options) {
    view.push(
      React.createElement(
        arco.Select.Option,
        {
          key: item.value,
          value: item.value,
        },
        item.label,
      ),
    );
  }
  return view;
};

/// 保存配置信息数据
const saveFormData = () => {
  let bmbpData = {};
  window.baseFormRef.current.validate().then((baseData) => {
    bmbpData.app = baseData;
    window.serverFormRef.current.validate().then((serverData) => {
      bmbpData.server = serverData;
      window.datasourceFormRef.current.validate().then((datasourceData) => {
        bmbpData.datasource = datasourceData;
        // TODO 增加数据源有效性验证
        let bmbpConfigData = {
          bmbp: bmbpData,
        };
        axios
          .post("/init/save/config.do", bmbpConfigData, {
            headers: {
              "Content-Type": "application/json",
            },
          })
          .then((resp) => {
            // TODO 增加重启等待框
            if (resp.code == 0) {
              let host = serverData.host + ":" + serverData.port
              arco.Notification.success({
                id: 'need_update',
                title: '服务重启中',
                duration: 500000,
                content: React.createElement('span', {}, host, React.createElement(arco.Spin, {}, null)),
                onClose: () => {
                  arco.Message.warning("自动重启失败，请重试!");
                }
              });
              setTimeout(() => {
                axios.get(host).then(resp => {
                  if (resp.code == 0) {
                    window.location.href = host;
                  }
                });
              }, 5000);
            } else {
              arco.Message.error(resp.msg);
            }
          })
          .catch((err) => {
            arco.Message.error(err.message);
          });
      });
    });
  });
};
/// 清空表单数据
const clearFormData = () => {
  window.baseFormRef.current.resetFields();
  window.serverFormRef.current.resetFields();
  window.datasourceFormRef.current.resetFields();
};
