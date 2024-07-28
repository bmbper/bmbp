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
    React.createElement(arco.Card, { className: "form-card", title: '应用基本信息' }, null),
    React.createElement(arco.Card, { className: "form-card", title: '应用服务信息' }, null),
    React.createElement(arco.Card, { className: "form-card", title: '数据源信息' }, null),
    React.createElement(FormBottomBar, { className: "form-card", }, null),
  ];

  return React.createElement(arco.Form, {}, formCard);
}
const FormBottomBar = () => {
  const okBtn = React.createElement(arco.Button, {}, '保存');
  const cancelBtn = React.createElement(arco.Button, {}, '取消');
  const space = React.createElement(arco.Space, {}, okBtn, cancelBtn)
  return React.createElement('div', {}, space);
}
