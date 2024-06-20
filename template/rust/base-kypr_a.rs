#!/usr/bin/env rust-script

//! ```cargo
//! [dependencies]
//! proconio={version="*", features=["derive"]}
//! ```

#[proconio::fastout]
fn main() {
    use proconio::marker::*;
    proconio::input! {
        {{_cursor_}}
    }
}
