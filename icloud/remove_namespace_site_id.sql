-- 移除namespaces表中的site_id列（namespace现在直接和租户关联）
ALTER TABLE namespaces DROP COLUMN site_id;
