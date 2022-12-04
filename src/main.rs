mod cli;
mod req;
mod utils;

use crate::req::RequestExt;
use anyhow::Result;
use clap::Parser;
use cli::Cli;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();

    println!("Got args:\n\t{:?}\n--------", args);

    match &args.action {
        cli::Action::Run(run_args) => {
            let request = RequestExt::try_from(run_args)?;
            let resp = request.send().await?;
            println!("response: {:#?}", resp);
        }
    }

    Ok(())
}
