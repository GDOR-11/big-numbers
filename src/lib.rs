use reqwest::{self, StatusCode};
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};
use std::time::SystemTime;
use std::error::Error;
use std::fmt::{self, Display};
use rug;

#[derive(Debug)]
pub enum RemoteError {
    WorkingTreeNotClean,
    FileTooLarge,
    FileNotFound,
    FileCreationError(std::io::Error),
    FileDeletionError(std::io::Error),
    GithubRequestError(reqwest::Error),
    GitExecutionError(std::io::Error)
}

impl Display for RemoteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WorkingTreeNotClean => write!(f, "working tree not clean"),
            Self::FileTooLarge => write!(f, "the number is too big to be saved on github"),
            Self::FileNotFound => write!(f, "the file has not been found"),
            Self::FileCreationError(error) => write!(f, "could not create local file ({error})"),
            Self::FileDeletionError(error) => write!(f, "could not delete local file ({error})"),
            Self::GithubRequestError(error) => write!(f, "could not get file data from github ({error})"),
            Self::GitExecutionError(error) => write!(f, "could not use git ({error})")
        }
    }
}

impl Error for RemoteError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::WorkingTreeNotClean => None,
            Self::FileTooLarge => None,
            Self::FileNotFound => None,
            Self::FileCreationError(error) => Some(error),
            Self::FileDeletionError(error) => Some(error),
            Self::GithubRequestError(error) => Some(error),
            Self::GitExecutionError(error) => Some(error)
        }
    }
}

pub fn number_filepath(number_title: &str, binary: bool) -> String {
    if binary {
        format!("binary-bigints/{number_title}/{number_title}.bigint")
    } else {
        format!("decimal-bigints/{number_title}/{number_title}.txt")
    }
}

pub async fn save_number(number_title: &str, number: &rug::Integer) -> Result<(), RemoteError> {
    let digits = number.significant_bits() as f64 * 0.30103; // 0.30103 > log10(2),
                                                             // therefore digits > actual digits

    if digits >= 104857600.0 {
        return Err(RemoteError::FileTooLarge);
    }
    if digits <= 52428800.0 {
        let file_path = &number_filepath(number_title, false);
        write_file(file_path, number.to_string().as_bytes())?;
    }

    let file_path = &number_filepath(number_title, true);
    let base256 = number.to_digits(rug::integer::Order::Msf);
    write_file(file_path, &base256)?;

    Ok(())
}

pub async fn read_file(file_path: &str) -> Result<Vec<u8>, RemoteError> {
    // use the current time to generate a new token with every request
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();

    let response = reqwest::get(
        format!("https://raw.githubusercontent.com/GDOR-11/factorial-calculator/main/{file_path}?token={:?}", now)
    ).await.map_err(|error| RemoteError::GithubRequestError(error))?;
    if response.status() != StatusCode::OK { return Err(RemoteError::FileNotFound); }

    let bytes = response.bytes().await.map_err(|error| RemoteError::GithubRequestError(error))?;
    Ok(Vec::from(bytes))
}
pub fn write_file(file_path: &str, file_content: &[u8]) -> Result<(), RemoteError> {
    // this won't work if there are unpushed commits,
    // git log --branches --not --remotes will check that for us
    if Command::new("git")
        .args(["log", "--branches", "--not", "--remotes"])
        .output()
        .map_err(|error| RemoteError::GitExecutionError(error))?
        .stdout.len() != 0
    {
        return Err(RemoteError::WorkingTreeNotClean);
    }

    if let Some(parent) = Path::new(file_path).parent() {
        fs::create_dir_all(parent)
            .map_err(|error| RemoteError::FileCreationError(error))?;
    }
    File::create(file_path)
        .and_then(|mut file| file.write_all(file_content))
        .map_err(|error| RemoteError::FileCreationError(error))?;

    // git reset
    // git add --sparse <file path>
    // git commit -m "Adding files automatically"
    // git push origin main
    // rm <file path>
    // rmdir <parent directory>
    // git sparse-checkout reapply

    Command::new("git")
        .arg("reset")
        .stdout(Stdio::null())
        .status()
        .map_err(|error| RemoteError::GitExecutionError(error))?;
    Command::new("git")
        .args(["add", "--sparse", file_path])
        .status()
        .map_err(|error| RemoteError::GitExecutionError(error))?;
    Command::new("git")
        .args(["commit", "-m", "Adding files automatically"])
        .stdout(Stdio::null())
        .status()
        .map_err(|error| RemoteError::GitExecutionError(error))?;
    Command::new("git")
        .args(["push", "origin", "main"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map_err(|error| RemoteError::GitExecutionError(error))?;


    fs::remove_file(file_path)
        .map_err(|error| RemoteError::FileDeletionError(error))?;
    if let Some(parent) = Path::new(file_path).parent() {
        let _ = fs::remove_dir(parent);
    }

    Command::new("git")
        .args(["sparse-checkout", "reapply"])
        .stdout(Stdio::null())
        .status()
        .map_err(|error| RemoteError::GitExecutionError(error))?;

    Ok(())
}
