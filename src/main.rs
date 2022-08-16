mod flag;
#[cfg(test)]
mod test;


use std::io::{ErrorKind, Result};






fn main() -> Result<()> {
    let cli = flag::build();
    match cli.do_action() {
        Ok(version) => {
            println!("version updated to {}", version)
        }
        Err(e) => {
            match e.kind() {
                ErrorKind::NotFound => {
                    eprintln!("Error: The current directory does not include \"Cargo.toml\" file")
                }
                ErrorKind::Other => {
                    eprintln!("{}", e)
                }
                _ => {
                    eprintln!("{}", "unknown error")
                }
            }
        }
    }
    Ok(())
}
