use anyhow::Result;
use concurrency::Metrics;

fn main() -> Result<()> {
  let metrics = Metrics::new();
  metrics.inc("req.page.1")?;
  metrics.inc("call.thread.worker.1")?;
  metrics.dec("11")?;
  metrics.dec("22")?;

  println!("{:#?}", metrics.snapshot().unwrap());

  Ok(())
}
