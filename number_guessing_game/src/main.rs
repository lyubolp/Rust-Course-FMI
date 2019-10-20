use std::io::stdin;
use rand::Rng;

fn read_number() -> u32
{
    let stdin = stdin();
    let mut line: String = String::new();
    stdin.read_line(&mut line);

    line.trim().parse().unwrap()
}
fn main() {

    let secret_number = rand::thread_rng().gen_range(0, 10_u32);
    println!("I've generated a number between 0 and 100. Try to guess it !");

    let tries:u32 = 5;

    println!("You will have {} tries", tries);

    let mut is_guessed:bool = false;
    for _ in 0..tries{
        println!("Please enter a number");
        let user_guess: u32 = read_number();

        if user_guess < secret_number{
            println!("The number you are looking for is higher");
        }
        else if user_guess > secret_number{
            println!("The number you are looking for is lower");
        }
        else{
            println!("Good job, you guessed it !");
            is_guessed = true;
            break;
        }
    }

    if !is_guessed{
        println!("You failed, the number was {}", secret_number);
    }
}
