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
