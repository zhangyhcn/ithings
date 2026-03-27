-- Add device_id column to device_instances table
-- This associates each device instance with a device definition

-- 1. Add device_id column (nullable first)
ALTER TABLE device_instances ADD COLUMN IF NOT EXISTS device_id uuid;

-- 2. For existing data, we need to find a device_id that matches the product_id
-- This is a best-effort approach: pick the first device with matching product_id
-- WARNING: If you have multiple devices for the same product, this might not be correct
-- You should manually verify after migration
UPDATE device_instances di
SET device_id = (
    SELECT d.id 
    FROM devices d 
    WHERE d.product_id = di.product_id 
    LIMIT 1
)
WHERE di.device_id IS NULL;

-- 3. Set device_id NOT NULL
ALTER TABLE device_instances ALTER COLUMN device_id SET NOT NULL;

-- 4. Add foreign key constraint
ALTER TABLE device_instances 
    DROP CONSTRAINT IF EXISTS device_instances_device_id_fkey,
    ADD CONSTRAINT device_instances_device_id_fkey 
    FOREIGN KEY (device_id) 
    REFERENCES devices(id) 
    ON DELETE CASCADE;
