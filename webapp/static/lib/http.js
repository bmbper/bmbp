const BmbpHttp = {
  post: (url, data) => {
    return axios.post(url, data);
  },
  get: (url, params) => {
    return axios.get(url, params);
  }
}

axios.interceptors.response.use((resp) => {
  if (resp.status == 200) {
    return resp.data;
  }
}, (err) => {
  arco.Message.error(err.message);
  return {
    code: '7000',
    msg: err.message,
  }
});
