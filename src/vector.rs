use anyhow::{anyhow, Result};
use std::ops::{Add, AddAssign, Deref, Mul};

/// 向量结构体
/// T: 向量元素的类型
pub struct Vector<T> {
  data: Vec<T>, // 存储向量数据的内部 Vec
}

// --点积函数，假设这是一个计算密集型操作--
pub fn dot_product<T>(a: Vector<T>, b: Vector<T>) -> Result<T>
where
  // Copy: 允许类型进行按位复制,避免所有权转移
  // Default: 提供类型的默认值,用于初始化 sum
  // Add<Output = T>: 允许类型进行加法运算,结果仍为 T 类型
  // AddAssign: 允许使用 += 运算符进行累加
  // Mul<Output = T>: 允许类型进行乘法运算,结果仍为 T 类型
  // 这些 trait 约束确保了 T 类型支持点积运算所需的所有操作
  T: Copy + Default + Add<Output = T> + AddAssign + Mul<Output = T>,
{
  // --检查两个向量的长度是否相等--
  if a.len() != b.len() {
    // a.len() 实际上调用 a.data.len() (通过 Deref trait)
    return Err(anyhow!("点积错误: a.len != b.len"));
  }

  // --初始化结果为类型 T 的默认值--
  let mut sum = T::default();
  // --遍历向量元素并计算点积--
  for i in 0..a.len() {
    sum += a[i] * b[i];
  }

  // --返回计算结果--
  Ok(sum)
}

/// 为 Vector<T> 实现 Deref trait
impl<T> Deref for Vector<T> {
  type Target = Vec<T>;

  // --解引用到内部的 Vec<T>--
  fn deref(&self) -> &Self::Target {
    &self.data
  }
}

/// Vector<T> 的实现
impl<T> Vector<T> {
  /// 创建新的 Vector 实例
  pub fn new(data: impl Into<Vec<T>>) -> Self {
    Self { data: data.into() }
  }
}
