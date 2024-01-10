use std::fs::{self, File};
use std::io::Write;
use rug::Integer;

pub fn get_closest_calculated_number(number: u64, directory: &str) -> (u64, Integer) {
    let calculated_nums: Vec<u64> = match fs::read_dir(directory) {
        Ok(paths) => paths,
        Err(reason) => panic!("Could not read directory '{directory}'\n\nReason: {reason}")
    }.filter_map(|path| {
        let path = String::from(path.ok()?.path().to_str()?);
        path[directory.len() + 1..path.len() - 4].parse::<u64>().ok()
    }).collect();

    let mut closest_calculated_num = None;
    for calculated_num in calculated_nums {
        if calculated_num < number && !closest_calculated_num.is_some_and(|num| calculated_num < num) {
            closest_calculated_num = Some(calculated_num);
        } else if calculated_num == number {
            println!("File already exists!");
            std::process::exit(0);
        }
    }

    if closest_calculated_num.is_none() { return (0, Integer::from(1)); }
    let closest_calculated_num = closest_calculated_num.unwrap();

    match fs::read_to_string(&format!("{directory}/{closest_calculated_num}.txt")) {
        Ok(string) => {
            println!("Found file with {closest_calculated_num} factorial!");
            println!("Reading file...");
            let factorial = Integer::from_str_radix(&string, 10);
            if let Ok(factorial) = factorial {
                println!("File read successfully!");
                (closest_calculated_num, factorial)
            } else {
                println!("File was invalid");
                (0, Integer::from(1))
            }
        },
        Err(_) => (0, Integer::from(1))
    }
}

pub fn save_factorial(number: u64, factorial: &Integer, directory: &str) -> Result<(), std::io::Error> {
    File::create(format!("{directory}/{number}.txt"))?.write_all(factorial.to_string().as_bytes())
}
