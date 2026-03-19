import { extend } from 'umi-request';

const request = extend({
  timeout: 10000,
  errorHandler: (error: any) => {
    console.error('Request error:', error);
  },
});

// 请求拦截器
request.interceptors.request.use((url, options) => {
  const token = localStorage.getItem('access_token');
  if (token) {
    options.headers.Authorization = `Bearer ${token}`;
  }
  return { url, options };
});

// 响应拦截器
request.interceptors.response.use((response) => {
  return response;
});

export default request;
