use std::io::{self, Write};
use std::env;
use std::{thread, time};
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let title = " Welcome to the guessing game ";
    let light_on = "@";
    let light_off = "*";
    let blink_period = time::Duration::from_millis(250);

    let lower = 1;
    let upper = 1000;
    let target = rand::thread_rng().gen_range(lower, upper+1);
    let max_tries = ((upper-lower) as f64).sqrt() as i64;


    let args: Vec<String> = env::args().collect();
    let mut is_testing = false;
    if args.len() > 1 && args[1] == "test"{
        println!("In testing mode!");
        is_testing = true;
    }

    println!("\n\n\n");

    if ! is_testing {
        for x in 1..16 {
            if x % 2 == 1 {
                print!("\r{}{:^32}{}",light_off, title, light_off);
                io::stdout().flush().unwrap();
                thread::sleep(blink_period);
            } else {
                print!("\r{}{:^32}{}",light_on, title, light_on);
                io::stdout().flush().unwrap();
                thread::sleep(blink_period);
            }
        }
        println!();
    }

    println!("I have guessed a number between {} and {}!\nCan you guess my number ?\nYou get {} tries to guess the number!", lower, upper, max_tries);

    if is_testing{
        println!("Target number {}", target)
    }

    let mut num_tries = 0;
    let mut has_won = false;
    while num_tries <= max_tries {
        num_tries += 1;
        print!("{} / {} | Guess the number ! ", num_tries, max_tries);
        io::stdout().flush().unwrap();
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                for x in 1..4 {
                    print!("Only numbers please!");
                    io::stdout().flush().unwrap();
                    thread::sleep(blink_period);
                }
                println!();
                num_tries -= 1;
                continue;
            },
        };

        print!("Your guess of {} was : ", guess);
        match guess.cmp(&target) {
            Ordering::Less => print!("too small!"),
            Ordering::Greater => print!("too big!"),
            Ordering::Equal => {
                println!("just right!");
                has_won = true;
            } 
        }

        io::stdout().flush().unwrap();
        println!();
        
        if has_won {
            break;
        }
    }

    if has_won {
        println!("Congrats! You just won a million guessing game bux!");
    } else {
        println!("Sorry, better luck next time!");
    }

    println!();
}
