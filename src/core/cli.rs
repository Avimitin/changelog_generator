use clap::Parser;

#[derive(Parser, Debug)]
#[clap(about, version, author)]
pub struct Args {
    range: String,

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
