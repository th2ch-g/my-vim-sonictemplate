#!/usr/bin/env rust-script

//! ```cargo
//! [dependencies]
//! proconio={version="*", features=["derive"]}
//! ```

use proconio::marker::*;
use proconio::*;
#[fastout]
fn main() {
    input! {
        {{_cursor_}}
    }
}
