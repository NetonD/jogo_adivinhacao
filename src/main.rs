use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 100);
    println!("Digit your guess: ");
    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Fail to read guess");

        let guess: i32 = match guess.trim().parse(){
            Ok(num)=> num,
            Err(_) => continue,
        };
        


        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("Uhuu! You win!");
                break;
            },
            Ordering::Greater => println!("So big."),
            Ordering::Less => println!("It's small"),
        };
        print!("Digit your guess: ");
    }
}
