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
  let fields = [
    React.createElement(arco.Form.Item, { label: '应用编码', field: 'code' }, React.createElement(arco.Input, { palaceHolder: '请输入应用名称' }, null)),
  ];

  return React.createElement(arco.Card, { className: "form-card", title: '应用基本信息' }, ...fields);

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
