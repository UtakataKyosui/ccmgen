use clap::{Parser, Subcommand};

mod commands;

#[derive(Parser)]
#[command(about = "Claude Code User Command Initializer", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// 言語毎のセットアップ
    /// 
    /// 言語を指定してセットアップを行います。
    /// 
    /// サポートされている言語:
    /// - Rust
    /// - JavaScript
    Init {
        #[arg(short, long)]
        lang: Option<String>,
        #[arg(long)]
        repo: Option<String>,
    },
    List,
    Remove {
        name: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Init { lang, repo }) => {
            commands::init(lang.clone(), repo.clone());
        }
        Some(Commands::List) => {
            commands::list();
        }
        Some(Commands::Remove { name }) => {
            commands::remove(name);
        }
        None => {
            println!("✨ Try: claude-cli init");
        }
    }
}
