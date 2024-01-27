use std::env;
use std::error::Error;
use factorial_calculator::*;
use indicatif::ProgressBar;
use rug::Integer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let arguments = interpret_arguments(env::args().collect()).unwrap_or_else(|error| {
        eprintln!("{error}");
        println!("Hint: use --help if you don't know how this tool works");
        std::process::exit(1);
    });


    let target = arguments.target_number;
    let save_step = arguments.save_step;


    let closest_calculated_number = get_closest_calculated_number(target).await.unwrap_or((0, Integer::from(1)));
    println!("Calculating {target}!, starting from {}!", closest_calculated_number.0);

    let progress_bar = ProgressBar::new(target);
    progress_bar.set_position(closest_calculated_number.0);

    let mut factorial = closest_calculated_number.1;

    for x in closest_calculated_number.0 + 1..=target {
        factorial *= x;
        if x % 10000 == 0 {
            progress_bar.set_position(x);
        }
        if save_step.is_some_and(|save_step| x % save_step == 0 && x != target) {
            save_factorial(x, &factorial).await.map_err(|error| error.to_string())?;
        }
    }

    save_factorial(target, &factorial).await.map_err(|error| error.to_string())?;

    Ok(())
}
