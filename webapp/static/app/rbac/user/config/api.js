const PageApi = {
}
const goBackBaseView = () => {
  window.parent.postMessage({ from: 'config', roleId: null }, "*");
}
