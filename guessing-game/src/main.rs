// use rand::Rng;
// use std::cmp::Ordering;
// use std::io;

// fn main() {
//     // --snip--
//     println!("Guess the number!");

//     let secret_number = rand::thread_rng().gen_range(1..101);

//     loop {
//         println!("Please input your guess.");

//         let mut guess = String::new();

//         io::stdin()
//             .read_line(&mut guess) // the & character indicates that the parameter is a reference, meaning that the is no need to make a copy of it in memory
//             .expect("Failed to read line");


//         //the line below causes the program to crash if the user inputs something other than a number
//         // let guess: u32 = guess.trim().parse().expect("Please type a number!");

//         //in order to prevent the crash, the function below ignores inputs != from numbers
//         let guess: u32 = match guess.trim().parse() {
//             Ok(num) => num,
//             Err(_) => continue,
//         };

//         println!("You guessed: {}", guess);

//         match guess.cmp(&secret_number) {
//             Ordering::Less => println!("Too small!"),
//             Ordering::Greater => println!("Too big!"),
//             Ordering::Equal => {
//                 println!("You win!");
//                 break;
//             }
//         }
//     }
// }

fn main() {}