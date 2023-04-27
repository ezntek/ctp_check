use octocrab::{self, models::Repository, Error};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = "None")]
pub struct CliArgs {
    pub search_query: String,
}

pub async fn get_repo<S>(repo: S) -> Result<Repository, Error>
where
    S: AsRef<str>
     + Into<String>
{
    octocrab::instance()
        .repos("catppuccin", repo)
        .get()
        .await
}