use clap::Parser;
use ctp_check;
use colored::Colorize;

#[tokio::main]
async fn main() {
    let args = ctp_check::CliArgs::parse();

    let repo = match ctp_check::get_repo(&args.search_query.to_lowercase()).await {
        Ok(repo) => repo,
        Err(_) => {
            println!("{} The repository {} probably doesn't exist!", "Error: ".bold().red(), &args.search_query.bold());
            std::process::exit(1);
        },
    };

    println!("{} Found repo {}!", "Info: ".yellow().bold(), &repo.name.bold());
    match &repo.description {
        Some(desc) => println!("{} \"{}\"", "Info: ".bold().yellow(), desc),
        None => {},
    }

    println!("{} You can clone it from {}.", "Info: ".bold().yellow(), &repo.html_url.unwrap().as_str().bold());
}
