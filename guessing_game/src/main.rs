use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main()
{
    let rng;
    let mut guess;

    println!("Guess the number!");


    rng = rand::thread_rng()
        .gen_range(1..=100);


    loop
    {
        println!("Please input your guess:");
        guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess
            .trim()
            .parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

        match guess.cmp(&rng)
        {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You Win!");
                break ;
            }
        }
    }
}
