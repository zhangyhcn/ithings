ALTER TABLE products ADD COLUMN IF NOT EXISTS device_image TEXT NOT NULL DEFAULT 'device-meter:latest';
