import axios, { AxiosRequestConfig, AxiosResponse } from 'axios';
import { message } from 'antd';
import type { Tenant, User, Organization, Department, Menu } from '@/types';

const request = axios.create({
  baseURL: '/api/v1',
  timeout: 30000,
});

request.interceptors.request.use(
  (config) => {
    const token = localStorage.getItem('access_token');
    if (token) {
      config.headers.Authorization = `Bearer ${token}`;
    }
    return config;
  },
  (error) => {
    return Promise.reject(error);
  }
);

request.interceptors.response.use(
  (response: AxiosResponse) => {
    const { data } = response;
    if (data.code === 200 || data.code === 0) {
      return data.data;
    }
    message.error(data.message || '请求失败');
    return Promise.reject(data);
  },
  (error) => {
    if (error.response) {
      const { status } = error.response;
      if (status === 401) {
        message.error('登录已过期，请重新登录');
        localStorage.removeItem('access_token');
        localStorage.removeItem('refresh_token');
        window.location.href = '/user/login';
      } else {
        message.error(error.response.data?.message || '请求失败');
      }
    } else {
      message.error('网络错误');
    }
    return Promise.reject(error);
  }
);

export default request;

export const authApi = {
  login: (data: { username: string; password: string }) =>
    request.post('/auth/login', data),
  register: (data: {
    username: string;
    email: string;
    password: string;
  }) => request.post('/auth/register', data),
  forgotPassword: (data: { email: string }) =>
    request.post('/auth/forgot-password', data),
  refresh: (data: { refresh_token: string }) =>
    request.post('/auth/refresh', data),
  getCurrentUser: () => request.get('/users/me'),
};

export interface PaginationResponse<T> {
  list: T[];
  total: number;
  page: number;
  page_size: number;
}

export const tenantApi = {
  list: (params?: { page?: number; page_size?: number }) =>
    request.get<PaginationResponse<Tenant>>('/tenants', { params }),
  get: (id: string) => request.get(`/tenants/${id}`),
  create: (data: { 
    name: string; 
    slug: string; 
    description?: string;
    admin_username: string;
    admin_email: string;
    admin_password: string;
  }) =>
    request.post('/tenants', data),
  update: (id: string, data: { name?: string; description?: string }) =>
    request.put(`/tenants/${id}`, data),
  delete: (id: string) => request.delete(`/tenants/${id}`),
};

export const siteApi = {
  list: (tenantId: string, params?: { page?: number; page_size?: number }) =>
    request.get(`/tenants/${tenantId}/sites`, { params }),
  get: (tenantId: string, id: string) =>
    request.get(`/tenants/${tenantId}/sites/${id}`),
  create: (
    tenantId: string,
    data: { name: string; slug: string; description?: string; location?: string }
  ) => request.post(`/tenants/${tenantId}/sites`, data),
  update: (
    tenantId: string,
    id: string,
    data: { name?: string; description?: string; location?: string; status?: string }
  ) => request.put(`/tenants/${tenantId}/sites/${id}`, data),
  delete: (tenantId: string, id: string) =>
    request.delete(`/tenants/${tenantId}/sites/${id}`),
};

export const namespaceApi = {
  list: (tenantId: string) =>
    request.get<Namespace[]>(`/tenants/${tenantId}/namespaces`),
  listAll: () =>
    request.get<Namespace[]>('/namespaces'),
  listByTenant: (tenantId: string) =>
    request.get<Namespace[]>(`/tenants/${tenantId}/namespaces`),
  get: (tenantId: string, id: string) =>
    request.get(`/tenants/${tenantId}/namespaces/${id}`),
  create: (
    tenantId: string,
    data: { site_id: string; name: string; slug: string; description?: string; namespace_type?: string }
  ) => request.post(`/tenants/${tenantId}/namespaces`, data),
  update: (
    tenantId: string,
    id: string,
    data: { name?: string; description?: string; namespace_type?: string; status?: string }
  ) => request.put(`/tenants/${tenantId}/namespaces/${id}`, data),
  delete: (tenantId: string, id: string) =>
    request.delete(`/tenants/${tenantId}/namespaces/${id}`),
};

export const userApi = {
  list: (params?: { page?: number; page_size?: number }) =>
    request.get<PaginationResponse<User>>('/users', { params }),
  get: (id: string) => request.get(`/users/${id}`),
  create: (data: { 
    username: string; 
    email: string; 
    password: string;
    role: string;
    is_active?: boolean;
  }) => request.post('/users', data),
  update: (id: string, data: { 
    username?: string; 
    email?: string; 
    password?: string;
    role?: string;
    is_active?: boolean;
  }) => request.put(`/users/${id}`, data),
  delete: (id: string) => request.delete(`/users/${id}`),
};

export const roleApi = {
  list: () => request.get<Role[]>('/roles'),
  get: (id: string) => request.get(`/roles/${id}`),
  create: (data: { 
    name: string; 
    slug: string; 
    description?: string;
    permissions?: string[];
  }) => request.post('/roles', data),
  update: (id: string, data: { 
    name?: string; 
    slug?: string; 
    description?: string;
    permissions?: string[];
    status?: string;
  }) => request.put(`/roles/${id}`, data),
  delete: (id: string) => request.delete(`/roles/${id}`),
};

export const userRoleApi = {
  getUserRoles: (userId: string) => request.get(`/user_roles/${userId}`),
  assignRoles: (data: { user_id: string; role_ids: string[] }) => 
    request.post('/user_roles', data),
  removeRole: (userId: string, roleId: string) => 
    request.delete(`/user_roles/${userId}/${roleId}`),
};

export const roleMenuApi = {
  getRoleMenus: (roleId: string) => request.get(`/role_menus/${roleId}`),
  assignMenus: (data: { role_id: string; menu_ids: string[] }) => 
    request.post('/role_menus', data),
  removeMenu: (roleId: string, menuId: string) => 
    request.delete(`/role_menus/${roleId}/${menuId}`),
};

export const organizationApi = {
  list: (tenantId: string) => request.get<Organization[]>(`/tenants/${tenantId}/organizations`),
  get: (tenantId: string, id: string) => request.get(`/tenants/${tenantId}/organizations/${id}`),
  create: (
    tenantId: string,
    data: { 
      name: string; 
      parent_id?: string;
      description?: string;
      sort_order?: number;
      status?: string;
    }
  ) => request.post(`/tenants/${tenantId}/organizations`, data),
  update: (
    tenantId: string,
    id: string,
    data: { 
      name?: string; 
      parent_id?: string;
      description?: string;
      sort_order?: number;
      status?: string;
    }
  ) => request.put(`/tenants/${tenantId}/organizations/${id}`, data),
  delete: (tenantId: string, id: string) => request.delete(`/tenants/${tenantId}/organizations/${id}`),
};

export const departmentApi = {
  list: (tenantId: string, organizationId: string) => 
    request.get<Department[]>(`/tenants/${tenantId}/organizations/${organizationId}/departments`),
  get: (tenantId: string, organizationId: string, id: string) => 
    request.get(`/tenants/${tenantId}/organizations/${organizationId}/departments/${id}`),
  create: (
    tenantId: string,
    organizationId: string,
    data: { 
      name: string; 
      parent_id?: string;
      description?: string;
      sort_order?: number;
      status?: string;
    }
  ) => request.post(`/tenants/${tenantId}/organizations/${organizationId}/departments`, data),
  update: (
    tenantId: string,
    organizationId: string,
    id: string,
    data: { 
      name?: string; 
      parent_id?: string;
      description?: string;
      sort_order?: number;
      status?: string;
    }
  ) => request.put(`/tenants/${tenantId}/organizations/${organizationId}/departments/${id}`, data),
  delete: (tenantId: string, organizationId: string, id: string) => 
    request.delete(`/tenants/${tenantId}/organizations/${organizationId}/departments/${id}`),
};

export const menuApi = {
  getMenuTree: () => request.get<any, Menu[]>('/menus'),
  getUserMenus: () => request.get<any, Menu[]>('/menus/user'),
  create: (data: Omit<Menu, 'id' | 'created_at' | 'updated_at'>) => 
    request.post('/menus', data),
  update: (id: string, data: Partial<Omit<Menu, 'id' | 'created_at' | 'updated_at'>>) => 
    request.put(`/menus/${id}`, data),
  delete: (id: string) => request.delete(`/menus/${id}`),
};

import type { CRD, Operator, Controller } from '@/types';

export const crdApi = {
  list: (params?: { page?: number; page_size?: number }) =>
    request.get<PaginationResponse<CRD>>('/crdes', { params }),
  get: (id: string) => request.get(`/crdes/${id}`),
  create: (data: {
    namespace_id: string;
    name: string;
    slug: string;
    group: string;
    version: string;
    kind: string;
    description?: string;
    yaml?: any;
  }) => request.post('/crdes', data),
  update: (id: string, data: {
    name?: string;
    group?: string;
    version?: string;
    kind?: string;
    description?: string;
    yaml?: any;
    status?: string;
  }) => request.put(`/crdes/${id}`, data),
  delete: (id: string) => request.delete(`/crdes/${id}`),
  publish: (id: string) => request.post(`/crdes/${id}/publish`),
};

export const operatorApi = {
  list: (params?: { page?: number; page_size?: number }) =>
    request.get<PaginationResponse<Operator>>('/operators', { params }),
  get: (id: string) => request.get(`/operators/${id}`),
  create: (data: {
    namespace_id: string;
    name: string;
    slug: string;
    version: string;
    description?: string;
    yaml?: any;
  }) => request.post('/operators', data),
  update: (id: string, data: {
    name?: string;
    version?: string;
    description?: string;
    yaml?: any;
    status?: string;
  }) => request.put(`/operators/${id}`, data),
  delete: (id: string) => request.delete(`/operators/${id}`),
  publish: (id: string) => request.post(`/operators/${id}/publish`),
};

export const controllerApi = {
  list: (params?: { page?: number; page_size?: number }) =>
    request.get<PaginationResponse<Controller>>('/controllers', { params }),
  get: (id: string) => request.get(`/controllers/${id}`),
  create: (data: {
    namespace_id: string;
    name: string;
    slug: string;
    kind: string;
    version: string;
    description?: string;
    yaml?: any;
  }) => request.post('/controllers', data),
  update: (id: string, data: {
    name?: string;
    kind?: string;
    version?: string;
    description?: string;
    yaml?: any;
    status?: string;
  }) => request.put(`/controllers/${id}`, data),
  delete: (id: string) => request.delete(`/controllers/${id}`),
  publish: (id: string) => request.post(`/controllers/${id}/publish`),
};

import type { ConfigMap, Secret } from '@/types';

export const configMapApi = {
  list: (params?: { page?: number; page_size?: number }) =>
    request.get<PaginationResponse<ConfigMap>>('/config_maps', { params }),
  get: (id: string) => request.get(`/config_maps/${id}`),
  create: (data: {
    namespace_id: string;
    name: string;
    slug: string;
    description?: string;
    data?: Record<string, string>;
  }) => request.post('/config_maps', data),
  update: (id: string, data: {
    name?: string;
    description?: string;
    data?: Record<string, string>;
    status?: string;
  }) => request.put(`/config_maps/${id}`, data),
  delete: (id: string) => request.delete(`/config_maps/${id}`),
  publish: (id: string) => request.post(`/config_maps/${id}/publish`),
};

export const secretApi = {
  list: (params?: { page?: number; page_size?: number }) =>
    request.get<PaginationResponse<Secret>>('/secrets', { params }),
  get: (id: string) => request.get(`/secrets/${id}`),
  create: (data: {
    namespace_id: string;
    name: string;
    slug: string;
    description?: string;
    data?: Record<string, string>;
  }) => request.post('/secrets', data),
  update: (id: string, data: {
    name?: string;
    description?: string;
    data?: Record<string, string>;
    status?: string;
  }) => request.put(`/secrets/${id}`, data),
  delete: (id: string) => request.delete(`/secrets/${id}`),
  publish: (id: string) => request.post(`/secrets/${id}/publish`),
};

import type { Product, Driver, Node, DeviceGroup, DeviceInstance } from '@/types';

export const productApi = {
  list: (tenantId: string, params?: { page?: number; page_size?: number }) =>
    request.get<Product[]>(`/tenants/${tenantId}/products`, { params }),
  get: (tenantId: string, id: string) =>
    request.get(`/tenants/${tenantId}/products/${id}`),
  create: (
    tenantId: string,
    data: {
      name: string;
      description?: string;
      thing_model: any;
    }
  ) => request.post(`/tenants/${tenantId}/products`, data),
  update: (
    tenantId: string,
    id: string,
    data: {
      name?: string;
      description?: string;
      thing_model?: any;
    }
  ) => request.put(`/tenants/${tenantId}/products/${id}`, data),
  delete: (tenantId: string, id: string) =>
    request.delete(`/tenants/${tenantId}/products/${id}`),
};

export const driverApi = {
  list: (tenantId: string, params?: { page?: number; page_size?: number }) =>
    request.get<Driver[]>(`/tenants/${tenantId}/drivers`, { params }),
  get: (tenantId: string, id: string) =>
    request.get(`/tenants/${tenantId}/drivers/${id}`),
  create: (
    tenantId: string,
    data: {
      name: string;
      description?: string;
      protocol_type: string;
      image: string;
      version: string;
    }
  ) => request.post(`/tenants/${tenantId}/drivers`, data),
  update: (
    tenantId: string,
    id: string,
    data: {
      name?: string;
      description?: string;
      protocol_type?: string;
      image?: string;
      version?: string;
    }
  ) => request.put(`/tenants/${tenantId}/drivers/${id}`, data),
  delete: (tenantId: string, id: string) =>
    request.delete(`/tenants/${tenantId}/drivers/${id}`),
  listTags: (tenantId: string, registry: string | undefined, image: string) =>
    request.get<{ code: number; message: string; data: string[] }>(
      `/tenants/${tenantId}/drivers/tags?image=${encodeURIComponent(image)}${registry ? `&registry=${encodeURIComponent(registry)}` : ''}`
    ),
};

export const nodeApi = {
  list: (tenantId: string) =>
    request.get<{ code: number; message: string; data: Node[] }>(`/tenants/${tenantId}/nodes`),
  sync: (tenantId: string) =>
    request.get<{ code: number; message: string; data: Node[] }>(`/tenants/${tenantId}/nodes/sync`),
  get: (tenantId: string, id: string) =>
    request.get(`/tenants/${tenantId}/nodes/${id}`),
  updateLabels: (
    tenantId: string,
    id: string,
    data: { labels: Record<string, string> }
  ) => request.put(`/tenants/${tenantId}/nodes/${id}/labels`, data),
};

export const deviceGroupApi = {
  list: (tenantId: string) =>
    request.get<DeviceGroup[]>(`/tenants/${tenantId}/device-groups`),
  get: (tenantId: string, id: string) =>
    request.get(`/tenants/${tenantId}/device-groups/${id}`),
  create: (
    tenantId: string,
    data: {
      org_id: string;
      site_id: string;
      name: string;
      driver_image: string;
      description?: string;
      node_id?: string;
    }
  ) => request.post(`/tenants/${tenantId}/device-groups`, data),
  update: (
    tenantId: string,
    id: string,
    data: {
      name?: string;
      driver_image?: string;
      description?: string;
      status?: string;
      node_id?: string;
    }
  ) => request.put(`/tenants/${tenantId}/device-groups/${id}`, data),
  delete: (tenantId: string, id: string) =>
    request.delete(`/tenants/${tenantId}/device-groups/${id}`),
  publish: (
    tenantId: string,
    id: string,
    data: {
      node_id: string;
      labels: Record<string, string>;
    }
  ) => request.post(`/tenants/${tenantId}/device-groups/${id}/publish`, data),
};

export const deviceInstanceApi = {
  list: (tenantId: string, params?: { group_id?: string }) =>
    request.get<DeviceInstance[]>(`/tenants/${tenantId}/device-instances`, { params }),
  get: (tenantId: string, id: string) =>
    request.get(`/tenants/${tenantId}/device-instances/${id}`),
  create: (
    tenantId: string,
    data: {
      group_id: string;
      device_id: string;
      name: string;
      driver_config?: any;
      thing_model?: any;
      poll_interval_ms?: number;
      node_id?: string;
    }
  ) => request.post(`/tenants/${tenantId}/device-instances`, data),
  update: (
    tenantId: string,
    id: string,
    data: {
      name?: string;
      driver_config?: any;
      thing_model?: any;
      poll_interval_ms?: number;
      node_id?: string;
      status?: string;
    }
  ) => request.put(`/tenants/${tenantId}/device-instances/${id}`, data),
  delete: (tenantId: string, id: string) =>
    request.delete(`/tenants/${tenantId}/device-instances/${id}`),
};

import type { Device } from '@/types';

export const deviceApi = {
  list: (tenantId: string, params?: { page?: number; page_size?: number }) =>
    request.get<Device[]>(`/tenants/${tenantId}/devices`, { params }),
  get: (tenantId: string, id: string) =>
    request.get(`/tenants/${tenantId}/devices/${id}`),
  create: (
    tenantId: string,
    data: {
      name: string;
      organization_id?: string;
      site_id?: string;
      product_id?: string;
      model?: string;
      manufacturer?: string;
      driver_image?: string;
      device_profile?: any;
      description?: string;
    }
  ) => request.post(`/tenants/${tenantId}/devices`, data),
  update: (
    tenantId: string,
    id: string,
    data: {
      name?: string;
      organization_id?: string;
      site_id?: string;
      product_id?: string;
      model?: string;
      manufacturer?: string;
      driver_image?: string;
      device_profile?: any;
      description?: string;
      status?: string;
    }
  ) => request.put(`/tenants/${tenantId}/devices/${id}`, data),
  delete: (tenantId: string, id: string) =>
    request.delete(`/tenants/${tenantId}/devices/${id}`),
};