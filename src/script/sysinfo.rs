use rhai::{CustomType, TypeBuilder};

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
    cache_dir: String,
    #[rhai_type(readonly)]
    config_dir: String,
    #[rhai_type(readonly)]
    home_dir: String,
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
                cache_dir: dirs::cache_dir()
                    .map(|x| x.to_string_lossy().to_string())
                    .unwrap_or_else(|| "unknown".into()),
                config_dir: dirs::config_dir()
                    .map(|x| x.to_string_lossy().to_string())
                    .unwrap_or_else(|| "unknown".into()),
                home_dir: dirs::home_dir()
                    .map(|x| x.to_string_lossy().to_string())
                    .unwrap_or_else(|| "unknown".into()),
            },
        }
    }

    pub fn canonical() -> Self {
        Self {
            hostname: "canonical-hostname".to_string(),
            username: "canonical-username".to_string(),
            paths: SystemInfoPaths {
                cache_dir: "/canonical/cache".to_string(),
                config_dir: "/canonical/config".to_string(),
                home_dir: "/canonical/home".to_string(),
            },
            distro: "distro".to_string(),
            device_name: "devicename".to_string(),
            arch: "x86_64".to_string(),
            desktop_env: "gnome".to_string(),
            platform: "linux".to_string(),
        }
    }
}
