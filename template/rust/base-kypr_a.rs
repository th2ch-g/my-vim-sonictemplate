#!/usr/bin/env rust-script

//! ```cargo
//! [dependencies]
//! proconio={version="*", features=["derive"]}
//! ```

#[allow(unused)]
use proconio::marker::*;
use proconio::*;
#[fastout]
fn main() {
    input! {
        {{_cursor_}}
    }
}
