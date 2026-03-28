export interface Response<T = any> {
  code: number;
  message: string;
  data: T;
}

export interface LoginParams {
  tenant_slug: string;
  username: string;
  password: string;
}

export interface LoginResponse {
  access_token: string;
  refresh_token: string;
  expires_in: number;
  user: User;
}

export interface User {
  id: string;
  tenant_id: string;
  username: string;
  email: string;
  role: string;
  is_superuser: boolean;
  is_active: boolean;
  created_at: string;
  updated_at: string;
  last_login?: string;
}

export interface Role {
  id: string;
  name: string;
  slug: string;
  description?: string;
  permissions: string[];
  status: string;
  created_at: string;
  updated_at: string;
}

export interface Tenant {
  id: string;
  name: string;
  slug: string;
  description?: string;
  config: any;
  status: string;
  created_at: string;
  updated_at: string;
}

export interface Organization {
  id: string;
  tenant_id: string;
  parent_id?: string;
  name: string;
  slug: string;
  description?: string;
  status: string;
  created_at: string;
  updated_at: string;
  children?: Organization[];
}

export interface Department {
  id: string;
  tenant_id: string;
  organization_id: string;
  parent_id?: string;
  name: string;
  description?: string;
  sort_order: number;
  status: string;
  created_at: string;
  updated_at: string;
  children?: Department[];
}

export interface Site {
  id: string;
  tenant_id: string;
  organization_id: string;
  name: string;
  slug: string;
  description?: string;
  location?: string;
  config: any;
  status: string;
  created_at: string;
  updated_at: string;
}

export interface Namespace {
  id: string;
  tenant_id: string;
  site_id: string;
  name: string;
  slug: string;
  description?: string;
  namespace_type: string;
  config: any;
  status: string;
  created_at: string;
  updated_at: string;
}

export interface Menu {
  id: string;
  parent_id?: string;
  name: string;
  path: string;
  component: string;
  icon?: string;
  sort_order: number;
  status: string;
  roles: string[];
  i18n_key?: string;
  children?: Menu[];
}

export interface CRD {
  id: string;
  namespace_id: string;
  name: string;
  slug: string;
  group: string;
  version: string;
  kind: string;
  description?: string;
  yaml?: any;
  status: string;
  k8s_name?: string;
  created_at: string;
  updated_at: string;
}

export interface Operator {
  id: string;
  namespace_id: string;
  name: string;
  slug: string;
  version: string;
  description?: string;
  yaml?: any;
  status: string;
  k8s_name?: string;
  created_at: string;
  updated_at: string;
}

export interface Controller {
  id: string;
  namespace_id: string;
  name: string;
  slug: string;
  kind: string;
  version: string;
  description?: string;
  yaml?: any;
  status: string;
  k8s_name?: string;
  created_at: string;
  updated_at: string;
}

export interface ConfigMap {
  id: string;
  namespace_id: string;
  name: string;
  slug: string;
  description?: string;
  data?: Record<string, string>;
  status: string;
  k8s_name?: string;
  created_at: string;
  updated_at: string;
}

export interface Product {
  id: string;
  tenant_id: string;
  name: string;
  description?: string;
  thing_model: any;
  rule: any;
  created_at: string;
  updated_at: string;
}

export interface Driver {
  id: string;
  tenant_id: string;
  name: string;
  description?: string;
  protocol_type: string;
  image: string;
  version: string;
  device_profile: any;
  created_at: string;
  updated_at: string;
}

export interface Node {
  id: string;
  name: string;
  status: string;
  labels: Record<string, string>;
  roles: string[];
  internal_ip?: string;
  os?: string;
  kernel_version?: string;
  container_runtime?: string;
  created_at: string;
  updated_at: string;
}

export interface DeviceGroup {
  id: string;
  tenant_id: string;
  org_id: string;
  org_name?: string;
  site_id: string;
  site_name?: string;
  namespace_id?: string;
  namespace_name?: string;
  name: string;
  description?: string;
  status: string;
  node_id?: string;
  created_at: string;
  updated_at: string;
}

export interface DeviceInstance {
  id: string;
  tenant_id: string;
  group_id: string;
  device_id: string;
  product_id: string;
  name: string;
  driver_config: any;
  thing_model: any;
  poll_interval_ms: number;
  node_id?: string;
  status: string;
  created_at: string;
  updated_at: string;
}

export interface Secret {
  id: string;
  namespace_id: string;
  name: string;
  slug: string;
  description?: string;
  data?: Record<string, string>;
  status: string;
  k8s_name?: string;
  created_at: string;
  updated_at: string;
}

export interface Device {
  id: string;
  tenant_id: string;
  product_id?: string;
  name: string;
  model?: string;
  manufacturer?: string;
  device_image: string;
  driver_image?: string;
  device_profile: any;
  description?: string;
  status: string;
  created_at: string;
  updated_at: string;
}
