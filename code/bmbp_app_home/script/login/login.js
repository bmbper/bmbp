const LoginToolBar = () => {
    const loginButton = React.createElement(arco.Button, {
        onClick: () => {
            alert("cccc");
        }
    }, "Login");
    const cancelButton = React.createElement(arco.Button, {
        onClick: () => {
            alert("cccc");
        }
    }, "Cancel")
    const toolbar = React.createElement('div', {}, loginButton, cancelButton);
    return toolbar;
}
const LoginApp = () => {
    const toolbar = React.createElement(LoginToolBar, {}, null);
    return toolbar;
}