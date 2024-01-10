use std::env;
use std::ops::MulAssign;
use factorial_calculator::*;
use indicatif::ProgressBar;

const DIRECTORY: &str = "factorials";

fn main() {
    let args: Vec<String> = env::args().collect();
    let num = args.get(1).expect("1 argument is required to run the program").parse::<u64>().expect("First argument must be a number!");
    let save_interval = args.get(2).map(|arg| arg.parse::<u64>().expect("Second argument must be a number, if present"));


    let closest_calculated_number = get_closest_calculated_number(num, DIRECTORY);
    println!("Calculating {num}!, starting from {}!", closest_calculated_number.0);

    let progress_bar = ProgressBar::new(num);
    progress_bar.set_position(closest_calculated_number.0);

    let mut factorial = closest_calculated_number.1;
    for x in closest_calculated_number.0 + 1..=num {
        factorial.mul_assign(x);
        if x % 10000 == 0 {
            progress_bar.set_position(x);
        }
        if save_interval.is_some_and(|save_interval| x % save_interval == closest_calculated_number.0 % save_interval) {
            save_factorial(x, &factorial, DIRECTORY).expect(&format!("Could not save file '{DIRECTORY}/{x}.txt'"));            
        }
    }

    save_factorial(num, &factorial, DIRECTORY).expect(&format!("Could not save file '{DIRECTORY}/{num}.txt'"));
}

