use anyhow::{Context, Result};
use regex::Regex;
use subprocess::{Exec, Redirection};

pub fn run() -> Result<()> {
    let out = Exec::cmd("git")
        .args(&["log", "--oneline", "--pretty=\"%s\""])
        .stdout(Redirection::Pipe)
        .capture()
        .with_context(|| "fail to execute git command")?
        .stdout_str();
    for line in out.lines() {
        let cmt = CommitTitle::new(line);
        println!("orig: {}\n{:#?}", line, cmt);
    }

    Ok(())
}

// CommitTitle contains semantic version information from commit message
#[allow(dead_code)]
#[derive(Debug)]
struct CommitTitle {
    prefix: String,
    component: Option<String>,
    summary: String,
    is_breaking: bool,
}

impl CommitTitle {
    pub fn new(title: &str) -> Option<CommitTitle> {
        let re = Regex::new(r"(^[a-zA-Z]{3})([,|!]?)([a-zA-Z/]*): (.+)").unwrap();
        let cap = re.captures(title)?;

        // TODO: This should be improve
        Some(CommitTitle {
            prefix: cap.get(1)?.as_str().to_string(),
            is_breaking: cap.get(2)?.as_str() == "!",
            component: cap.get(3).map(|text| text.as_str().to_string()),
            summary: cap.get(4)?.as_str().to_string(),
        })
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_regex() {
        use super::CommitTitle;
        assert!(CommitTitle::new("rew,core: remove useless pretty arg").is_some());
        assert!(CommitTitle::new("rew,keymap/neogit: update neogit and fugitive keymap").is_some());
        assert!(CommitTitle::new("doc: update readme").is_some());
        assert!(CommitTitle::new("rew!plugins: remove famiu/nvim-reload").is_some());
    }
}
