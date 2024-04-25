use anyhow::Result;
use std::{
    fmt,
    ops::{Add, AddAssign, Mul},
};

/// 矩阵运算
/// [[1,2],[3,4],[5,6]] => [1,2,3,4,5,6]

/// 矩阵 通过row 和 col 切割内部数组
pub struct Matrix<T> {
    data: Vec<T>,
    row: usize,
    col: usize,
}

/// 实现New
impl<T: fmt::Debug> Matrix<T> {
    pub fn new(data: impl Into<Vec<T>>, row: usize, col: usize) -> Self {
        Self {
            data: data.into(),
            row,
            col,
        }
    }
}

// 实现Display
impl<T> fmt::Display for Matrix<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        for i in 0..self.row {
            for j in 0..self.col {
                write!(f, "{}", self.data[i * self.col + j])?;
                if j != self.col - 1 {
                    write!(f, " ")?;
                }
            }

            if i != self.row - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, "}}")?;
        Ok(())
    }
}

impl<T> fmt::Debug for Matrix<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Matrix(row={},col={},{}", self.row, self.col, self)?;
        Ok(())
    }
}

/// 计算矩阵
pub fn multiply<T>(a: &Matrix<T>, b: &Matrix<T>) -> Result<Matrix<T>>
where
    T: Default + Copy + Mul<Output = T> + Add<Output = T> + AddAssign,
{
    if a.col != b.row {
        return Err(anyhow::anyhow!("矩阵无法相乘 因为a.col != b.col"));
    }

    let mut data = vec![T::default(); a.row * b.col];

    for i in 0..a.row {
        for j in 0..b.col {
            for k in 0..a.col {
                data[i * b.col + j] += a.data[i * a.col + k] * b.data[k * b.col + j];
            }
        }
    }
    Ok(Matrix {
        data,
        row: a.row,
        col: b.col,
    })
}

/// 单元测试
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiply() {
        let a = Matrix::new(vec![1, 2, 3, 4, 5, 6], 2, 3);
        let b = Matrix::new(vec![1, 2, 3, 4, 5, 6], 3, 2);
        let c = multiply(&a, &b).unwrap();
        assert_eq!(format!("{:?}", c), "Matrix(row=2,col=2,{22 28, 49 64}");
    }

    #[test]
    fn test_display() {
        let a = Matrix::new(vec![1, 2, 3, 4, 5, 6], 2, 3);
        assert_eq!(format!("{}", a), "{1 2 3, 4 5 6}");
    }
}
