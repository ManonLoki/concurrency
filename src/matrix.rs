use anyhow::Result;
use std::{
    fmt,
    ops::{Add, AddAssign, Mul},
    sync::mpsc,
};

use crate::{dot_product, Vector};

/// 最大线程数
const MAX_THREADS: usize = 4;

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

/// 实现Mul
impl<T> Mul for Matrix<T>
where
    T: Default + Copy + Mul<Output = T> + Add<Output = T> + AddAssign + Send + 'static,
{
    // 这里改造了下 把返回值改成了Option 避免在实现中出现无法捕获的错误
    type Output = Option<Matrix<T>>;

    fn mul(self, rhs: Self) -> Self::Output {
        multiply(&self, &rhs).ok()
    }
}

/// 输入消息
struct MessageInput<T> {
    index: usize,
    row: Vector<T>,
    col: Vector<T>,
}
/// 输出消息
struct MessageOutput<T> {
    index: usize,
    data: T,
}

/// 包装在Sender中的数据
struct Message<T> {
    input: MessageInput<T>,
    sender: oneshot::Sender<MessageOutput<T>>,
}

/// 计算矩阵
pub fn multiply<T>(a: &Matrix<T>, b: &Matrix<T>) -> Result<Matrix<T>>
where
    T: Default + Copy + Mul<Output = T> + Add<Output = T> + AddAssign + Send + 'static,
{
    if a.col != b.row {
        return Err(anyhow::anyhow!("矩阵无法相乘 因为a.col != b.col"));
    }
    // 根据线程 创建Channel
    let senders = (0..MAX_THREADS)
        .map(|_| {
            let (tx, rx) = mpsc::channel::<Message<T>>();

            // 开启新线程 每一个线程 带一个自己的channel
            std::thread::spawn(move || {
                for msg in rx {
                    // 计算点积
                    let value = dot_product(msg.input.row, msg.input.col)?;

                    // 准备输出结果
                    let output = MessageOutput {
                        index: msg.input.index,
                        data: value,
                    };
                    // 尝试发送数据
                    if let Err(e) = msg.sender.send(output) {
                        eprintln!("Error: {}", e);
                    }
                }
                Ok::<_, anyhow::Error>(())
            });
            tx
        })
        .collect::<Vec<_>>();

    // 新的矩阵长度
    let matrix_len = a.row * b.col;
    // 新的Receiver列表
    let mut receivers = vec![];
    // 新的空矩阵
    let mut data = vec![T::default(); matrix_len];

    for i in 0..a.row {
        for j in 0..b.col {
            // 假设矩阵 A
            // 1 2 3
            // 4 5 6
            // 矩阵 B
            // 1 2
            // 3 4
            // 5 6

            // 那么本次要计算的行数据 应该是 a的行，例如 [1,2,3] 或者 [4,5,6]
            // 那么他们的Start 就是 a的行索引i*行内的列a.col 为start 并且取a.col个数据
            let row = Vector::new(&a.data[i * a.col..(i + 1) * a.col]);

            // 那么B取列，则是按照b的列索引开始，每隔b.col个数据取一次
            // 那么当j=0时，取的是1,3，5 当j=1时，取的是2,4,6
            let col_data = b.data[j..]
                .iter()
                .step_by(b.col)
                .copied()
                .collect::<Vec<T>>();
            let col = Vector::new(col_data);

            // 记录索引 行 * 列 + 列 = 当前矩阵计算的索引 如刚刚的3x2矩阵 第1行2列 = 1*2+2=4
            let idx = i * b.col + j;
            let input = MessageInput {
                index: idx,
                row,
                col,
            };
            //  创面回执channel
            let (tx, rx) = oneshot::channel();
            // 发送Msg 到Mpsc管道中
            if let Err(e) = senders[idx % MAX_THREADS].send(Message { input, sender: tx }) {
                eprintln!("Send Error: {}", e);
            }
            // 将onshot的接收者放入receivers中
            receivers.push(rx);
        }
    }

    // 等待oneshot channel的所有的数据返回
    for rx in receivers {
        let output = rx.recv()?;
        data[output.index] = output.data;
    }

    // 返回结果
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
        let c = a * b;
        assert_eq!(
            format!("{:?}", c.unwrap()),
            "Matrix(row=2,col=2,{22 28, 49 64}"
        );
    }

    #[test]
    fn test_display() {
        let a = Matrix::new(vec![1, 2, 3, 4, 5, 6], 2, 3);
        assert_eq!(format!("{}", a), "{1 2 3, 4 5 6}");
    }

    #[test]
    #[should_panic]
    fn test_multiply_option_and_panic() {
        let a = Matrix::new(vec![1, 2, 3, 4, 5, 6], 2, 3);
        let b = Matrix::new(vec![1, 2, 3, 4, 5, 6], 2, 3);
        let c = a * b;
        c.unwrap();
    }
}
