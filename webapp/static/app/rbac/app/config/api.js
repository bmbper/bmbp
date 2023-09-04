const PageApi = {
  appMenuValueUrl: '/rbac/v1/menu/index.view',
  queryInfoUrl: "/rbac/v1/app/find/info/",
}

const goBackAppBaseView = () => {
  window.parent.postMessage({ from: 'menu', recordId: null }, "*");
}

const onQueryFormInfo = (recordId, callback) => {
  BmbpHttp.post(PageApi.queryInfoUrl + recordId, {}).then((resp) => {
    if (resp.code == 0) {
      callback(resp.data);
    } else {
      arco.Message.error(resp.msg);
    }
  });
}
