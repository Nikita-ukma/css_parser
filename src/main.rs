
use anyhow::Result;
use clap::CommandFactory;
use clap::{Parser, Subcommand};
use std::fs;
use my_rust_css_parser::parse_css;

#[derive(Parser)]
#[command(author = "Тєлєгін Нікіта", version = "1.0", about = "CSS Parser CLI")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Parse {
        #[arg(short, long)]
        file: String,
    },
    Credits,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Parse { file }) => {
            let css_content = fs::read_to_string(file)?;
            parse_css(&css_content)?;
        }
        Some(Commands::Credits) => {
            println!("CSS Parser Project\nАвтор: Тєлєгін Нікіта");
        }
        None => {
            println!("Команда не була вказана.\n");
            Cli::command().print_help()?;
            println!();
        }
    }
    Ok(())
}
