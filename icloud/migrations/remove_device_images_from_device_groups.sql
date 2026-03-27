-- Remove device_image and driver_image columns from device_groups table
-- Since these fields are now stored in device definition (devices table)

ALTER TABLE device_groups DROP COLUMN IF EXISTS device_image;
ALTER TABLE device_groups DROP COLUMN IF EXISTS driver_image;
