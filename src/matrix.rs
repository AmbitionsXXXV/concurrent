// --导入所需的标准库模块--
use std::{
  fmt::{self, Debug, Display},
  ops::{Add, AddAssign, Mul},
  sync::mpsc,
  thread,
};

// --导入错误处理相关的模块--
use anyhow::{anyhow, Result};

// --导入自定义的点积函数和向量结构--
use crate::{dot_product, Vector};

// --定义用于并行计算的线程数--
const NUM_THREADS: usize = 4;

/// 矩阵结构体
/// T: 矩阵元素的类型
pub struct Matrix<T> {
  data: Vec<T>, // 存储矩阵数据的向量
  row: usize,   // 矩阵的行数
  col: usize,   // 矩阵的列数
}

/// 消息输入结构体，用于线程间通信
pub struct MsgInput<T> {
  idx: usize,     // 结果在输出矩阵中的索引
  row: Vector<T>, // 输入矩阵的一行
  col: Vector<T>, // 输入矩阵的一列
}

/// 消息输出结构体，用于线程间通信
pub struct MsgOutput<T> {
  idx: usize, // 结果在输出矩阵中的索引
  value: T,   // 计算得到的值
}

/// 消息结构体，包含输入和发送器
pub struct Msg<T> {
  input: MsgInput<T>,
  // 用于发送结果的发送器
  sender: oneshot::Sender<MsgOutput<T>>, // 使用 oneshot 为了确保只发送一次
}

/// 矩阵乘法函数
pub fn multiply<T>(a: &Matrix<T>, b: &Matrix<T>) -> Result<Matrix<T>>
where
  T: Copy
    + Default
    + Mul<Output = T>
    + Add<Output = T>
    + AddAssign<T>
    + Send
    + 'static,
{
  // --检查矩阵维度是否匹配--
  if a.col != b.row {
    return Err(anyhow!("Matrix multiply error: a.col != b.row"));
  }

  // --创建发送器数组，每个线程一个--
  let senders = (0..NUM_THREADS)
    .map(|_| {
      let (tx, rx) = mpsc::channel::<Msg<T>>();

      // --为每个发送器创建一个工作线程--
      thread::spawn(move || {
        for msg in rx {
          // --计算点积--
          let value = dot_product(msg.input.row, msg.input.col)?;
          // --发送计算结果--
          if let Err(e) = msg.sender.send(MsgOutput {
            idx: msg.input.idx,
            value,
          }) {
            eprintln!("send error: {}", e);
          }
        }

        Ok::<_, anyhow::Error>(())
      });

      tx
    })
    .collect::<Vec<_>>();

  // --准备结果矩阵--
  let matrix_len = a.row * b.col;
  let mut data = vec![T::default(); matrix_len];
  let mut receivers = Vec::with_capacity(matrix_len);

  // --分发计算任务--
  for i in 0..a.row {
    for j in 0..b.col {
      // --准备输入数据--
      let row = Vector::new(&a.data[i * a.col..(i + 1) * a.col]);
      let col_data: Vec<T> = b.data[j..]
        .iter()
        .step_by(b.col)
        .copied()
        .collect::<Vec<_>>();
      let col = Vector::new(col_data);
      let idx = i * b.col + j;
      let input = MsgInput::new(idx, row, col);

      // --创建一次性通道--
      let (tx, rx) = oneshot::channel();
      let msg = Msg::new(input, tx);

      // --发送任务到工作线程--
      if let Err(e) = senders[idx % NUM_THREADS].send(msg) {
        eprintln!("send error: {}", e);
      }

      receivers.push(rx);
    }
  }

  // --收集计算结果--
  for rx in receivers {
    let ret = rx.recv()?;
    data[ret.idx] = ret.value;
  }

  // --返回结果矩阵--
  Ok(Matrix {
    data,
    row: a.row,
    col: b.col,
  })
}

/// Matrix 结构体的实现
impl<T: Debug> Matrix<T> {
  /// 创建新的矩阵
  pub fn new(data: impl Into<Vec<T>>, row: usize, col: usize) -> Self {
    Self {
      data: data.into(),
      row,
      col,
    }
  }
}

/// 为 Matrix 实现 Display trait
impl<T> Display for Matrix<T>
where
  T: Display,
{
  // --以特定格式显示矩阵--
  // display a 2x3 as {1 2 3, 4 5 6}, 3x2 as {1 2, 3 4, 5 6}
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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

    write!(f, "}}")
  }
}

/// 为 Matrix 实现 Debug trait
impl<T> Debug for Matrix<T>
where
  T: Display,
{
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Matrix(row={}, col={}, {})", self.row, self.col, self)
  }
}

/// MsgInput 结构体的实现
impl<T> MsgInput<T> {
  pub fn new(idx: usize, row: Vector<T>, col: Vector<T>) -> Self {
    Self { idx, row, col }
  }
}

/// Msg 结构体的实现
impl<T> Msg<T> {
  pub fn new(input: MsgInput<T>, sender: oneshot::Sender<MsgOutput<T>>) -> Self {
    Self { input, sender }
  }
}

/// 测试模块
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_matrix_multiply() -> Result<()> {
    // --创建测试矩阵--
    let a = Matrix::new([1, 2, 3, 4, 5, 6], 2, 3);
    let b = Matrix::new([1, 2, 3, 4, 5, 6], 3, 2);

    // --执行矩阵乘法--
    let c = multiply(&a, &b)?;

    // --验证结果--
    assert_eq!(c.col, 2);
    assert_eq!(c.row, 2);
    assert_eq!(c.data, vec![22, 28, 49, 64]);
    assert_eq!(format!("{:?}", c), "Matrix(row=2, col=2, {22 28, 49 64})");

    Ok(())
  }
}
