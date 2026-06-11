use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};
use copy_for_ai_core::{convert_file, estimate_tokens, format_document, OutputFormat, OutputStyle};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(name = "copy-for-ai")]
#[command(about = "Convert files into AI-ready Markdown or text")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Convert {
        /// File to convert
        path: PathBuf,

        /// Output format
        #[arg(long, value_enum, default_value_t = CliOutputFormat::Markdown)]
        format: CliOutputFormat,

        /// Output style
        #[arg(long, value_enum, default_value_t = CliOutputStyle::Clean)]
        style: CliOutputStyle,

        /// Print only the estimated token count
        #[arg(long)]
        tokens_only: bool,
    },
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum CliOutputFormat {
    Markdown,
    Text,
    Prompt,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum CliOutputStyle {
    Clean,
    Compact,
    Detailed,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Convert {
            path,
            format,
            style,
            tokens_only,
        } => {
            let document = convert_file(path)?;

            if tokens_only {
                println!("{}", estimate_tokens(&document.content));
                return Ok(());
            }

            let output = format_document(&document, format.into(), style.into());
            println!("{output}");
        }
    }

    Ok(())
}

impl From<CliOutputFormat> for OutputFormat {
    fn from(value: CliOutputFormat) -> Self {
        match value {
            CliOutputFormat::Markdown => OutputFormat::Markdown,
            CliOutputFormat::Text => OutputFormat::PlainText,
            CliOutputFormat::Prompt => OutputFormat::PromptWrappedMarkdown,
        }
    }
}

impl From<CliOutputStyle> for OutputStyle {
    fn from(value: CliOutputStyle) -> Self {
        match value {
            CliOutputStyle::Clean => OutputStyle::Clean,
            CliOutputStyle::Compact => OutputStyle::Compact,
            CliOutputStyle::Detailed => OutputStyle::Detailed,
        }
    }
}
