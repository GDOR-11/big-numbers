use std::env;
use factorial_calculator::*;
use indicatif::ProgressBar;
use rug::Integer;


const DIRECTORY: &str = "factorials";

#[tokio::main]
async fn main() {
    let arguments = interpret_arguments(env::args().collect()).unwrap_or_else(|error| {
        eprintln!("{error}");
        println!("Hint: use --help if you don't know how this tool works");
        std::process::exit(1);
    });


    let target = arguments.target_number;
    let save_step = arguments.save_step;
    let use_remote_files = arguments.use_remote_files;

    let save = |number: u64, factorial: &Integer| {
        let file_path = filepath(DIRECTORY, number);

        match save_factorial(number, factorial, DIRECTORY, use_remote_files) {
            Ok(_) => (),
            Err(SaveError::WorkingTreeNotClean) => {
                eprintln!("Could not save file '{}' to remote, as the working tree is not clean", file_path);
                std::process::exit(1);
            },
            Err(SaveError::PathDoesNotExist) => {
                eprintln!("Attempted to do operations on a non-existent file, Exiting early...");
                std::process::exit(1);
            },
            Err(SaveError::IoError(error)) => {
                eprintln!("Could not save file '{}' due to an error:\n{}", file_path, error);
                std::process::exit(1);
            }
        }
    };

    let closest_calculated_number = get_closest_calculated_number(target, DIRECTORY, use_remote_files).await.unwrap_or((0, Integer::from(1)));
    println!("Calculating {target}!, starting from {}!", closest_calculated_number.0);

    let progress_bar = ProgressBar::new(target);
    progress_bar.set_position(closest_calculated_number.0);

    let mut factorial = closest_calculated_number.1;

    for x in closest_calculated_number.0 + 1..=target {
        factorial *= x;
        if x % 10000 == 0 {
            progress_bar.set_position(x);
        }
        if save_step.is_some_and(|save_step| x % save_step == 0) {
            save(x, &factorial);
        }
    }

    save(target, &factorial);
}
