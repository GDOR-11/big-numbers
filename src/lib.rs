use std::fmt;
use std::fs::{self, File};
use std::io::Write;
use rug::Integer;
use termimad::MadSkin;
use reqwest;

pub struct CLIArguments {
    pub target_number: u64,
    pub save_step: Option<u64>,
    pub use_remote_files: bool
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
            Self::InvalidSyntax(string) => write!(f, "Syntax error: {}", string),
            Self::IncorrectArguments(string) => write!(f, "Arguments error: {}", string)
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

    let (mut target_number, mut save_step, mut use_remote_files) = (None, None, None);
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
            "use-git-sparse-checkout" => {
                use_remote_files = match value {
                    "yes" | "true" | "easter egg :D" => Some(true),
                    "no" | "false" => Some(false),
                    _ => return Err(CLIArgumentsError::IncorrectArguments(format!("use-remote-files must be 'yes'/'true' or 'no'/'false'")))
                }
            },
            _ => return Err(CLIArgumentsError::IncorrectArguments(format!("Argument '{name}' does not exist")))
        }
    }
    let Some(target_number) = target_number else {
        return Err(CLIArgumentsError::IncorrectArguments(format!("target number is missing")));
    };
    let use_remote_files = use_remote_files.unwrap_or(false);

    Ok(CLIArguments { target_number, save_step, use_remote_files })
}

pub fn filepath(directory: &str, number: u64) -> String {
    format!("{directory}/{number}/{number}.txt")
}

async fn read_file(filepath: &str, from_remote: bool) -> Option<String> {
    if from_remote {
        reqwest::get(
            format!("https://raw.githubusercontent.com/GDOR-11/factorial-calculator/main/{filepath}")
        ).await.ok()?.text().await.ok()
    } else {
        fs::read_to_string(filepath).ok()
    }
}

pub async fn get_closest_calculated_number(number: u64, directory: &str, use_remote_files: bool) -> Option<(u64, Integer)> {
    let calculated_nums: Vec<u64> =
        read_file(&format!("{directory}/factorials/factorials.txt"), use_remote_files).await?
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
        &read_file(&filepath(directory, closest_calculated_num), use_remote_files).await?,
        10
    ).ok()?;

    Some((closest_calculated_num, factorial))
}

pub fn save_factorial(number: u64, factorial: &Integer, directory: &str) -> Result<(), std::io::Error> {
    if let Some(directory) = std::path::Path::new(&filepath(directory, number)).parent() {
        fs::create_dir_all(directory)?;
    }
    File::create(filepath(directory, number))?.write_all(factorial.to_string().as_bytes())
}
