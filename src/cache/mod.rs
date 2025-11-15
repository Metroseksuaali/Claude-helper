use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheEntry<T> {
    pub data: T,
    pub timestamp: u64,
    pub ttl_seconds: u64,
}

impl<T> CacheEntry<T>
where
    T: Serialize + for<'de> Deserialize<'de>,
{
    pub fn new(data: T, ttl_seconds: u64) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Self {
            data,
            timestamp,
            ttl_seconds,
        }
    }

    pub fn is_fresh(&self) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        now - self.timestamp < self.ttl_seconds
    }
}

pub struct Cache {
    cache_dir: PathBuf,
}

impl Cache {
    pub fn new() -> Result<Self> {
        let cache_dir = dirs::cache_dir()
            .context("Failed to find cache directory")?
            .join("claude-helper");

        if !cache_dir.exists() {
            fs::create_dir_all(&cache_dir)
                .context("Failed to create cache directory")?;
        }

        Ok(Self { cache_dir })
    }

    pub fn get<T>(&self, key: &str) -> Result<Option<T>>
    where
        T: Serialize + for<'de> Deserialize<'de>,
    {
        let cache_file = self.cache_dir.join(format!("{}.json", key));

        if !cache_file.exists() {
            return Ok(None);
        }

        let contents = fs::read_to_string(&cache_file)
            .context("Failed to read cache file")?;

        let entry: CacheEntry<T> = serde_json::from_str(&contents)
            .context("Failed to parse cache file")?;

        if entry.is_fresh() {
            Ok(Some(entry.data))
        } else {
            // Cache expired, remove it
            let _ = fs::remove_file(&cache_file);
            Ok(None)
        }
    }

    pub fn set<T>(&self, key: &str, data: T, ttl_seconds: u64) -> Result<()>
    where
        T: Serialize + for<'de> Deserialize<'de>,
    {
        let cache_file = self.cache_dir.join(format!("{}.json", key));
        let entry = CacheEntry::new(data, ttl_seconds);

        let contents = serde_json::to_string(&entry)
            .context("Failed to serialize cache entry")?;

        fs::write(&cache_file, contents)
            .context("Failed to write cache file")?;

        Ok(())
    }

    pub fn clear(&self, key: &str) -> Result<()> {
        let cache_file = self.cache_dir.join(format!("{}.json", key));

        if cache_file.exists() {
            fs::remove_file(&cache_file)
                .context("Failed to remove cache file")?;
        }

        Ok(())
    }

    pub fn clear_all(&self) -> Result<()> {
        if self.cache_dir.exists() {
            fs::remove_dir_all(&self.cache_dir)
                .context("Failed to remove cache directory")?;
            fs::create_dir_all(&self.cache_dir)
                .context("Failed to recreate cache directory")?;
        }

        Ok(())
    }
}
