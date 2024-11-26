export const HomeState: { [key: string]: any } = {};
export const HomeAction = {
    init: () => {
        HomeState.loginRef = React.useRef(null);
    },
    doLogin: () => {
        HomeState.loginRef.current?.validate().then((data: any) => {
            data.password = CryptoJS.MD5(CryptoJS.SHA256(data.password).toString()).toString();
            axios.post("./login.action", data).then((resp: any) => {
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