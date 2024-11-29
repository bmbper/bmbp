import {HomeAction, HomeState} from "./action";

window.onload = () => {
    const root = ReactDOM.createRoot(document.getElementById("app"));
    root.render(<LoginView/>);
}

const LoginView = () => {
    HomeAction.init();
    return (
        <>
            <div className="bmbp_login">
                <div className={"login_left"}>
                    <div className={'login_image'}>
                        <img src={"./../static/bmbp_app_auth/image/login.jpeg"}></img>
                    </div>
                </div>
                <div className={"login_right"}>
                    <div className={"login_form"}>
                        <div className={'form_container'}>
                            <div className={"title"}>
                                {appLoginName}
                            </div>
                            <div className={"form"}>
                                <FormView></FormView>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </>
    )
}
const FormView = () => {
    return (<>
        <arco.Form ref={HomeState.loginRef} style={{width: "100%"}} autoComplete='off' className={"login_form_body"}>
            <arco.Form.Item label='用户名' field={'username'} className={"login_form_field"}
                            rules={[{required: true, message: "用户名不能为空"}]}>
                <arco.Input placeholder=''/>
            </arco.Form.Item>
            <arco.Form.Item label='密码' field={'password'} className={"login_form_field"}
                            rules={[{required: true, message: "密码不能为空"}]}>
                <arco.Input type="password" placeholder=''/>
            </arco.Form.Item>
            <arco.Form.Item label=" " className={"login_form_field"}>
                <arco.Button type='primary' className={"login_form_button"} onClick={() => {
                    HomeAction.doLogin();
                }}>登录
                </arco.Button>
            </arco.Form.Item>
        </arco.Form>
    </>)
}
