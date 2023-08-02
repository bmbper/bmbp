const doLogin = (formData) => {
  axios.post('/api/v1/login.do', formData)
    .then(function(resp) {
      const loginData = resp.data;
      if (loginData.code == 0) {
        let userInfo = loginData.data;
        let token = userInfo.token;
        window.location.href = "/portal.view?token=" + token;
      } else {
        arco.Message.error(loginData.message);
      }
    })
    .catch(function(error) {
      arco.Message.error(error.message);
    });
}

// 登录表单
function LoginForm() {
  const formRef = React.useRef();
  const submitClick = async () => {
    formRef.current.validate().then((formData) => {
      doLogin(formData);
    }).catch((_) => {
    });

  }
  return <div className="bmbp-login-form">
    <arco.Form ref={formRef} autoComplete='off'>
      <arco.Form.Item field="username" rules={[{ required: true, message: '用户名不能为空' }, { minLength: 6, maxLength: 15, message: '用户名长度大于5小于15' }]}>
        <arco.Input className="bmbp-login-input" placeholder='用户名' />
      </arco.Form.Item>
      <arco.Form.Item field="password" rules={[{ required: true, message: '密码不能为空' }, { minLength: 6, maxLength: 15, message: '密码长度大于5小于15' }]}>
        <arco.Input.Password className="bmbp-login-input" placeholder='密码' />
      </arco.Form.Item>
      <arco.Form.Item>
        <arco.Button className="bmbp-login-submit" onClick={submitClick}>登录</arco.Button>
      </arco.Form.Item>
    </arco.Form>
  </div>;
}

function LoginView() {
  return <div className="bmbp-login-body">
    <LoginForm />
  </div>;
}
