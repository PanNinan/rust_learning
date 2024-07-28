use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("猜数!");
    let mut rng = rand::thread_rng();
    let random_number = rng.gen_range(0..=100);

    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("无法读取行!");

        let guess: i64 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess.cmp(&random_number) {
            Ordering::Less => println!("小了"),
            Ordering::Greater => println!("大了"),
            Ordering::Equal => {
                println!("对了");
                break;
            }
        }
    }
}
