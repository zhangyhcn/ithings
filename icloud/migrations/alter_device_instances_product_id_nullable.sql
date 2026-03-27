-- Alter product_id column to be nullable in device_instances table
-- Since product_id is inherited from device definition, which can be Optional

ALTER TABLE device_instances ALTER COLUMN product_id DROP NOT NULL;
