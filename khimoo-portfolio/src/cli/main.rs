use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::cli::commands::{ProcessArticlesCommand, ProcessArticlesArgs, ValidateLinksCommand, ValidateLinksArgs};

/// CLI for khimoo-portfolio tools
#[derive(Parser)]
#[command(name = "khimoo-portfolio")]
#[command(about = "Portfolio management tools")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Process articles and generate static data
    ProcessArticles(ProcessArticlesArgs),
    /// Validate links in markdown articles
    ValidateLinks(ValidateLinksArgs),
}

impl Cli {
    pub fn execute(self) -> Result<()> {
        match self.command {
            Commands::ProcessArticles(args) => {
                let command = ProcessArticlesCommand::new()?;
                command.execute(args)
            }
            Commands::ValidateLinks(args) => {
                let command = ValidateLinksCommand::new()?;
                command.execute(args)
            }
        }
    }
}