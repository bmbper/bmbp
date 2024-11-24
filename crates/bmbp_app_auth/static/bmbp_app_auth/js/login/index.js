// web/tsx/login/index.tsx
window.onload = () => {
  const root = ReactDOM.createRoot(document.getElementById("app"));
  root.render(/* @__PURE__ */ React.createElement(LoginView, null));
};
var LoginView = () => {
  return /* @__PURE__ */ React.createElement(React.Fragment, null, /* @__PURE__ */ React.createElement("div", {
    className: "bmbp_login"
  }, /* @__PURE__ */ React.createElement("div", {
    className: "login_left"
  }, /* @__PURE__ */ React.createElement("div", {
    className: "login_image"
  }, /* @__PURE__ */ React.createElement("img", {
    src: "/static/bmbp_app_auth/image/login.jpeg"
  }))), /* @__PURE__ */ React.createElement("div", {
    className: "login_right"
  }, /* @__PURE__ */ React.createElement("div", {
    className: "login_form"
  }, /* @__PURE__ */ React.createElement("div", {
    className: "form_container"
  }, /* @__PURE__ */ React.createElement("div", {
    className: "title"
  }, appLoginName), /* @__PURE__ */ React.createElement("div", {
    className: "form"
  }, /* @__PURE__ */ React.createElement(FormView, null)))))));
};
var FormView = () => {
  return /* @__PURE__ */ React.createElement(React.Fragment, null, /* @__PURE__ */ React.createElement(arco.Form, {
    style: { width: "100%" },
    autoComplete: "off",
    className: "login_form_body"
  }, /* @__PURE__ */ React.createElement(arco.Form.Item, {
    label: "用户名",
    field: "username",
    className: "login_form_field",
    rules: [{ required: true, message: "用户名不能为空" }]
  }, /* @__PURE__ */ React.createElement(arco.Input, {
    placeholder: ""
  })), /* @__PURE__ */ React.createElement(arco.Form.Item, {
    label: "密码",
    field: "password",
    className: "login_form_field",
    rules: [{ required: true, message: "密码不能为空" }]
  }, /* @__PURE__ */ React.createElement(arco.Input, {
    type: "password",
    placeholder: ""
  })), /* @__PURE__ */ React.createElement(arco.Form.Item, {
    label: " ",
    className: "login_form_field"
  }, /* @__PURE__ */ React.createElement(arco.Button, {
    type: "primary",
    className: "login_form_button"
  }, "登录"))));
};
