const PageApi = {
}
const goBackBaseView = () => {
  window.parent.postMessage({ from: 'menu', roleId: null }, "*");
}
