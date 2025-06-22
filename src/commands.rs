use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use dialoguer::{theme::ColorfulTheme, Select};
use dirs::home_dir;

/// 言語ごとのテンプレート定義
fn get_language_templates() -> Vec<(&'static str, Vec<(&'static str, &'static str)>)> {
    vec![
        (
            "rust",
            vec![
                ("review-performance", "Analyze the performance characteristics of this code and suggest improvements to make it faster or more efficient:"),
                ("generate-tests", "Generate unit tests for the following function using a common testing framework such as Rust’s built-in test framework:"),
                ("summarize-diff", "Summarize the following Git diff in natural language, describing what changed and why:"),
                ("readme-generator", "Based on this Rust project’s structure and content, generate a complete README.md file:"),
            ],
        ),
        (
            "typescript",
            vec![
                ("explain-code", "Explain the following TypeScript code in detail:"),
                ("add-jsdoc", "Add JSDoc comments to the following TypeScript code:"),
                ("generate-tests", "Generate unit tests for the following TypeScript function using Jest:"),
                ("commit-conventional", "Generate a Conventional Commit-style message for the following code diff:"),
            ],
        ),
    ]
}

/// コマンドを~/.claude/commandsに保存
fn save_command(name: &str, body: &str) -> io::Result<()> {
    let path = get_command_dir().join(format!("{name}.md"));
    let mut file = File::create(path)?;
    writeln!(file, "{}", body)?;
    Ok(())
}

/// ユーザーディレクトリのパス取得
fn get_command_dir() -> PathBuf {
    home_dir()
        .expect("Could not get home directory")
        .join(".claude/commands")
}

/// `claude-cli init` コマンド本体
pub fn init(lang: Option<String>, repo: Option<String>) {
    if let Some(repo_url) = repo {
        println!("🔗 GitHubテンプレートのダウンロードは未実装です: {repo_url}");
        // TODO: GitHub連携処理（git2またはreqwest+zip）
        return;
    }

    let templates = get_language_templates();

    let selected_lang = match lang {
        Some(lang) => lang,
        None => {
            let langs: Vec<&str> = templates.iter().map(|(l, _)| *l).collect();
            let selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("言語を選択してください")
                .items(&langs)
                .default(0)
                .interact()
                .unwrap();
            langs[selection].to_string()
        }
    };

    let template_set = templates
        .iter()
        .find(|(l, _)| *l == selected_lang)
        .map(|(_, t)| t.clone())
        .expect("テンプレートが見つかりません");

    let cmd_dir = get_command_dir();
    fs::create_dir_all(&cmd_dir).expect("コマンドディレクトリの作成に失敗しました");

    for (name, body) in template_set {
        match save_command(name, body) {
            Ok(_) => println!("✅ {name}.md を作成しました"),
            Err(e) => eprintln!("❌ {name}.md の作成に失敗しました: {}", e),
        }
    }

    println!("🎉 完了しました: ~/.claude/commands にコマンドが作成されました");
}

/// `claude-cli list` コマンド
pub fn list() {
    let dir = get_command_dir();
    if !dir.exists() {
        println!("⚠️ ユーザーコマンドはまだ存在しません");
        return;
    }

    println!("📋 現在のユーザーコマンド一覧:");
    for entry in fs::read_dir(&dir).unwrap() {
        if let Ok(file) = entry {
            if let Some(name) = file.path().file_name() {
                println!(" - {}", name.to_string_lossy());
            }
        }
    }
}

/// `claude-cli remove <name>` コマンド
pub fn remove(name: &str) {
    let path = get_command_dir().join(format!("{name}.md"));
    if path.exists() {
        fs::remove_file(&path).expect("ファイル削除に失敗しました");
        println!("🗑️ 削除しました: {name}.md");
    } else {
        println!("❓ 指定されたコマンドが見つかりません: {name}.md");
    }
}
