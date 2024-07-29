window.onload = () => {
  init();
}
const init = () => {
  const root = ReactDOM.createRoot(document.getElementById('app'));
  const app = React.createElement(InitView, {}, null);
  root.render(app);
};
const InitView = () => {
  let formTitle = React.createElement(FormTitle, {}, null);
  let formBody = React.createElement(FormBody, {}, null);
  return React.createElement('div', {
    class: 'bmbp-app-container'
  }, formTitle, formBody);
}

const FormTitle = () => {
  const h1 = React.createElement('h1', {}, '系统初化配置');
  return React.createElement('div', {}, h1);
}
const FormBody = () => {
  let formCard = [
    React.createElement(BaseFormCard, {}, null),
    React.createElement(arco.Card, { className: "form-card", title: '应用服务信息' }, null),
    React.createElement(arco.Card, { className: "form-card", title: '数据源信息' }, null),
    React.createElement(FormBottomBar, { className: "form-card", }, null),
  ];
  window.formRef = React.useRef();
  return React.createElement(arco.Form, {
    ref: window.formRef
  }, formCard);
}


const BaseFormCard = () => {
  let cols = [
    React.createElement(arco.Grid.Col, { span: 12 }, React.createElement(arco.Form.Item, { label: '应用编码', field: 'code' }, React.createElement(arco.Input, { palaceHolder: '请输入应用编码' }, null))),
    React.createElement(arco.Grid.Col, { span: 12 }, React.createElement(arco.Form.Item, { label: '应用名称', field: 'name' }, React.createElement(arco.Input, { palaceHolder: '请输入应用名称' }, null))),
    React.createElement(arco.Grid.Col, { span: 12 }, React.createElement(arco.Form.Item, { label: '登录标题', field: 'login_name' }, React.createElement(arco.Input, { palaceHolder: '请输入登录标题' }, null))),
    React.createElement(arco.Grid.Col, { span: 12 }, React.createElement(arco.Form.Item, { label: '导航标题', field: 'nav_name' }, React.createElement(arco.Input, { palaceHolder: '请输入导航标题' }, null))),
    React.createElement(arco.Grid.Col, { span: 12 }, React.createElement(arco.Form.Item, { label: '版权信息', field: 'copy_right' }, React.createElement(arco.Input, { palaceHolder: '请输入版权信息' }, null)))

  ];
  let rows = [React.createElement(arco.Grid.Row, {}, ...cols)];

  return React.createElement(arco.Card, { className: "form-card", title: '应用基本信息' }, ...rows);

}

const FormBottomBar = () => {
  const okBtn = React.createElement(arco.Button, {
    type: 'primary', onClick: (e) => {
      window.formRef.current.validate().then((values) => {
        saveFormData(values);
      })
    }
  }, '保存');
  const cancelBtn = React.createElement(arco.Button, {
    onClick: () => {
      clearFormData();
    }
  }, '取消');
  const space = React.createElement(arco.Space, {}, okBtn, cancelBtn)
  return React.createElement('div', {}, space);
}

/// 保存配置信息数据
const saveFormData = (data) => {
  arco.Message.info(JSON.stringify(data));
}
/// 清空表单数据
const clearFormData = () => {
  window.formRef.current.resetFields();
}
