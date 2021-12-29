use anyhow::{Context, Result};
use regex::Regex;
use subprocess::{Exec, Redirection};

pub fn run() -> Result<()> {
    let out = Exec::cmd("git")
        .args(&["log", "--oneline", "--pretty='format:(%h) %s'"])
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
    hash: String,
    prefix: String,
    component: Option<String>,
    summary: String,
    is_breaking: bool,
}

impl CommitTitle {
    pub fn new(title: &str) -> Option<CommitTitle> {
        let rule = r"\(([a-zA-Z0-9]+)\) ([a-zA-Z]{3})([,|!]?)([a-zA-Z/]*): (.+)";
        let re = Regex::new(rule).unwrap();
        let cap = re.captures(title)?;

        // TODO: This should be improve
        Some(CommitTitle {
            hash: cap.get(1)?.as_str().to_string(),
            prefix: cap.get(2)?.as_str().to_string(),
            is_breaking: cap.get(3)?.as_str() == "!",
            component: cap.get(4).map(|text| text.as_str().to_string()),
            summary: cap.get(5)?.as_str().to_string(),
        })
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_regex() {
        use super::CommitTitle;
        let should_work = |expect: &str| -> bool {
            let ct = CommitTitle::new(expect);
            println!("{:?}", ct);
            ct.is_some()
        };
        assert!(should_work("(4b05c2e) new,core: implement commit title parser"));
        assert!(should_work("(e0fbc13) rew,core: remove useless pretty arg"));
        assert!(should_work("(8eee8e5) new: initiate changelog generator"));
        assert!(should_work("(adad53h) rew!plugins: remove famiu/nvim-reload"));
    }
}
