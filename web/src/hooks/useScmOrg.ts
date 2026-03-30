import { useEffect, useState } from 'react';
import { message } from 'antd';
import { organizationApi } from '@/services/api';

/**
 * 用于 SCM 模块的自定义 hook,自动处理组织 ID
 * 如果没有 org_id,会自动获取租户下的第一个组织
 */
export function useScmOrg() {
  const [orgId, setOrgId] = useState<string>('');
  const [tenantId, setTenantId] = useState<string>('');
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string>('');

  useEffect(() => {
    const tenant = localStorage.getItem('tenant_id') || '';
    const org = localStorage.getItem('org_id') || '';

    setTenantId(tenant);

    if (org) {
      setOrgId(org);
      setLoading(false);
    } else if (tenant) {
      // 尝试自动获取第一个组织
      fetchFirstOrg(tenant);
    } else {
      setError('缺少租户信息');
      setLoading(false);
    }
  }, []);

  const fetchFirstOrg = async (tenant: string) => {
    try {
      const orgs = await organizationApi.list(tenant);
      if (orgs && orgs.length > 0) {
        const firstOrgId = orgs[0].id;
        localStorage.setItem('org_id', firstOrgId);
        setOrgId(firstOrgId);
      } else {
        setError('当前租户下没有组织,请联系管理员创建组织');
      }
    } catch (err) {
      console.error('Failed to fetch organizations:', err);
      setError('获取组织信息失败');
    } finally {
      setLoading(false);
    }
  };

  return { orgId, tenantId, loading, error };
}
