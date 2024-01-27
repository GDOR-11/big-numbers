use reqwest;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};
use std::time::SystemTime;
use std::error::Error;
use std::fmt::{self, Display};

#[derive(Debug)]
pub enum RemoteError {
    WorkingTreeNotClean,
    FileCreationError(std::io::Error),
    FileDeletionError(std::io::Error),
    GithubRequestError(reqwest::Error),
    GitExecutionError(std::io::Error)
}

impl Display for RemoteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WorkingTreeNotClean => write!(f, "working tree not clean"),
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
            Self::FileCreationError(error) => Some(error),
            Self::FileDeletionError(error) => Some(error),
            Self::GithubRequestError(error) => Some(error),
            Self::GitExecutionError(error) => Some(error)
        }
    }
}

pub async fn read_file(file_path: &str) -> Result<String, RemoteError> {
    // use the current time to generate a new token with every request
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();

    reqwest::get(
        format!("https://raw.githubusercontent.com/GDOR-11/factorial-calculator/main/{file_path}?token={:?}", now)
    ).await
    .map_err(|error| RemoteError::GithubRequestError(error))?
    .text().await
    .map_err(|error| RemoteError::GithubRequestError(error))
}
pub fn write_file(file_path: &str, file_content: &str) -> Result<(), RemoteError> {
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
        .and_then(|mut file| file.write_all(file_content.as_bytes()))
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
        .status()
        .map_err(|error| RemoteError::GitExecutionError(error))?;

    Ok(())
}
