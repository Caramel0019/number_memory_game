use std::cmp::Ordering;
use std::io;
use std::time::Duration;
use std::thread::sleep;
use rand::Rng;

fn main() {
    let number = rand::thread_rng().gen_range(1000..=9999);

    println!("welcome to number memery remeber the number ");
    println!("number is: {number}");

    sleep(Duration::from_secs(2));
    for _ in 0..50 {
        println!();
    }

    println!("input the number");
    let mut user_number = String::new();

    io::stdin()
        .read_line(&mut user_number)
        .expect("faild to read input");

    let user_number: u32 = match user_number.trim().parse() {
        Ok(num) => num,
        Err(_) => return,
    };

    match user_number.cmp(&number) {
        Ordering::Less => println!("incorrect"),
        Ordering::Greater => println!("incorrect"),
        Ordering::Equal => {
            println!("correct, you win");
            return;
        }
    }

}
