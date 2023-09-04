const PageApi = {
  baseViewUrl: '/rbac/v1/role/base.view',
  confViewUrl: '/rbac/v1/role/config.view?roleId='
}
const handIframeMsg = (msg) => {
  let pageIFrameSrc = PageApi.baseViewUrl;
  if (msg.data.from == "role") {
    pageIFrameSrc = PageApi.confViewUrl + msg.data.roleId;
  }
  PageContext.setPageSrc(pageIFrameSrc);
}
