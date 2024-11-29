// web/tsx/login/action.tsx
var HomeState = {};
var HomeAction = {
  init: () => {
    HomeState.loginRef = React.useRef(null);
  },
  doLogin: () => {
    HomeState.loginRef.current?.validate().then((data) => {
      data.password = CryptoJS.MD5(CryptoJS.SHA256(data.password).toString()).toString();
      axios.post("./login.action", data).then((resp) => {
        if (resp.code == 0) {
          arco.Message.success(resp.msg);
          window.localStorage.setItem("token", resp.data.token);
          window.location.href = appHomeView;
        } else {
          arco.Message.error(resp.msg);
        }
      });
    });
  }
};

// web/tsx/login/index.tsx
window.onload = () => {
  const root = ReactDOM.createRoot(document.getElementById("app"));
  root.render(/* @__PURE__ */ React.createElement(LoginView, null));
};
var LoginView = () => {
  HomeAction.init();
  return /* @__PURE__ */ React.createElement(React.Fragment, null, /* @__PURE__ */ React.createElement("div", {
    className: "bmbp_login"
  }, /* @__PURE__ */ React.createElement("div", {
    className: "login_left"
  }, /* @__PURE__ */ React.createElement("div", {
    className: "login_image"
  }, /* @__PURE__ */ React.createElement("img", {
    src: "./../static/bmbp_app_auth/image/login.jpeg"
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
    ref: HomeState.loginRef,
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
    className: "login_form_button",
    onClick: () => {
      HomeAction.doLogin();
    }
  }, "登录"))));
};
