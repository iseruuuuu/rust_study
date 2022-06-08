
//say関数を使えるようになる
use ferris_says::say; 
//stdout・・・標準出力
use std::io::{stdout, BufWriter};

fn main() {
    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    //上のStringの長さを取得する。
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();

    //出力コマンドについて
    /*

    ・インストールコマンド
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    
    ・Rustのバージョン確認
    rustup -V

    ・Cargoのバージョン確認
    cargo -V

    ・新しいファイルを作るコマンド
    cargo new hello-rust

    ・出力方法
    ①　cd hello-rust
    ②　cargo run

    ・依存関係の追加方法
    ①Cargo.tomlに追加したいものを入れる
    ②　cargo build
    */


    // 第2回
    // let x = 5;ではなく、
    let mut x = 5;
    println!("{}",x);
    x = 6;
    println!("{}", x);

    //定数もある
    const MAX_POINTS: u32 = 100_00;
    println!("{}", MAX_POINTS);
    let a: i32 = 10;
    println!("{}", a);

    //シャドーイングがある
    let x = 5;

    let x = x + 1;
    let x = x * 2;
 
    println!("{}", x);
    println!("{}", x);

}