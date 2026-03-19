pub const DATA_PUBLISH_TOPIC: &str = "driver/data";
pub const WRITE_REQUEST_TOPIC: &str = "driver/write";
pub const CONFIG_UPDATE_TOPIC: &str = "driver/config_update";
pub const CONFIG_DELETE_TOPIC: &str = "driver/config_delete";
pub const LOG_LEVEL_CHANGE_TOPIC: &str = "driver/log_level";

pub const REMOTE_DEVICE_DATA_TOPIC_TPL: &str = "{tenant_id}/{org_id}/{site_id}/{device_id}/data";
pub const REMOTE_DEVICE_COMMANDS_TOPIC_TPL: &str = "{tenant_id}/{org_id}/{site_id}/{device_id}/commands";
pub const REMOTE_DEVICE_CONFIG_TOPIC_TPL: &str = "{tenant_id}/{org_id}/{site_id}/{device_id}/config";
