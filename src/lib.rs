// use std::borrow::Borrow;

use deno_core::op;

#[op]
pub fn op_add(a: i32, b: i32) -> i32 {
    a + b
}
