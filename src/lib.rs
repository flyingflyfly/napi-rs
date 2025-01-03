#![deny(clippy::all)]
use futures::prelude::*;
use napi::bindgen_prelude::*;
use napi::tokio::{fs, task};

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
