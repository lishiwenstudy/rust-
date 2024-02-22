use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("猜数!");
    let number = rand::thread_rng().gen_range(1..101);
    println!("神秘数字是:{}",number);
    println!("猜测一个数");
    
    let mut guess = String::new();
    
    io::stdin().read_line(&mut guess).expect("无法读取行");
    let guess:u32 = guess.trim().parse().expect("please type a number!");
    println!("你猜测的数是: {}",guess);
    //新增代码，比较猜数与随机数的大小
    match guess.cmp(&number){
        Ordering::Less=>println!("small"),
        Ordering::Greater=>println!("big"),
        Ordering::Equal=>println!("win"),
    }
}
