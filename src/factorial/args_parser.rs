use std::fmt;
use std::fs;
use termimad::MadSkin;

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
        let text = fs::read_to_string("./src/factorial/help.md").unwrap_or("Help file not found, good luck figuring out how this works lmao".to_string());
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

