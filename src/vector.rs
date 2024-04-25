use anyhow::Result;
use std::ops::{Add, AddAssign, Deref, Mul};
/// 实现一个向量
pub struct Vector<T> {
    data: Vec<T>,
}

impl<T> Vector<T> {
    /// 实现New
    pub fn new(data: impl Into<Vec<T>>) -> Self {
        Self { data: data.into() }
    }
}

/// 实现解引用trait
impl<T> Deref for Vector<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

/// 计算矩阵点积
pub fn dot_product<T>(a: Vector<T>, b: Vector<T>) -> Result<T>
where
    T: Default + Copy + Mul<Output = T> + Add<Output = T> + AddAssign,
{
    if a.len() != b.len() {
        return Err(anyhow::anyhow!("向量长度不一致"));
    }

    let mut sum = T::default();
    for i in 0..a.len() {
        sum += a[i] * b[i];
    }

    Ok(sum)
}
