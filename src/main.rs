use std::env;
use std::fs::{File, OpenOptions};
use std::io::{Error, ErrorKind, Read, Result, Write};
use std::path::PathBuf;
use clap::{Parser, Subcommand};
use regex::{Captures, Regex};

#[derive(Parser)]
pub struct Cli {
    #[clap(subcommand)]
    command: Ver,
}

#[derive(Subcommand)]
pub enum Ver {
    /// 升级预发布号
    Prerelease,
    /// 升级修订号，保留预发布号
    Prepatch,
    /// 升级次版本号，保留预发布号
    Preminor,
    /// 升级主版本号，保留预发布号
    Premajor,
    /// 升级修订号
    Patch,
    /// 升级次版本号
    Minor,
    /// 升级主版本号
    Major,
}

impl Cli {
    pub fn do_action(self) -> Result<()> {
        let ver = self.parse_version()?;
        let version = match self.command {
            Ver::Prerelease => self.prerelease(ver)?,
            Ver::Prepatch => self.prepatch(ver)?,
            Ver::Preminor => self.preminor(ver)?,
            Ver::Premajor => self.premajor(ver)?,
            Ver::Patch => self.patch(ver)?,
            Ver::Minor => self.minor(ver)?,
            Ver::Major => self.major(ver)?,
        };
        // change the version
        let data = self.change_resource(version)?;
        self.save_resource(data)?;
        Ok(())
    }
    // change cargo.toml content
    fn change_resource(&self, version: String) -> Result<String> {
        let (_start, _end, content) = self.parse_package_content()?;
        let content_re = Regex::new(r#"version\s*=\s*"(\d).(\d).(\d)-?(\d)?""#).expect("init regex failed!");
        let new_content = content_re.replace(&content, format!("version = \"{}\"", version)).to_string();
        // change package section content
        let resource = self.get_resource()?;
        let resource_re = Regex::new(r#"\[package]\n([^\[]+)\["#).expect("init regex failed!");
        let new_resource = resource_re.replace(&resource, format!("[package]\n{}[", new_content)).to_string();
        Ok(new_resource)
    }
    /// 0.0.1 -> 0.0.1-0 or 0.0.1-0 -> 0.0.1-1
    fn prerelease(&self, ver: Vec<Option<String>>) -> Result<String> {
        let vec = version_plus(3, ver, true);
        Ok(format!("{}-{}", vec[0..3].join("."), vec[3]))
    }
    /// 0.0.1 -> 0.0.2-0 or 0.0.1-0 -> 0.0.2-0
    fn prepatch(&self, ver: Vec<Option<String>>) -> Result<String> {
        let vec = version_plus(2, ver, true);
        Ok(format!("{}-{}", vec[0..3].join("."), vec[3]))
    }
    /// 0.0.1 -> 0.1.1-0 or 0.1.1-0 -> 0.2.1-0
    fn preminor(&self, ver: Vec<Option<String>>) -> Result<String> {
        let vec = version_plus(1, ver, true);
        Ok(format!("{}-{}", vec[0..3].join("."), vec[3]))
    }
    /// 0.0.1 -> 1.0.1-0 or 0.0.1-0 -> 1.0.1-0
    fn premajor(&self, ver: Vec<Option<String>>) -> Result<String> {
        let vec = version_plus(0, ver, true);
        Ok(format!("{}-{}", vec[0..3].join("."), vec[3]))
    }
    /// 0.0.1 -> 0.0.2 or 0.0.1-0 -> 0.0.2
    fn patch(&self, ver: Vec<Option<String>>) -> Result<String> {
        let vec = version_plus(2, ver, false);
        Ok(vec.join("."))
    }
    /// 0.0.1 -> 0.1.1 or 0.0.1-0 -> 0.1.1
    fn minor(&self, ver: Vec<Option<String>>) -> Result<String> {
        let vec = version_plus(1, ver, false);
        Ok(vec.join("."))
    }
    /// 0.0.1 -> 1.0.1 or 0.0.1-0 -> 1.0.1
    fn major(&self, ver: Vec<Option<String>>) -> Result<String> {
        let vec = version_plus(0, ver, false);
        Ok(vec.join("."))
    }
    fn get_resource(&self) -> Result<String> {
        let path = match env::var("CARGO_TOML_PATH") {
            Ok(str) => {
                PathBuf::from(str)
            }
            Err(_) => {
                env::current_dir()?.join("Cargo.toml")
            }
        };
        let mut fd = File::open(path)?;
        let mut str = String::new();
        fd.read_to_string(&mut str)?;
        Ok(str)
    }
    fn save_resource(&self, data: String) -> Result<()> {
        let path = match env::var("CARGO_TOML_PATH") {
            Ok(str) => {
                PathBuf::from(str)
            }
            Err(_) => {
                env::current_dir()?.join("Cargo.toml")
            }
        };
        let mut fd = OpenOptions::new().write(true).truncate(true).open(path)?;
        fd.write_all(data.as_bytes())
    }
    fn parse_package_content(&self) -> Result<(usize, usize, String)> {
        let resource = self.get_resource()?;
        let re = Regex::new(r#"\[package]\n([^\[]+)\["#).expect("init regex failed!");
        let (start, end, data) = if let Some(caps) = re.captures(&resource) {
            let cap = caps.get(1).expect("could not get the package match");
            (cap.start(), cap.end(), cap.as_str().to_string())
        } else { return Err(Error::new(ErrorKind::Other, "could not get the package match")); };
        Ok((start, end, data))
    }
    fn parse_version(&self) -> Result<Vec<Option<String>>> {
        let (_, _, data) = self.parse_package_content()?;
        let re = Regex::new(r#"version\s*=\s*"(\d).(\d).(\d)-?(\d)?""#).expect("init regex failed!");
        let mut vec = vec![];
        if let Some(caps) = re.captures(&data) {
            let clos = |capt: &Captures, i: usize| match capt.get(i) {
                None => {
                    None
                }
                Some(cap) => {
                    Some(cap.as_str().to_string())
                }
            };
            vec.push(clos(&caps, 1));
            vec.push(clos(&caps, 2));
            vec.push(clos(&caps, 3));
            vec.push(clos(&caps, 4));
        } else {
            return Err(Error::new(ErrorKind::Other, "could not get the version match"));
        }
        Ok(vec)
    }
}

fn version_plus(index: usize, ver: Vec<Option<String>>, is_pre: bool) -> Vec<String> {
    assert_eq!(ver.len(), 4);
    let mut vec = vec![];
    for (k, v) in ver.iter().enumerate() {
        match v {
            None => {
                if is_pre {
                    vec.push("0".to_string())
                }
            }
            Some(str) => {
                if !is_pre && k == 3 {
                    continue;
                }
                if k == index {
                    let patch = str.parse::<usize>().expect("version parse failed");
                    vec.push(format!("{}", patch + 1))
                } else {
                    vec.push(str.clone())
                }
            }
        }
    }
    if is_pre {
        assert_eq!(vec.len(), 4);
    } else {
        assert_eq!(vec.len(), 3);
    }
    vec
}

fn main() -> Result<()> {
    let cli: Cli = Cli::parse();
    cli.do_action()?;
    Ok(())
}

#[cfg(test)]
mod test {
    use std::env;
    use super::{Cli, Ver};
    use std::io::Result;

    fn set_env() -> Result<()> {
        let p = env::current_dir()?.join("test.toml");
        env::set_var("CARGO_TOML_PATH", p.display().to_string());
        Ok(())
    }

    #[test]
    fn test_prerelease() -> Result<()> {
        set_env()?;
        let pre_release = r#"[package]
name = "ver"
version = "0.0.1-0"
edition = "2021"
[dependencies]"#;
        let cmd = Cli {
            command: Ver::Prerelease
        };
        let ver = cmd.parse_version()?;
        let version = cmd.prerelease(ver)?;
        let data = cmd.change_resource(version)?;
        assert_eq!(pre_release, data);
        Ok(())
    }

    #[test]
    fn test_prepatch() -> Result<()> {
        set_env()?;
        let pre_release = r#"[package]
name = "ver"
version = "0.0.2-0"
edition = "2021"
[dependencies]"#;
        let cmd = Cli {
            command: Ver::Prerelease
        };
        let ver = cmd.parse_version()?;
        let version = cmd.prepatch(ver)?;
        let data = cmd.change_resource(version)?;
        assert_eq!(pre_release, data);
        Ok(())
    }

    #[test]
    fn test_preminor() -> Result<()> {
        set_env()?;
        let pre_release = r#"[package]
name = "ver"
version = "0.1.1-0"
edition = "2021"
[dependencies]"#;
        let cmd = Cli {
            command: Ver::Prerelease
        };
        let ver = cmd.parse_version()?;
        let version = cmd.preminor(ver)?;
        let data = cmd.change_resource(version)?;
        assert_eq!(pre_release, data);
        Ok(())
    }

    #[test]
    fn test_premajor() -> Result<()> {
        set_env()?;
        let pre_release = r#"[package]
name = "ver"
version = "1.0.1-0"
edition = "2021"
[dependencies]"#;
        let cmd = Cli {
            command: Ver::Prerelease
        };
        let ver = cmd.parse_version()?;
        let version = cmd.premajor(ver)?;
        let data = cmd.change_resource(version)?;
        assert_eq!(pre_release, data);
        Ok(())
    }

    #[test]
    fn test_patch() -> Result<()> {
        set_env()?;
        let pre_release = r#"[package]
name = "ver"
version = "0.0.2"
edition = "2021"
[dependencies]"#;
        let cmd = Cli {
            command: Ver::Prerelease
        };
        let ver = cmd.parse_version()?;
        let version = cmd.patch(ver)?;
        let data = cmd.change_resource(version)?;
        assert_eq!(pre_release, data);
        Ok(())
    }

    #[test]
    fn test_minor() -> Result<()> {
        set_env()?;
        let pre_release = r#"[package]
name = "ver"
version = "0.1.1"
edition = "2021"
[dependencies]"#;
        let cmd = Cli {
            command: Ver::Prerelease
        };
        let ver = cmd.parse_version()?;
        let version = cmd.minor(ver)?;
        let data = cmd.change_resource(version)?;
        assert_eq!(pre_release, data);
        Ok(())
    }

    #[test]
    fn test_major() -> Result<()> {
        set_env()?;
        let pre_release = r#"[package]
name = "ver"
version = "1.0.1"
edition = "2021"
[dependencies]"#;
        let cmd = Cli {
            command: Ver::Prerelease
        };
        let ver = cmd.parse_version()?;
        let version = cmd.major(ver)?;
        let data = cmd.change_resource(version)?;
        assert_eq!(pre_release, data);
        Ok(())
    }
}
