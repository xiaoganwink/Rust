use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("Guess the number!");
    let secret_number=rand::thread_rng().gen_range(1,101);
    //println!("The secret_numeber is {}",secret_number);
    println!("please input your number");
    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line");

        //println!("your guessed is {}", guess);
        let guess: u32 =  match guess.trim().parse(){
            Ok(num)=>num,
            Err(_)=>continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;}
            };
        }
    }


