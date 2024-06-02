use anyhow::{anyhow, Error, Result};
use flate2::read::GzDecoder;
use regex::Regex;
use reqwest::{blocking::Client, blocking::Response};
use std::cmp::Ordering;
use std::cmp::PartialOrd;
use std::env;
use std::fmt;
use std::fs::File;
use tar::Archive;

const LATEST_RELEASES_URL: &str = "https://api.github.com/repos/oniku-2929/tagcm/releases/latest";
const TMP_DL_ARCHIVE: &str = "_tagcm.tar.gz";

#[derive(Debug, PartialEq)]
struct Semver {
    major: u64,
    minor: u64,
    patch: u64,
}

impl Semver {
    fn new(version: &str) -> Result<Self, Error> {
        let version = version.trim_start_matches('v');

        let parts: Vec<&str> = version.split('.').collect();
        if parts.len() != 3 {
            return Err(anyhow!("invalid version"));
        }

        let major = parts[0].parse::<u64>();
        let minor = parts[1].parse::<u64>();
        let patch = parts[2].parse::<u64>();

        if major.is_err() || minor.is_err() || patch.is_err() {
            return Err(anyhow!("invalid version digits"));
        }

        Ok(Semver {
            major: major.unwrap(),
            minor: minor.unwrap(),
            patch: patch.unwrap(),
        })
    }
}

impl fmt::Display for Semver {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "v{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl PartialOrd for Semver {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        match self.major.cmp(&rhs.major) {
            Ordering::Equal => match self.minor.cmp(&rhs.minor) {
                Ordering::Equal => Some(self.patch.cmp(&rhs.patch)),
                other => Some(other),
            },
            other => Some(other),
        }
    }
}

#[derive(Debug)]
struct DownloadBinary {
    url: String,
}

impl DownloadBinary {
    fn new() -> Result<Self, Error> {
        match std::env::consts::OS {
            "macos" => {
                let url = format!("tagcm-{}-apple-darwin.tar.gz", std::env::consts::ARCH);
                Ok(DownloadBinary {
                    url: url.to_string(),
                })
            }
            "windows" => {
                let url = format!(
                    "tagcm-{}-pc-windows-msvc.exe.tar.gz",
                    std::env::consts::ARCH
                );
                Ok(DownloadBinary {
                    url: url.to_string(),
                })
            }
            "linux" => {
                let url = format!("tagcm-{}-unknown-linux-gnu.tar.gz", std::env::consts::ARCH);
                Ok(DownloadBinary {
                    url: url.to_string(),
                })
            }
            _ => {
                println!("unsupported OS: {}", std::env::consts::OS);
                Err(anyhow!("unsupported OS"))
            }
        }
    }

    fn get_url(&self) -> String {
        self.url.clone()
    }

    fn get_binary_name(&self) -> String {
        return self.url.split('.').collect::<Vec<&str>>()[0].to_string();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_semver_greater() {
        let v1 = Semver::new("v0.1.0").unwrap();
        let v2 = Semver::new("v0.1.1").unwrap();
        let v3 = Semver::new("v0.2.0").unwrap();
        let v4 = Semver::new("v1.0.0").unwrap();

        assert!(v1 < v2);
        assert!(v2 < v3);
        assert!(v3 < v4);
        assert!(v1 < v4);
    }

    #[test]
    fn test_semver_is_same() {
        let v1 = Semver::new("v0.1.0").unwrap();
        let v2 = Semver::new("v0.1.0").unwrap();
        assert!(v1 == v2);

        let v3 = Semver::new("v0.1.1").unwrap();
        assert!(v1 != v3);
    }

    #[test]
    fn test_semver_invalid() {
        let v1 = Semver::new("v0.1").unwrap_err();
        let v2 = Semver::new("v0.1.0.0").unwrap_err();
        let v3 = Semver::new("v0.").unwrap_err();
        let v4 = Semver::new("va.b.c").unwrap_err();

        assert_eq!(v1.to_string(), "invalid version");
        assert_eq!(v2.to_string(), "invalid version");
        assert_eq!(v3.to_string(), "invalid version");

        assert_eq!(v4.to_string(), "invalid version digits");
    }

    #[test]
    fn test_download_binary() {
        DownloadBinary::new().unwrap();
    }
}

pub fn update(current_version: &str) -> Result<()> {
    let client: Client = Client::builder().use_rustls_tls().build()?;
    let resp: Response = client
        .get(LATEST_RELEASES_URL)
        .header("charset", "UTF-8")
        .header("User-Agent", "tagcm")
        .header("Accept", "application/vnd.github.text+json")
        .send()?
        .error_for_status()?;

    let txt = resp.text()?;
    let re = Regex::new(r"v[0-9]{1,}\.[0-9]{1,}\.[0-9]{1,}").unwrap();
    let matches = re.find(&txt).unwrap();

    println!("current version: \"v{}\"", current_version);
    println!("latest version: {:?}", matches.as_str());

    let latest_version = Semver::new(matches.as_str())?;
    let current_version = Semver::new(current_version)?;
    if latest_version <= current_version {
        println!("this is the latest version");
        return Ok(());
    }

    println!(
        "latest version is available, update to {}? [y/n]",
        latest_version
    );

    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    if input.trim() != "y" {
        println!("update canceled");
        return Ok(());
    }

    let mut binary = std::fs::File::create(TMP_DL_ARCHIVE)?;
    let download_binary = DownloadBinary::new()?;
    let target_binary = format!(
        "https://github.com/oniku-2929/tagcm/releases/download/{}/{}",
        latest_version,
        download_binary.get_url()
    );
    reqwest::blocking::get(target_binary)?
        .error_for_status()?
        .copy_to(&mut binary)?;

    let tar_gz = File::open(TMP_DL_ARCHIVE)?;
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    archive.unpack(".")?;

    match std::fs::remove_file(std::env::current_exe()?) {
        Err(e) => {
            println!("failed to remove the old binary: {}", e);
            return Err(e.into());
        }
        Ok(_) => {
            match std::fs::rename(
                format!("./{}", download_binary.get_binary_name()),
                env::current_exe()?,
            ) {
                Err(e) => {
                    println!("failed to rename the new binary: {}", e);
                    return Err(e.into());
                }
                Ok(_) => {
                    println!("Successfully updated to {}!", latest_version);
                }
            }
        }
    }

    std::fs::remove_file(TMP_DL_ARCHIVE)?;
    Ok(())
}
