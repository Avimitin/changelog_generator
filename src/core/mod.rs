use std::fmt;

use anyhow::{Context, Result};
use regex::Regex;
use subprocess::{Exec, Redirection};

struct CommitCollection {
    store: Vec<CommitTitle>,
}

impl CommitCollection {
    fn new() -> CommitCollection {
        CommitCollection { store: Vec::new() }
    }

    fn push(&mut self, commit: CommitTitle) {
        self.store.push(commit);
    }
}

impl fmt::Display for CommitCollection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = String::new();
        for commit in &self.store {
            if let Some(component) = &commit.component {
                output.push_str(format!("* {}: {}\n", component, commit.summary).as_str());
            } else {
                output.push_str(format!("* {}\n", commit.summary).as_str());
            }
        }
        write!(f, "{}", output)
    }
}

pub fn run() -> Result<()> {
    let out = Exec::cmd("git")
        .args(&["log", "--oneline", "--pretty=(%h) %s"])
        .stdout(Redirection::Pipe)
        .capture()
        .with_context(|| "fail to execute git command")?
        .stdout_str();
    let mut breaking_change_commit = CommitCollection::new();
    let mut feature_commit = CommitCollection::new();
    let mut fix_commit = CommitCollection::new();
    let mut changes_commit = CommitCollection::new();
    let mut uncategory = Vec::new();
    for line in out.lines() {
        let cmt = CommitTitle::new(line);
        if let Some(cmt) = cmt {
            if cmt.is_breaking {
                breaking_change_commit.push(cmt);
            } else {
                match cmt.prefix.as_str() {
                    "new" => feature_commit.push(cmt),
                    "fix" => fix_commit.push(cmt),
                    "rwt" => changes_commit.push(cmt),
                    _ => uncategory.push(line),
                }
            }
        } else {
            uncategory.push(line);
        }
    }

    println!(
        "Version {}
==========
{description}


Breaking Changes
----------------
{breakchange}

Features
--------
{feature}

Fix
---
{fix}

Changes
--------
{changes}
",
        0.1,
        description = "Fuck You",
        breakchange = breaking_change_commit,
        feature = feature_commit,
        fix = fix_commit,
        changes = changes_commit,
    );

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
            component: cap.get(4).and_then(|text| {
                if text.as_str().is_empty() {
                    None
                } else {
                    Some(text.as_str().to_string())
                }
            }),
            summary: cap.get(5)?.as_str().to_string(),
        })
    }
}

impl fmt::Display for CommitTitle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Commit Information:
Hash: {}
Type: {}
Component: {}
Summary: {}
Is Breaking Change: {}",
            self.hash,
            self.prefix,
            self.component
                .as_ref()
                .unwrap_or(&String::from("No Component")),
            self.summary,
            self.is_breaking
        )
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
        assert!(should_work(
            "(4b05c2e) new,core: implement commit title parser"
        ));
        assert!(should_work("(e0fbc13) rew,core: remove useless pretty arg"));
        assert!(should_work("(8eee8e5) new: initiate changelog generator"));
        assert!(should_work(
            "(adad53h) rew!plugins: remove famiu/nvim-reload"
        ));
    }
}
