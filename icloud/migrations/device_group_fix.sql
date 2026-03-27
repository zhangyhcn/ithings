-- 创建设备组表
CREATE TABLE IF NOT EXISTS device_groups (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    org_id UUID NOT NULL,
    site_id UUID NOT NULL,
    name TEXT NOT NULL,
    driver_image TEXT NOT NULL,
    description TEXT,
    status TEXT NOT NULL DEFAULT 'active',
    node_id UUID,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    FOREIGN KEY (tenant_id) REFERENCES tenants(id) ON DELETE CASCADE,
    FOREIGN KEY (org_id) REFERENCES organizations(id) ON DELETE CASCADE,
    FOREIGN KEY (site_id) REFERENCES sites(id) ON DELETE CASCADE
);

-- 修改设备实例表，使用 group_id 替代其他字段
ALTER TABLE device_instances DROP COLUMN IF EXISTS org_id;
ALTER TABLE device_instances DROP COLUMN IF EXISTS site_id;
ALTER TABLE device_instances DROP COLUMN IF EXISTS device_id;
ALTER TABLE device_instances DROP COLUMN IF EXISTS node_id;
ALTER TABLE device_instances ADD COLUMN IF NOT EXISTS group_id UUID NOT NULL;
ALTER TABLE device_instances ADD COLUMN IF NOT EXISTS product_id UUID NOT NULL;
ALTER TABLE device_instances ADD FOREIGN KEY (group_id) REFERENCES device_groups(id) ON DELETE CASCADE;
ALTER TABLE device_instances ADD FOREIGN KEY (product_id) REFERENCES products(id) ON DELETE CASCADE;