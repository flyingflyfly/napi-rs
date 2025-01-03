#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

#[napi]
pub fn sum(a: i32, b: i32) -> i32 {
  a + b
}

#[napi]
pub fn sum_async(a: i32, b: i32) -> napi::Result<i32> {
  Ok(a + b)
}

#[napi]
pub fn fibonacci(n: i32) -> i32 {
  if n <= 1 {
    return n;
  }

  fibonacci(n - 1) + fibonacci(n - 2)
}
