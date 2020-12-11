use std::thread;
use rand::Rng;
fn main() {
    loop {
        thread::spawn(move || {
           loop {
                let correct_guess: i32 = rand::thread_rng().gen_range(0, 2147483647);
                let guess: i32 = rand::thread_rng().gen_range(0, 2147483647);
                if guess == correct_guess {
                    println!("The correct number is: {}", guess)
                }
           }
        });
    }
}
