axios.interceptors.response.use(function (response) {
  let data = response.data;
  if (data.error) {
    return {
      code: data.error.code,
      msg: data.error.name,
      data: data.error
    }
  } else {
    return data;
  }
}, function (error) {
  return Promise.reject(error);
});
