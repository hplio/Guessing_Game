//NOTE:  gussing program

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!!");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut gusse = String::new();

        io::stdin()
            .read_line(&mut gusse)
            .expect("Please input your guess");

        let gusse: u32 = match gusse.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match gusse.cmp(&secret_number) {
            Ordering::Less => {
                println!("too small");
                continue;
            }
            Ordering::Greater => {
                println!("too big");
                continue;
            }
            Ordering::Equal => {
                println!("you win!");
                break;
            }
        };
    }
    let check = 10_000;
    println!("the value is : {check}");

    // let mut x = 10;
    // let r = &mut x;
    //
    // *r += 10;
    //
    // println!("x is: {x}");
    // println!("y is: {}",&r);
}
