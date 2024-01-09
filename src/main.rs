use std::{env, fs::File, ops::MulAssign, io::Write};

use rug::Integer;
use indicatif::ProgressBar;

const DIRECTORY: &str = "factorials";

fn main() {
    let args: Vec<String> = env::args().collect();
    let num = args.get(1).expect("1 argument is required to run the program").parse::<u64>().expect("First argument must be a number!");

    let filename = &format!("{DIRECTORY}/{num}.txt");

    let mut file = match File::create(filename) {
        Ok(file) => file,
        Err(reason) => panic!("Could not create the file '{filename}'\n\nReason: {reason}")
    };

    let progress_bar = ProgressBar::new(num);

    let mut factorial = Integer::from(1);
    for x in 2..=num {
        factorial.mul_assign(x);
        if x % 10000 == 0 {
            progress_bar.set_position(x);
        }
    }

    match file.write_all(factorial.to_string().as_bytes()) {
        Ok(_) => (),
        Err(reason) => panic!("Could not write to file '{filename}'\n\nReason: {reason}")
    };
}
