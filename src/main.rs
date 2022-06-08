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

    // 第2回(7)
    // let x = 5;ではなく、
    let mut x = 5;
    println!("{}", x);
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

    //第3回目（8）
    //アルファベットは１bytes
    //日本語は、３bytes
    let s1 = "helloこんにちわ挨拶"; // 5bytes(英語5文字) + 21byte(日本語７文字)　= 26bytes
    let s2 = "hello";
    println!("Stack adreess of s1 is {:p}", &s1); //0x7ff7bb82bf60 -> 60(16進数)を10進数になおすと、96
    println!("Stack adreess of s2 is {:p}", &s2); //0x7ff7bb82bf70 -> 70(16進数)を10進数になおすと、112
                                                  //２つは、16個分のアドレスのずれがある。

    println!("Static memory address of s1: {:?}", s1.as_ptr()); //0x10e38f5e2のアドレスの先頭に入っている
    println!("Static memory address of s2: {:?}", s2.as_ptr()); //0x10e38f5fcのアドレスの先頭に入っている

    println!("Len of s1 is: {}", s1.len());
    println!("Len of s2 is: {}", s2.len());

    let mut s1 = String::from("hello");
    let mut s2 = String::from("helloworld");

    println!("Stack addresss of s1 is {:p}", &s1);
    println!("Stack addresss of s2 is {:p}", &s2);

    //実データのヒープアドレスが取得できる
    println!("Heap memory address of s1 {:?}", s1.as_ptr());
    println!("Heap memory address of s2 {:?}", s2.as_ptr());

    println!("Len of s1 {:?}", s1.len());
    println!("Len of  of s2 {:?}", s2.len());

    println!("Capacity of s1 {:?}", s1.capacity());
    println!("Capacity of s2 {:?}", s2.capacity());

    s1.push_str("_new1");
    s2.push_str("_new2");

    println!("{} {}", s1, s2);

}
