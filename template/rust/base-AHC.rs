use proconio::{*};
use proconio::marker::{*};
use std::collections::{*};
use petgraph::algo::{*};
use petgraph::graph::{*};
use petgraph::unionfind::{*};
use petgraph::data::{*};
use itertools::{*};
use std::time::Instant;
use std::process;
const END_TIME: u128 = 4950;
#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let start_time = Instant::now();

    // policy


    let stop_time = Instant::now();dbg!(&stop_time.duration_since(start_time));
}

fn timeout(start_time: &Instant) {
    start_time.elapsed().as_millis() > END_TIME
}
