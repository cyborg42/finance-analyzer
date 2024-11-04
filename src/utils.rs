use std::{
    path::Path,
    time::{Duration, UNIX_EPOCH},
};

use time::OffsetDateTime;

pub trait ToFile: serde::Serialize + serde::de::DeserializeOwned {
    fn save(&self, path: impl AsRef<Path>) -> anyhow::Result<()> {
        let data = serde_json::to_string_pretty(self)?;
        std::fs::write(path, data)?;
        Ok(())
    }
    fn load(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let data = std::fs::read_to_string(path)?;
        let ret: Self = serde_json::from_str(&data)?;
        Ok(ret)
    }
}

pub fn sec_time(timestamp: u64) -> OffsetDateTime {
    OffsetDateTime::from(UNIX_EPOCH + Duration::from_secs(timestamp))
}
