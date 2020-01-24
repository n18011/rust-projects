use std::time::{Instant};
fn main() {
    let start = Instant::now();
    // ここから実行速度を確かめたい関数を実行する
    println!("Hello, world!");
    // ここまでの間に
    let end = start.elapsed();
    println!("{}.{:08}sec", end.as_secs(), end.subsec_nanos());
}
