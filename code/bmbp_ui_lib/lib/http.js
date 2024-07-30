// 添加请求拦截器
axios.interceptors.request.use(
  function (config) {
    // 在发送请求之前做些什么
    debugger;
    if (!config.headers) {
      config.headers = { "content-type": "application/json" };
    } else if (!config.headers["content-type"]) {
      config.headers["content-type"] = "application/json";
    }
    return config;
  },
  function (error) {
    // 对请求错误做些什么
    return Promise.reject(error);
  },
);

axios.interceptors.response.use(
  function (response) {
    let data = response.data;
    if (data.error) {
      return {
        code: data.error.code,
        msg: data.error.name,
        data: data.error,
      };
    } else {
      return data;
    }
  },
  function (error) {
    return Promise.reject(error);
  },
);
