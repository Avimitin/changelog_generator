use subprocess::{Exec, Redirection};
use anyhow::{Context, Result};

pub fn run() -> Result<()> {
    let out = Exec::cmd("git").args(&["log", "--oneline", "--pretty=\"format:%s (%h)\""])
        .stdout(Redirection::Pipe)
        .capture()
        .with_context(|| { "fail to execute git command" })?
        .stdout_str();
    println!("{}", out);

    Ok(())
}
