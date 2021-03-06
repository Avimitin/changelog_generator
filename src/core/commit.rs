use std::fmt;
use regex::Regex;

/// CommitTitle contains semantic version information from commit message
#[allow(dead_code)]
#[derive(Debug)]
pub struct CommitTitle<'a> {
    hash: &'a str,
    prefix: &'a str,
    component: Option<&'a str>,
    summary: &'a str,
    is_breaking: bool,
}

impl<'a> CommitTitle<'a> {
    /// Parse a commit to `CommitTitle`
    ///
    /// # Example
    ///
    /// ```rust
    /// let commit = CommitTitle::new("(ae87b3c) new,plugin: modify the plugin sequences")
    ///
    /// assert_eq!(commit, Some(
    ///   CommitTitle{
    ///     hash: String::from("ae87b3c"),
    ///     prefix: String::from("new"),
    ///     component: Option(String::from("plugin")),
    ///     summary: "modify the plugin sequences",
    ///     is_breaking: false,
    ///   })
    /// )
    /// ```
    pub fn new(title: &str) -> Option<CommitTitle> {
        let rule = r"\(([a-zA-Z0-9]+)\) ([a-zA-Z]{3})([,|!]?)([a-zA-Z/]*): (.+)";
        let re = Regex::new(rule).unwrap();
        let cap = re.captures(title)?;

        Some(CommitTitle {
            hash: cap.get(1)?.as_str(),
            prefix: cap.get(2)?.as_str(),
            is_breaking: cap.get(3)?.as_str() == "!",
            component: cap.get(4).and_then(|text| {
                if text.as_str().is_empty() {
                    None
                } else {
                    Some(text.as_str())
                }
            }),
            summary: cap.get(5)?.as_str(),
        })
    }

    /// Return true if the commit contains `!` character
    pub fn is_breaking(&self) -> bool {
        self.is_breaking
    }

    /// Return the commit type
    pub fn prefix(&self) -> &str {
        &self.prefix
    }
}

impl<'a> fmt::Display for CommitTitle<'a> {
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
                .unwrap_or("No Component"),
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

pub struct CommitCollection<'a> {
    store: Vec<CommitTitle<'a>>,
}

impl<'a> CommitCollection<'a> {
    pub fn new() -> CommitCollection<'a> {
        CommitCollection { store: Vec::new() }
    }

    pub fn push(&mut self, commit: CommitTitle<'a>) {
        self.store.push(commit);
    }
}

impl<'a> fmt::Display for CommitCollection<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = String::new();

        if self.store.is_empty() {
            output.push_str("Null");
            return write!(f, "{}", output);
        }

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
