const PageApi = {
  baseViewUrl: '/rbac/v1/user/base.view',
  confViewUrl: '/rbac/v1/user/config.view?userId='
}
const handIframeMsg = (msg) => {
  let pageIFrameSrc = PageApi.baseViewUrl;
  if (msg.data.from == "user") {
    pageIFrameSrc = PageApi.confViewUrl + msg.data.roleId;
  }
  PageContext.setPageSrc(pageIFrameSrc);
}
