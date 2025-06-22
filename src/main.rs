use clap::{Parser, Subcommand};

mod commands;
mod config;
mod project;
mod smart_templates;
mod templates;

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
    /// プロジェクトを自動検出してセットアップを行います。
    /// 
    /// サポートされている言語:
    /// - Rust (Normal)
    /// - Rust (WASM)
    /// - JavaScript
    /// - TypeScript  
    /// - Node.js
    Init {
        #[arg(short, long)]
        lang: Option<String>,
        #[arg(long)]
        repo: Option<String>,
        #[arg(short, long)]
        path: Option<String>,
    },
    /// プロジェクト情報を表示
    Detect {
        #[arg(short, long)]
        path: Option<String>,
    },
    /// 作成済みコマンドを一覧表示
    List,
    /// 指定したコマンドを削除
    Remove {
        name: String,
    },
    /// 設定ファイルを初期化
    Config,
    /// プロジェクト詳細分析と推奨コマンド表示
    Analyze {
        #[arg(short, long)]
        path: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Init { lang, repo, path }) => {
            commands::init(lang.clone(), repo.clone(), path.clone());
        }
        Some(Commands::Detect { path }) => {
            commands::detect(path.clone());
        }
        Some(Commands::List) => {
            commands::list();
        }
        Some(Commands::Remove { name }) => {
            commands::remove(name);
        }
        Some(Commands::Config) => {
            commands::config();
        }
        Some(Commands::Analyze { path }) => {
            commands::analyze(path.clone());
        }
        None => {
            println!("✨ Try: ccmgen init");
        }
    }
}
