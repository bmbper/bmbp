const PageContext = {
};
const PageView = () => {
  const [pageSrc, setPageSrc] = React.useState("/rbac/v1/app/base.view");
  PageContext.pageSrc = pageSrc;
  PageContext.setPageSrc = setPageSrc;
  const [iframeRef, setIframeRef] = React.useState(React.useRef());
  PageContext.iframeRef = iframeRef;
  PageContext.setIframeRef = setIframeRef;
  window.addEventListener("message", handAppIframeMsg, false);
  return <IFrameView />;
}

const IFrameView = () => {
  return <iframe ref={PageContext.iframeRef} className="bmbp-iframe-body" src={PageContext.pageSrc}></iframe>
}
