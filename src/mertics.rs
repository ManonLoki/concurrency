use anyhow::Result;
use std::collections::HashMap;
use std::default::Default;
use std::fmt::Display;
use std::sync::{Arc, RwLock};

/// 追踪信息 实现 inc,dec,snapshot
/// 这里的Clone调用了Data的Clon 相当于 Metrics{ data: data.clone() }
#[derive(Debug, Clone)]
pub struct Metrics {
    data: Arc<RwLock<HashMap<String, u64>>>,
}

impl Metrics {
    pub fn new() -> Self {
        Self {
            data: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    /// +1
    pub fn inc(&self, key: impl Into<String>) -> Result<()> {
        let mut data = self
            .data
            .write()
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;

        let count = data.entry(key.into()).or_insert(0);
        *count += 1;

        Ok(())
    }

    /// -1
    pub fn dec(&self, key: impl Into<String>) -> Result<()> {
        let mut data = self
            .data
            .write()
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;

        let count = data.entry(key.into()).or_insert(0);
        *count -= 1;

        Ok(())
    }

    /// 获取快照
    pub fn snapshot(&self) -> HashMap<String, u64> {
        self.data.read().unwrap().clone()
    }
}

impl Display for Metrics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let data = self.data.read().map_err(|_| std::fmt::Error {})?;
        for (key, value) in data.iter() {
            writeln!(f, "{}: {}", key, value)?;
        }
        Ok(())
    }
}

impl Default for Metrics {
    fn default() -> Self {
        Self::new()
    }
}
