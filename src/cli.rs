use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    name = "jpcargo",
    author = "jpcargo team",
    version = env!("CARGO_PKG_VERSION"),
    about = "Rust日本語診断Cargoラッパー - コンパイルエラーを分かりやすい日本語で診断・解説します",
    long_about = "jpcargo は、Cargo および rustc の出力を解析し、Rust コンパイラのエラーを親切な日本語診断・解説・修正案に変換して表示する CLI ツールです。"
)]
pub struct Cli {
    /// 原文（英語のコンパイラ出力）も併記して表示
    #[arg(long, global = true)]
    pub original: bool,

    /// 簡潔な出力モード（概要と発生箇所のみ）
    #[arg(short, long, global = true)]
    pub quiet: bool,

    /// 詳細なログを出力
    #[arg(short, long, global = true)]
    pub verbose: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// cargo run を日本語診断付きで実行
    Run {
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },

    /// cargo build を日本語診断付きで実行
    Build {
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },

    /// cargo check を日本語診断付きで実行
    Check {
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },

    /// cargo test を日本語診断付きで実行
    Test {
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },

    /// cargo clippy を日本語診断付きで実行
    Clippy {
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },

    /// cargo doc を実行
    Doc {
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },

    /// rustc の自動修正候補（MachineApplicable）を適用
    Fix {
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },

    /// Rust/Cargo/Toolchain/Linker (cc/gcc/clang)/OS 環境の総合日本語診断
    Doctor,

    /// jpcargo を最新バージョンに自動アップデート
    Update,

    /// 指定された Rust エラーコード（例: E0596, E0308, E0382）を日本語で詳しく解説
    Explain {
        /// エラーコード (例: E0596)
        code: String,
    },

    /// エラーコードや用語をキーワードで日本語検索 (例: borrow, mut, 型)
    Search {
        /// 検索キーワード
        query: String,
    },

    /// jpcargo が対応しているエラーコードの一覧を表示
    List,

    /// 対応エラーコードのカテゴリ別統計情報を表示
    Stats,

    /// 単体 .rs ファイルを rustc で直接コンパイルして日本語診断
    Rustc {
        /// 対象の .rs ファイルパス
        file: String,

        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },
}
