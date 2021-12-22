use rand::prelude::*;


const MIN_SECRET: u32 = 0;
const MAX_SECRET: u32 = 10;


fn main() {
    let mut rng = rand::thread_rng();
    let secret: u32 = rng.gen_range(MIN_SECRET..MAX_SECRET+1);
    println!("Secret: {}", secret);

    loop {
        let mut buf = String::new();

        std::io::stdin()
            .read_line(&mut buf)
            .expect("InputError");

        let trimmed = buf.trim();
       
        match trimmed.parse::<u32>() {
            Ok(guess) => {
                if guess == secret {
                    println!("Correct!");
                    break;
                } else {
                    println!("Wrong");
                }
            }
            Err(..) => println!("ParseError"),
        }
    }
}
