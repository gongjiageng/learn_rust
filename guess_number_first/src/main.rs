// 猜测随机数
use std::io;
fn main() {
    println!("猜数：");
    
    println!("猜测一个数：");

    // let mut foo = 1;
    // let bar = foo;
    // foo = 1;
    
    
    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("无法读取行");

    println!("你猜测的数是：{}", guess);


}