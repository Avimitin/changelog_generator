use clap::Parser;

#[derive(Parser, Debug)]
#[clap(about, version, author)]
pub struct Args {
    /// Same as the git revision range, pass to the git log command
    range: String,

    /// Add description under the header
    #[clap(short, long)]
    description: Option<String>,
}

impl Args {
    pub fn range(&self) -> &str {
        &self.range
    }

    pub fn description(&self) -> Option<&String> {
        self.description.as_ref()
    }
}
