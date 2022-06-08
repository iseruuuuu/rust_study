pub fn run() {
    //5回目（１０）
    //1の値を700000個収納している。
    // let a1: [u8; 9000000] = [1; 9000000];
    //stack内には8MBまでしか格納できない

    //ベクター型
    let mut v1 = vec![1, 2, 3, 4];
    let v2 = vec![5, 6, 7, 8];
    let mut v3 = vec![9, 10];

    //アンパサント？？でアドレスを取得する
    println!("Stack address of v1 is: {:p}", &v1); //0x7ff7b69e4f38
    println!("Stack address of v2 is: {:p}", &v2); //0x7ff7b69e4f50

    //ポインタの値を取得
    //Heap内の実データのアドレス
    println!("Heap memory address of v1: {:?}", v1.as_ptr());
    println!("Len of v1: {:?}", v1.len());
    println!("Capacity memory address of v1: {:?}", v1.capacity());

    //値を置換する
    v1.insert(1, 10);
    println!("{:?}", v1);
    //値を削除する
    v1.remove(0);
    println!("{:?}", v1);

    //v3をv1に連結する
    v1.append(&mut v3);
    println!("{:?}", v1);
    //値を渡したから値がなくなる
    println!("{:?}", v3);
}
