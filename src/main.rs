use anyhow::{anyhow, Result};
use rand::random;
use std::{sync::mpsc, thread};

const NUM_PRODUCTS: usize = 4;

#[allow(dead_code)]
#[derive(Debug)]
struct Msg {
  id: usize,
  value: usize,
}

fn main() -> Result<()> {
  let (tx, rx) = mpsc::channel();

  for i in 0..NUM_PRODUCTS {
    let tx = tx.clone();

    thread::spawn(move || producer(i, tx));
  }

  let consumer = thread::spawn(move || {
    for msg in rx {
      println!("Consumer: {:?}", msg);
    }
  });

  consumer
    .join()
    .map_err(|e| anyhow!("Consumer thread panicked: {:?}", e))?;

  Ok(())
}

fn producer(idx: usize, tx: mpsc::Sender<Msg>) -> Result<()> {
  loop {
    let value = rand::random::<usize>();
    tx.send(Msg::new(idx, value))?;
    let sleep_time = random::<u8>() as u64 * 10;
    thread::sleep(std::time::Duration::from_millis(sleep_time as _));
  }
}

impl Msg {
  fn new(id: usize, value: usize) -> Self {
    Self { id, value }
  }
}
