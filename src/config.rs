use std::ffi::OsStr;
use std::fmt::{Debug, Display};
use std::str::FromStr;

// 日志相关配置。
pub const CONFIG_KEY_LOG_PATH: &'static str = "LOG_PATH";
pub const CONFIG_KEY_MAX_LOG_FILES: &'static str = "MAX_LOG_FILES";
pub const CONFIG_KEY_LOG_WITH_COLOR: &'static str = "LOG_WITH_COLOR";

// Web 相关配置。
pub const CONFIG_KEY_SERVER_ADDRESS: &'static str = "SERVER_ADDRESS";
pub const CONFIG_KEY_ACCESS_LOG_FORMAT: &'static str = "ACCESS_LOG_FORMAT";
pub const CONFIG_KEY_REQUEST_ID_HEADER: &'static str = "REQUEST_ID_HEADER";
pub const CONFIG_KEY_SERVER_NAME: &'static str = "SERVER_NAME";

pub fn get_config<K: AsRef<OsStr> + Display, T: FromStr>(key: K) -> T
where
    <T as FromStr>::Err: Debug,
{
    std::env::var(key.as_ref())
        .expect(&format!("without `{}` set in .env", key))
        .parse()
        .expect(&format!(
            "config `{}` is not valid type",
            CONFIG_KEY_LOG_WITH_COLOR
        ))
}

pub fn get_config_default<K: AsRef<OsStr> + Display, T: FromStr>(key: K, default: String) -> T
where
    <T as FromStr>::Err: Debug,
{
    std::env::var(key.as_ref())
        .unwrap_or(default)
        .parse()
        .expect(&format!(
            "config `{}` is not valid type",
            CONFIG_KEY_LOG_WITH_COLOR
        ))
}
