use dashmap::DashMap;
use std::default::Default;
use std::fmt::Display;
use std::sync::Arc;

/// 追踪信息 实现 inc,dec,snapshot
/// 这里的Clone调用了Data的Clon 相当于 Metrics{ data: data.clone() }
#[derive(Debug, Clone)]
pub struct CmapMetrics {
    data: Arc<DashMap<String, u64>>,
}

impl CmapMetrics {
    pub fn new() -> Self {
        Self {
            data: Arc::new(DashMap::new()),
        }
    }
    /// +1
    pub fn inc(&self, key: impl Into<String>) {
        let mut count = self.data.entry(key.into()).or_insert(0);
        *count += 1;
    }

    /// -1
    pub fn dec(&self, key: impl Into<String>) {
        let mut count = self.data.entry(key.into()).or_insert(0);
        *count -= 1;
    }
}

impl Display for CmapMetrics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for entry in self.data.iter() {
            writeln!(f, "{}: {}", entry.key(), entry.value())?;
        }
        Ok(())
    }
}

impl Default for CmapMetrics {
    fn default() -> Self {
        Self::new()
    }
}
