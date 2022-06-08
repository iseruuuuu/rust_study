//say関数を使えるようになる
use ferris_says::say;
//stdout・・・標準出力
use std::io::{stdout, BufWriter};

mod stack_heap;

fn main() {
    stack_heap::run();
}
