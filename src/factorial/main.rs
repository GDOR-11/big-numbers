mod args_parser;
mod factorial_calculator;

use std::env;
use std::error::Error;
use std::time::Instant;
use args_parser::*;
use big_numbers::*;
use indicatif::ProgressBar;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let arguments = interpret_arguments(env::args().collect()).unwrap_or_else(|error| {
        eprintln!("{error}");
        println!("Hint: use --help if you don't know how this tool works");
        std::process::exit(1);
    });

    let target = arguments.target_number;
    let save_step = arguments.save_step;

    let progress_bar = ProgressBar::new(target as u64);

    let start_time = Instant::now();

    let mut number = if let Some(savestep) = save_step { savestep } else { target };
    while number <= target {
        progress_bar.println(format!("calculating {number}!"));

        let factorial = factorial_calculator::factorial(number);
        save_number(&format!("{number}!"), &factorial).await.map_err(|error| error.to_string())?;

        progress_bar.set_position(number as u64);

        number += if let Some(savestep) = save_step { savestep } else { 1 };
    }
    println!("done in {:?}", Instant::now() - start_time);


    Ok(())
}
