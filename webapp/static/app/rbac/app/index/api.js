const PageApi = {
  appBaseViewUrl: '/rbac/v1/app/base.view',
  appConfigViewUrl: '/rbac/v1/app/config.view?recordId='
}
const handAppIframeMsg = (msg) => {
  let pageIFrameSrc = PageApi.appBaseViewUrl;
  if (msg.data.from == "app") {
    pageIFrameSrc = PageApi.appConfigViewUrl + msg.data.recordId;
  }
  PageContext.setPageSrc(pageIFrameSrc);
}
