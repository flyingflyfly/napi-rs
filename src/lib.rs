#![deny(clippy::all)]
use futures::prelude::*;
use napi::bindgen_prelude::*;
use napi::tokio::{fs, task};
use std::sync::{Arc, Mutex};
use std::thread;

#[macro_use]
extern crate napi_derive;

#[napi]
pub fn sum(a: i32, b: i32) -> i32 {
  a + b
}

#[napi]
pub fn fibonacci(n: i32) -> i32 {
  if n <= 1 {
    return n;
  }

  fibonacci(n - 1) + fibonacci(n - 2)
}

#[napi]
pub async fn read_file_async(path: String) -> Result<Buffer> {
  fs::read(path)
    .map(|r| match r {
      Ok(content) => Ok(content.into()),
      Err(e) => Err(Error::new(
        Status::GenericFailure,
        format!("failed to read file, {}", e),
      )),
    })
    .await
}

#[napi]
pub async fn async_fibonacci(arg: u32) -> Result<u32> {
  task::spawn(async move {
    let res = fibonacci(arg as i32) as u32;
    Ok(res)
  })
  .await
  .unwrap()
}

#[napi]
pub fn multi_threaded_fibonacci(arg: u32) -> Result<Vec<u32>> {
  let res_vex = Arc::new(Mutex::<Vec<u32>>::new(Vec::new()));
  let mut hadle_vex: Vec<thread::JoinHandle<()>> = Vec::new();
  for _ in 0..10 {
    let clone_vex = Arc::clone(&res_vex);
    let handle: thread::JoinHandle<()> = thread::spawn(move || {
      let num = fibonacci(arg as i32) as u32;
      let mut res = clone_vex.lock().unwrap(); // 这里需要加锁，因为多个线程同时访问
      res.push(num);
    });
    hadle_vex.push(handle);
  }
  for handle in hadle_vex.into_iter() {
    handle.join().unwrap();
  }

  Ok(res_vex.clone().lock().unwrap().to_vec())
}
