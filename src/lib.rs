mod remote_files_handler;

use std::fmt;
use std::fs;
use rug::Integer;
use termimad::MadSkin;

pub fn factorial_path(number: u64) -> String {
    format!("factorials/{number}/{number}.txt")
}
pub const FACTORIALS_LIST_PATH: &str = "factorials/list/list.txt";

pub struct CLIArguments {
    pub target_number: u64,
    pub save_step: Option<u64>
}
pub enum CLIArgumentsError {
    InvalidSyntax(String),
    IncorrectArguments(String)
}
impl CLIArgumentsError {
    fn invalid_syntax_default() -> Self {
        Self::InvalidSyntax(String::from("Arguments must come in the form '<name>=<value>'"))
    }
}
impl fmt::Display for CLIArgumentsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidSyntax(string) => write!(f, "Syntax error: {string}"),
            Self::IncorrectArguments(string) => write!(f, "Arguments error: {string}")
        }
    }
}
pub fn interpret_arguments(args: Vec<String>) -> Result<CLIArguments, CLIArgumentsError> {
    if args.get(1).is_some_and(|arg| arg == "--help" || arg == "-h") {
        let text = fs::read_to_string("./help.md").unwrap_or("Help file not found, good luck figuring out how this works lmao".to_string());
        let skin = MadSkin::default();
        println!("{}", skin.term_text(&text));
        std::process::exit(0);
    }

    let (mut target_number, mut save_step) = (None, None);
    for arg in args.split_at(1).1 {
        let Some((name, value)) = arg.split_once('=') else {
            return Err(CLIArgumentsError::invalid_syntax_default());
        };
        match name {
            "target" => {
                target_number = value.parse::<u64>().ok();
                if target_number == None {
                    return Err(CLIArgumentsError::IncorrectArguments(format!("target must be a non-negative integer less than or equal to {}", u64::MAX)))
                }
            },
            "save-step" => {
                save_step = value.parse::<u64>().ok();
                if save_step == None || save_step == Some(0) {
                    return Err(CLIArgumentsError::IncorrectArguments(format!("save-step must be a non-negative integer less than or equal to {}", u64::MAX)))
                }
            },
            _ => return Err(CLIArgumentsError::IncorrectArguments(format!("Argument '{name}' does not exist")))
        }
    }
    let Some(target_number) = target_number else {
        return Err(CLIArgumentsError::IncorrectArguments(format!("target number is missing")));
    };

    Ok(CLIArguments { target_number, save_step })
}


pub async fn get_closest_calculated_number(number: u64) -> Option<(u64, Integer)> {
    let calculated_nums: Vec<u64> =
        remote_files_handler::read_file(FACTORIALS_LIST_PATH).await.ok()?
        .split('\n')
        .filter_map(|str| str.parse::<u64>().ok())
        .collect();

    let mut closest_calculated_num = None;
    for calculated_num in calculated_nums {
        if calculated_num <= number && !closest_calculated_num.is_some_and(|num| calculated_num < num) {
            closest_calculated_num = Some(calculated_num);
        }
    }
    let Some(closest_calculated_num) = closest_calculated_num else { return None; };

    let factorial = Integer::from_str_radix(
        &remote_files_handler::read_file(&factorial_path(closest_calculated_num)).await.ok()?,
        36
    ).ok()?;

    Some((closest_calculated_num, factorial))
}

pub async fn save_factorial(number: u64, factorial: &Integer) -> Result<(), remote_files_handler::RemoteError> {
    let file_path = &factorial_path(number);

    let mut factorials = remote_files_handler::read_file(&FACTORIALS_LIST_PATH).await?;
    if factorials
        .split('\n')
        .find(|string| string == &number.to_string())
        .is_some()
    {
        return Ok(());
    }

    factorials.push_str(&format!("\n{number}"));

    remote_files_handler::write_file(file_path, &factorial.to_string_radix(36))?;
    remote_files_handler::write_file(FACTORIALS_LIST_PATH, &factorials)?;
    
    Ok(())
}
