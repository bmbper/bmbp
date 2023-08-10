const BmbpHttp = {
  post: (url, data) => {
    return axios.post(url, data);
  }
}

axios.interceptors.response.use((resp) => {
  if (resp.status == 200) {
    return resp.data;
  }
}, (err) => {
  return {
    code: '7000',
    msg: err.message,
  }
});
