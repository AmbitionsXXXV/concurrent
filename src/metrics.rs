// 指标数据结构
// 基本功能：增加(inc)/减少(dec)/快照(snapshot)

use anyhow::{anyhow, Ok, Result};
use std::{
  collections::HashMap,
  sync::{Arc, Mutex},
};

/// 指标结构体
/// 用于存储和管理各种指标的计数
#[derive(Debug, Clone)]
pub struct Metrics {
  // 使用 HashMap 存储指标名称和对应的计数值
  data: Arc<Mutex<HashMap<String, i64>>>,
}

impl Metrics {
  /// 创建一个新的 Metrics 实例
  pub fn new() -> Self {
    Self {
      data: Arc::new(Mutex::new(HashMap::new())),
    }
  }

  /// 增加指定指标的计数
  pub fn inc(&self, key: &str) -> Result<()> {
    // 如果指标不存在，则插入并初始化为 0，然后增加 1
    *self
      .data
      .lock()
      .map_err(|e| anyhow!(e.to_string()))?
      .entry(key.into())
      .or_insert(0) += 1;

    Ok(())
  }

  /// 减少指定指标的计数
  pub fn dec(&self, key: &str) -> Result<()> {
    // 如果指标不存在，则插入并初始化为 0，然后减少 1
    *self
      .data
      .lock()
      .map_err(|e| anyhow!(e.to_string()))?
      .entry(key.into())
      .or_insert(0) -= 1;

    Ok(())
  }

  /// 获取当前 Metrics 实例的快照
  pub fn snapshot(&self) -> Result<HashMap<String, i64>> {
    // 克隆整个 data HashMap 并返回
    Ok(
      self
        .data
        .lock()
        .map_err(|e| anyhow!(e.to_string()))?
        .clone(),
    )
  }
}

impl Default for Metrics {
  fn default() -> Self {
    Self::new()
  }
}
