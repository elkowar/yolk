use std::{
    collections::HashMap,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct SyncTimes {
    sync_times: HashMap<PathBuf, SyncTime>,
}

impl SyncTimes {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_sync_time(&self, path: &PathBuf) -> SyncTime {
        self.sync_times
            .get(path)
            .copied()
            .unwrap_or(SyncTime(UNIX_EPOCH))
    }

    pub fn set_sync_time(&mut self, path: PathBuf, time: impl Into<SyncTime>) {
        self.sync_times.insert(path, time.into());
    }
}
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct SyncTime(#[serde(with = "serde_systemtime")] pub SystemTime);

impl PartialEq<SystemTime> for SyncTime {
    fn eq(&self, other: &SystemTime) -> bool {
        self.0 == *other
    }
}
impl PartialOrd<SystemTime> for SyncTime {
    fn partial_cmp(&self, other: &SystemTime) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(other)
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
