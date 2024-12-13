use std::{
    borrow::BorrowMut,
    collections::HashMap,
    time::{SystemTime, UNIX_EPOCH},
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Cache {
    template_file_data: HashMap<String, TmplData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TmplData {
    #[serde(with = "serde_systemtime")]
    pub last_synced: SystemTime,
}

impl Default for TmplData {
    fn default() -> Self {
        Self {
            last_synced: SystemTime::UNIX_EPOCH,
        }
    }
}

impl Cache {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn tmpl_mut(&mut self, file: &str) -> &mut TmplData {
        self.template_file_data
            .entry(file.to_string())
            .or_default()
            .borrow_mut()
    }

    pub fn set_file_synced_at(&mut self, file: String, time: SystemTime) {
        self.template_file_data.entry(file).or_default().last_synced = time;
    }

    pub fn get_last_sync_time(&self, file: &str) -> SystemTime {
        self.template_file_data
            .get(file)
            .map(|x| x.last_synced)
            .unwrap_or(UNIX_EPOCH)
    }
}

mod serde_systemtime {
    use std::time::{Duration, SystemTime, UNIX_EPOCH};

    use serde::{Deserializer, Serializer};

    pub fn serialize<S>(value: &SystemTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(
            value
                .duration_since(UNIX_EPOCH)
                .map(|x| x.as_secs())
                .unwrap_or(0),
        )
    }
    pub fn deserialize<'de, D>(deserializer: D) -> Result<SystemTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let val = Duration::from_secs(serde::Deserialize::deserialize(deserializer)?);
        Ok(UNIX_EPOCH + val)
    }
}
