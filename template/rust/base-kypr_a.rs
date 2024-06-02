#!/usr/bin/env rust-script

//! ```cargo
//! [dependencies]
//! proconio={version="*", features=["derive"]}
//! num="*"
//! num-bigint="*"
//! num-complex="*"
//! num-integer="*"
//! num-iter="*"
//! num-rational="*"
//! num-traits="*"
//! num-derive="*"
//! ndarray="*"
//! nalgebra="*"
//! alga="*"
//! libm="*"
//! rand={version="*", features=["small_rng"]}
//! getrandom="*"
//! rand_chacha="*"
//! rand_core="*"
//! rand_hc="*"
//! rand_pcg="*"
//! rand_distr="*"
//! petgraph="*"
//! indexmap="*"
//! regex="*"
//! lazy_static="*"
//! ordered-float="*"
//! ascii="*"
//! permutohedron="*"
//! superslice="*"
//! itertools="*"
//! itertools-num="*"
//! maplit="*"
//! either="*"
//! im-rc="*"
//! fixedbitset="*"
//! bitset-fixed="*"
//! text_io="*"
//! whiteread="*"
//! rustc-hash="*"
//! smallvec="*"
//! ```


#[proconio::fastout]
fn main() {
    proconio::input! {
        {{_cursor_}}
    }
}

