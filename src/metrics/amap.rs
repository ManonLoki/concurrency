use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
    sync::{atomic::AtomicI64, Arc},
};

#[derive(Debug, Clone)]
pub struct AmapMetrics {
    data: Arc<HashMap<&'static str, AtomicI64>>,
}

impl AmapMetrics {
    pub fn new(init_data: &[&'static str]) -> Self {
        let data = init_data
            .iter()
            .map(|item| (*item, AtomicI64::new(0)))
            .collect();
        Self {
            data: Arc::new(data),
        }
    }
    pub fn inc(&self, key: &str) {
        let counter = self.data.get(key).unwrap();
        counter.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    }
}

impl Display for AmapMetrics {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();
        for (key, value) in self.data.iter() {
            result.push_str(&format!(
                "{}: {}\n",
                key,
                value.load(std::sync::atomic::Ordering::Relaxed)
            ));
        }
        write!(f, "{}", result)
    }
}
