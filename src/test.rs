use std::env;
use super::flag;
use std::io::Result;

pub fn set_env() -> Result<()> {
    let p = env::current_dir()?.join("test.toml");
    env::set_var("CARGO_TOML_PATH", p.display().to_string());
    Ok(())
}

#[test]
pub fn test_prerelease() -> Result<()> {
    set_env()?;
    let pre_release = r#"[package]
name = "ver"
version = "0.0.1-0"
edition = "2021"
[dependencies]"#;
    let cmd = flag::_test("prerelease");
    let ver = cmd.parse_version()?;
    let version = cmd.prerelease(ver)?;
    let data = cmd.change_resource(version)?;
    assert_eq!(pre_release, data);
    Ok(())
}

#[test]
pub fn test_prepatch() -> Result<()> {
    set_env()?;
    let pre_release = r#"[package]
name = "ver"
version = "0.0.2-0"
edition = "2021"
[dependencies]"#;
    let cmd = flag::_test("prepatch");
    let ver = cmd.parse_version()?;
    let version = cmd.prepatch(ver)?;
    let data = cmd.change_resource(version)?;
    assert_eq!(pre_release, data);
    Ok(())
}

#[test]
pub fn test_preminor() -> Result<()> {
    set_env()?;
    let pre_release = r#"[package]
name = "ver"
version = "0.1.1-0"
edition = "2021"
[dependencies]"#;
    let cmd = flag::_test("preminor");
    let ver = cmd.parse_version()?;
    let version = cmd.preminor(ver)?;
    let data = cmd.change_resource(version)?;
    assert_eq!(pre_release, data);
    Ok(())
}

#[test]
pub fn test_premajor() -> Result<()> {
    set_env()?;
    let pre_release = r#"[package]
name = "ver"
version = "1.0.1-0"
edition = "2021"
[dependencies]"#;
    let cmd = flag::_test("premajor");
    let ver = cmd.parse_version()?;
    let version = cmd.premajor(ver)?;
    let data = cmd.change_resource(version)?;
    assert_eq!(pre_release, data);
    Ok(())
}

#[test]
pub fn test_patch() -> Result<()> {
    set_env()?;
    let pre_release = r#"[package]
name = "ver"
version = "0.0.2"
edition = "2021"
[dependencies]"#;
    let cmd = flag::_test("patch");
    let ver = cmd.parse_version()?;
    let version = cmd.patch(ver)?;
    let data = cmd.change_resource(version)?;
    assert_eq!(pre_release, data);
    Ok(())
}

#[test]
pub fn test_minor() -> Result<()> {
    set_env()?;
    let pre_release = r#"[package]
name = "ver"
version = "0.1.1"
edition = "2021"
[dependencies]"#;
    let cmd = flag::_test("minor");
    let ver = cmd.parse_version()?;
    let version = cmd.minor(ver)?;
    let data = cmd.change_resource(version)?;
    assert_eq!(pre_release, data);
    Ok(())
}

#[test]
pub fn test_major() -> Result<()> {
    set_env()?;
    let pre_release = r#"[package]
name = "ver"
version = "1.0.1"
edition = "2021"
[dependencies]"#;
    let cmd = flag::_test("major");
    let ver = cmd.parse_version()?;
    let version = cmd.major(ver)?;
    let data = cmd.change_resource(version)?;
    assert_eq!(pre_release, data);
    Ok(())
}