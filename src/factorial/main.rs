mod args_parser;

use std::env;
use std::error::Error;
use args_parser::*;
use big_numbers::*;
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
    println!("Calculating {target}!");

    let progress_bar = ProgressBar::new(target);

    let mut factorial = Integer::from(1);
    for x in 2..=target {
        factorial *= x;
        if x % 10000 == 0 {
            progress_bar.set_position(x);
        }
        if save_step.is_some_and(|save_step| x % save_step == 0 && x != target) {
            save_number(&format!("{x}!"), &factorial).await.map_err(|error| error.to_string())?;
        }
    }

    save_number(&format!("{target}!"), &factorial).await.map_err(|error| error.to_string())?;

    Ok(())
}
