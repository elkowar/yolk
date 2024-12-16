use rhai::{CustomType, TypeBuilder};
use std::path::PathBuf;

#[derive(Debug, Clone, CustomType)]
pub struct SystemInfo {
    #[rhai_type(readonly)]
    hostname: String,
    #[rhai_type(readonly)]
    username: String,
    #[rhai_type(readonly)]
    distro: String,
    #[rhai_type(readonly)]
    device_name: String,
    #[rhai_type(readonly)]
    arch: String,
    #[rhai_type(readonly)]
    desktop_env: String,
    #[rhai_type(readonly)]
    platform: String,
    #[rhai_type(readonly)]
    paths: SystemInfoPaths,
}

#[derive(Debug, Clone, CustomType)]
pub struct SystemInfoPaths {
    #[rhai_type(readonly)]
    cache_dir: PathBuf,
    #[rhai_type(readonly)]
    config_dir: PathBuf,
    #[rhai_type(readonly)]
    home_dir: PathBuf,
    #[rhai_type(readonly)]
    yolk_dir: PathBuf,
}

impl SystemInfo {
    pub fn generate() -> Self {
        #[cfg(test)]
        return Self::canonical();
        #[cfg(not(test))]
        Self {
            hostname: whoami::fallible::hostname().unwrap_or_else(|_| "unknown".to_string()),
            username: whoami::username(),
            distro: whoami::distro().to_string(),
            device_name: whoami::fallible::devicename().unwrap_or_else(|_| "unknown".to_string()),
            arch: whoami::arch().to_string(),
            desktop_env: whoami::desktop_env().to_string(),
            platform: whoami::platform().to_string(),
            paths: SystemInfoPaths {
                cache_dir: dirs::cache_dir().unwrap_or_else(|| "unknown".into()),
                config_dir: dirs::config_dir().unwrap_or_else(|| "unknown".into()),
                home_dir: dirs::home_dir().unwrap_or_else(|| "unknown".into()),
                yolk_dir: default_yolk_dir(),
            },
        }
    }

    pub fn canonical() -> Self {
        Self {
            hostname: "canonical-hostname".to_string(),
            username: "canonical-username".to_string(),
            paths: SystemInfoPaths {
                cache_dir: (PathBuf::from("/canonical/cache")),
                config_dir: (PathBuf::from("/canonical/config")),
                home_dir: (PathBuf::from("/canonical/home")),
                yolk_dir: PathBuf::from("/canonical/yolk"),
            },
            distro: "distro".to_string(),
            device_name: "devicename".to_string(),
            arch: "x86_64".to_string(),
            desktop_env: "gnome".to_string(),
            platform: "linux".to_string(),
        }
    }
}

#[allow(unused)]
fn default_yolk_dir() -> PathBuf {
    dirs::config_dir()
        .expect("No config dir found")
        .join("yolk")
}
