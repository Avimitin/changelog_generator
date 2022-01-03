use anyhow::{Context, Result};
use subprocess::{Exec, Redirection};
use clap::Parser;

mod cli;
mod commit;

use commit::{CommitTitle, CommitCollection};

pub fn run() -> Result<()> {
    let cli = cli::Args::parse();

    let out = Exec::cmd("git")
        .args(&["log", "--oneline", "--pretty=(%h) %s", cli.range()])
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

        if cmt.is_none() {
            uncategory.push(line);
            continue;
        }

        let cmt = cmt.unwrap();

        if cmt.is_breaking() {
            breaking_change_commit.push(cmt);
            continue;
        }

        match cmt.prefix() {
            "new" => feature_commit.push(cmt),
            "fix" => fix_commit.push(cmt),
            "rwt" => changes_commit.push(cmt),
            _ => uncategory.push(line),
        }
    }

    println!(
        "{}
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
        cli.range(),
        description = cli.description().unwrap_or(&String::from("")),
        breakchange = breaking_change_commit,
        feature = feature_commit,
        fix = fix_commit,
        changes = changes_commit,
    );

    Ok(())
}

