use crate::local_files_handler;
use reqwest;
use std::path::Path;
use std::process::{Command, Stdio};
use std::time::SystemTime;

#[derive(Debug)]
pub enum RemoteError {
    WorkingTreeNotClean,
    LocalError(std::io::Error),
    GithubRequestError(reqwest::Error),
    GitExecutionError(std::io::Error),
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
    if Command::new("git")
        .args(["log", "--branches", "--not", "--remotes"])
        .output()
        .map_err(|error| RemoteError::GitExecutionError(error))?
        .stdout
        .len()
        != 0
    {
        return Err(RemoteError::WorkingTreeNotClean);
    }
    local_files_handler::write_file(Path::new(file_path), file_content)
        .map_err(|error| RemoteError::LocalError(error))?;

    // git reset
    // git add --sparse <file path>
    // git commit -m "Adding files automatically"
    // git push origin main
    // rm -rf <parent directory>
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

    local_files_handler::delete_path(
        Path::new(file_path)
            .parent()
            .expect("dont mess with the code"),
    )
    .map_err(|error| RemoteError::LocalError(error))?;

    Command::new("git")
        .args(["sparse-checkout", "reapply"])
        .status()
        .map_err(|error| RemoteError::GitExecutionError(error))?;

    Ok(())
}
