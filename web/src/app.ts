export function onRouteChange({ location, routes, clientRoutes, action }: any) {
  const token = localStorage.getItem('access_token');
  const isLoginPage = location.pathname.startsWith('/user/');
  const isCreateTenantPage = location.pathname === '/create_tenant';
  
  // 如果用户已登录且访问登录页面，跳转到首页
  if (token && isLoginPage) {
    window.location.href = '/';
  }
  
  // 如果用户未登录且访问受保护页面，跳转到登录页
  if (!token && !isLoginPage && !isCreateTenantPage) {
    window.location.href = '/user/login';
  }
}
