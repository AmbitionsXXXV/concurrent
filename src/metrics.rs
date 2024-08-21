// 指标数据结构
// 基本功能：增加(inc)/快照(snapshot)

use anyhow::Result;
use dashmap::DashMap;
use std::{fmt, sync::Arc};

/// 指标结构体
/// 用于存储和管理各种指标的计数
#[derive(Debug, Clone)]
pub struct Metrics {
  // 使用 HashMap 存储指标名称和对应的计数值
  data: Arc<DashMap<String, i64>>,
}

impl Metrics {
  /// 创建一个新的 Metrics 实例
  pub fn new() -> Self {
    Self {
      data: Arc::new(DashMap::new()),
    }
  }

  /// 增加指定指标的计数
  pub fn inc(&self, key: impl Into<String>) -> Result<()> {
    // 如果指标不存在，则插入并初始化为 0，然后增加 1
    *self.data.entry(key.into()).or_insert(0) += 1;

    Ok(())
  }
}

impl Default for Metrics {
  fn default() -> Self {
    Self::new()
  }
}

impl fmt::Display for Metrics {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    for entry in self.data.iter() {
      writeln!(f, "{}: {}", entry.key(), entry.value())?;
    }

    Ok(())
  }
}
