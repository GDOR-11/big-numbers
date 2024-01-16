use std::env;
use std::ops::MulAssign;
use factorial_calculator::*;
use indicatif::ProgressBar;
use rug::Integer;


const DIRECTORY: &str = "factorials";

#[tokio::main]
async fn main() {
    let arguments = interpret_arguments(env::args().collect()).unwrap_or_else(|error| {
        eprintln!("{error}");
        println!("Hint: use --help if you don't know how this tool works");
        std::process::exit(0);
    });

    let target = arguments.target_number;
    let save_step = arguments.save_step;
    let use_remote_files = arguments.use_remote_files;


    let closest_calculated_number = get_closest_calculated_number(target, DIRECTORY, use_remote_files).await.unwrap_or((0, Integer::from(1)));
    println!("Calculating {target}!, starting from {}!", closest_calculated_number.0);

    let progress_bar = ProgressBar::new(target);
    progress_bar.set_position(closest_calculated_number.0);

    let mut factorial = closest_calculated_number.1;
    for x in closest_calculated_number.0 + 1..=target {
        factorial.mul_assign(x);
        if x % 10000 == 0 {
            progress_bar.set_position(x);
        }
        if save_step.is_some_and(|save_step| x % save_step == 0) {
            save_factorial(x, &factorial, DIRECTORY, use_remote_files).expect(&format!("Could not save file '{}'", filepath(DIRECTORY, x)));            
        }
    }

    save_factorial(target, &factorial, DIRECTORY, use_remote_files).expect(&format!("Could not save file '{}'", filepath(DIRECTORY, target)));
}
