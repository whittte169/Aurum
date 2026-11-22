use std::os::windows::process::CommandExt;
use std::path::Path;
use std::process::Command;
use std::time::Duration;
use serde::Deserialize;

#[allow(dead_code)]
const GITHUB_RELEASE_URL: &str =
    "https://api.github.com/repos/whittte169/Aurum/releases/latest";

#[allow(dead_code)]
#[derive(Deserialize)]
struct GitHubRelease {
    tag_name: String,
    html_url: String,
}

pub async fn execute_command(
    command: &str,
    current_dir: &Path,
) -> Result<String, String> {
    let script = format!(
        "[Console]::OutputEncoding = New-Object System.Text.UTF8Encoding; $OutputEncoding = [Console]::OutputEncoding; {}",
        command
    );

    let output = Command::new("powershell.exe")
        .args([
            "-NoLogo",
            "-NoProfile",
            "-NonInteractive",
            "-ExecutionPolicy",
            "Bypass",
            "-Command",
            &script,
        ])
        .current_dir(current_dir)
        .creation_flags(0x08000000)
        .output()
        .map_err(|error| {
            format!("Could not start PowerShell: {}", error)
        })?;

    let mut result = String::from_utf8_lossy(
        &output.stdout,
    )
    .to_string();

    let stderr = String::from_utf8_lossy(
        &output.stderr,
    );

    if !stderr.trim().is_empty() {
        if !result.is_empty() && !result.ends_with('\n') {
            result.push('\n');
        }

        result.push_str(&stderr);
    }

    if result.is_empty() {
        result = if output.status.success() {
            "Command completed.".to_string()
        } else {
            format!(
                "Command exited with code {:?}.",
                output.status.code()
            )
        };
    }

    Ok(result)
}

#[allow(dead_code)]
pub fn check_for_updates(
) -> Result<Option<(String, String)>, String> {
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(15))
        .build()
        .map_err(|error| error.to_string())?;

    let response = client
        .get(GITHUB_RELEASE_URL)
        .header(
            reqwest::header::USER_AGENT,
            "Aurum-Terminal",
        )
        .header(
            reqwest::header::ACCEPT,
            "application/vnd.github+json",
        )
        .send()
        .map_err(|error| {
            format!("GitHub connection failed: {}", error)
        })?;

    if response.status() == reqwest::StatusCode::NOT_FOUND {
        return Ok(None);
    }

    if !response.status().is_success() {
        return Err(format!(
            "GitHub returned HTTP status {}",
            response.status()
        ));
    }

    let release: GitHubRelease = response
        .json()
        .map_err(|error| {
            format!("Could not parse release information: {}", error)
        })?;

    Ok(Some((
        release.tag_name,
        release.html_url,
    )))
}
