use futures::prelude::*;
use hubcaps::issues::{IssueListOptions, State};
use hubcaps::{Credentials, Github};
use std::env;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();
    let token = env::var("GITHUB_TOKEN")?;
    let github = Github::new(
        concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")),
        Credentials::Token(token),
    )?;
    github
        .repo("matthiasbeyer", "imag")
        .issues()
        .iter(
            &IssueListOptions::builder()
                .per_page(100)
                .state(State::All)
                .build(),
        )
        .try_for_each(move |issue| async move {
            println!("{} ({})", issue.title, issue.state);
            Ok(())
        })
        .await?;
    Ok(())
}
