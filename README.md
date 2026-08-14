# 🦀 jpcargo — Rust 日本語診断 Cargo ラッパー

[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange.svg)](https://www.rust-lang.org)
[![Error Codes](https://img.shields.io/badge/Supported%20Errors-518-green.svg)](https://doc.rust-lang.org/error_codes/error-index.html)

**`jpcargo`** は、Rust コンパイラ（`rustc`）および `cargo` の出力を解析し、英語のコンパイルエラーを**親切でわかりやすい日本語の解説・原因の仕組み・具体的な修正案**に変換してターミナルに表示する CLI ツールです。

公式エラーインデックスに登録されている **全518種類のエラーコード（`E0001` 〜 `E0806`）** を網羅しています。

---

## 📸 出力プレビュー

### 例1: 不変変数の変更エラー (`E0596`)

```text
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
エラー [E0596] 不変な借用値を変更しようとしています (可変性 (Mutability))
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

【発生場所】
  --> src/main.rs:3:5

【問題のコード】
  2 |     let a = 1;
  3 |     a = 2;
    |     ^^^^^ 不変な変数として借用されています

【概要】
  不変（immutable）として宣言された変数や借用先の内容を変更しようとしました。

【原因とRustの仕組み】
  Rust では変数はデフォルトで不変（immutable）です。
  値を変更するには、変数宣言時に明示的に `mut` キーワードを付与する必要があります。

【修正方法】
  変数の宣言箇所に `mut` を追加してください:
    修正前: let a = 1;
    修正後: let mut a = 1;

【コンパイラからのヒント】
  • help: consider changing this to be mutable: `mut a`
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

---

### 例2: 初心者向け解説モード (`--level beginner`)

```bash
jpcargo --level beginner check
```

```text
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
エラー [E0499] 同時に複数の可変借用（&mut）が存在しています (借用 (Borrow))
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

💡【初心者向け解説】
  「同時に書き込める人は常に1人だけ」というRustのルールです。前の `&mut` の作業が終わるまで待ちましょう。

【発生場所】
  --> src/main.rs:5:14

【概要】
  同一の変数に対して、重複して可変参照（`&mut`）を取得しようとした場合に発生します。

【原因とRustの仕組み】
  データ競合（Data Race）を防ぐため、可変参照はスコープ内で唯一（排他的）である必要があります。

【修正方法】
  参照の利用順序を分けるか、スコープを `{ ... }` ブロックで分離してください。
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

---

### 例3: 環境エラー（Cリンカーが見つからない場合）

```text
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
環境エラー [LINKER] Cコンパイラ/リンカーが見つかりません
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

【原因】
  Rustコード自体の問題ではなく、OS環境にリンカー（Cコンパイラ: cc / gcc / clang）がインストールされていません。

【解決方法】
  以下のコマンドを実行してビルド必須ツールをインストールしてください:
    Debian/Ubuntu: sudo apt update && sudo apt install build-essential
    macOS:         xcode-select --install
    Fedora/RHEL:   sudo dnf groupinstall 'Development Tools'
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

---

## 🚀 インストール方法

### 方法 1: `curl` ワンライナー（コンパイル不要・最速 1秒 ⚡）

ターミナルで以下のコマンドを実行するだけで、**お使いのPCでコンパイルすることなく**、GitHub Actions で事前ビルドされたバイナリを直接ダウンロードして **わずか 1 秒** でインストールが完了します:

```bash
curl -fsSL https://raw.githubusercontent.com/sha256san/jpcargo/main/install.sh | bash
```

> **Note**: キャッシュをバイパスして最新版を確実に取得したい場合はこちら:
> ```bash
> curl -fsSL "https://raw.githubusercontent.com/sha256san/jpcargo/main/install.sh?v=$(date +%s)" | bash
> ```

---

### またはバイナリを直接 1行で配置（最速）

```bash
# Linux (x86_64)
curl -fsSL https://github.com/sha256san/jpcargo/releases/latest/download/jpcargo-x86_64-unknown-linux-gnu.tar.gz | tar -xz -C ~/.cargo/bin

# macOS (Apple Silicon / M1, M2, M3)
curl -fsSL https://github.com/sha256san/jpcargo/releases/latest/download/jpcargo-aarch64-apple-darwin.tar.gz | tar -xz -C ~/.cargo/bin
```

---

### 方法 2: `cargo install` でソースからビルドしてインストール

```bash
cargo install --git https://github.com/sha256san/jpcargo.git
```

---

### 方法 3: リポジトリから手動ビルド・インストール

```bash
git clone https://github.com/sha256san/jpcargo.git
cd jpcargo
./install.sh
```

> **Note**: `~/.cargo/bin` に PATH が通っていることを確認してください。
> ```bash
> export PATH="$HOME/.cargo/bin:$PATH"
> ```

---

## 🛠️ 使い方

通常の `cargo` コマンドの代わりに `jpcargo` を使用するだけです。

### 1. 日常の開発コマンド

```bash
# cargo run の代わりに
jpcargo run

# cargo check の代わりに
jpcargo check

# cargo build の代わりに
jpcargo build

# cargo test の代わりに
jpcargo test

# cargo clippy の代わりに（日本語警告）
jpcargo clippy

# rustc の安全な修正案を自動適用
jpcargo fix
```

---

### 2. 学習・診断・検索コマンド

```bash
# 1. 開発環境の総合ヘルスチェック
jpcargo doctor

# 2. 指定したエラーコードの日本語詳細解説を表示
jpcargo explain E0596
jpcargo explain E0382
jpcargo explain E0277

# 3. キーワードでエラーや用語を日本語検索
jpcargo search borrow
jpcargo search 所有権
jpcargo search ライフタイム

# 4. 対応している全エラーコード一覧（518件）
jpcargo list

# 5. エラーカテゴリ別の統計情報を表示
jpcargo stats
```

---

## ⚙️ コマンドラインオプション

| オプション | 説明 |
|---|---|
| `--level <beginner\|normal\|expert>` | 解説の難易度・詳細度を切り替えます（デフォルト: `normal`） |
| `--original` | 日本語診断に加えて、コンパイラの英語原文エラーも併記します |
| `-q, --quiet` | 簡潔な出力モード（概要と発生箇所のみ） |
| `-v, --verbose` | 検出エラー・警告件数などの詳細ログを表示 |

---

## 🩺 開発環境診断 (`jpcargo doctor`)

`jpcargo doctor` を実行すると、Rust/Cargo/リンカー/OS環境の整合性を一括診断します。

```bash
$ jpcargo doctor

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 🩺 jpcargo doctor — Rust & Cargo 開発環境総合診断
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  Checking Rust コンパイラ (rustc)             ... OK (rustc 1.97.1)
  Checking Cargo パッケージマネージャ               ... OK (cargo 1.97.1)
  Checking rustup ツールチェーンマネージャ            ... OK (rustup 1.29.0)
  Checking C リンカー (cc / gcc / clang)      ... OK (cc - 15.2.0)

【システム環境情報】
  - OS:            linux
  - アーキテクチャ: x86_64
  - ファミリ:      unix

【診断結果サマリー】
  ✅ [正常] すべての必須コンポーネントが正常に利用可能です。快適に Rust 開発を行えます！

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

---

## 📊 対応エラーコード統計 (全518件)

```bash
$ jpcargo stats

=== jpcargo 診断統計 ===
対応エラー総数: 518 件

【カテゴリ別内訳】
  - トレイト (Trait)                     : 138 件
  - ライフタイム (Lifetime)                : 57 件
  - 型システム (Type)                     : 53 件
  - 所有権 (Ownership)                  : 51 件
  - 関数 (Function)                    : 41 件
  - 構造体 (Struct)                     : 30 件
  - 借用 (Borrow)                      : 26 件
  - パターンマッチ (Pattern Match)          : 26 件
  - モジュール・インポート (Module/Import)      : 24 件
  - ジェネリクス (Generic)                 : 15 件
  - 演算子 (Operator)                   : 13 件
  - その他 (Other)                      : 12 件
  - 可変性 (Mutability)                 : 7 件
  - 定数評価 (Const)                     : 6 件
  - 機能フラグ (Feature)                  : 5 件
  - 可視性 (Visibility)                 : 3 件
  - マクロ (Macro)                      : 3 件
  - アンセーフ (Unsafe)                   : 2 件
  - 名前解決 (Name Resolution)           : 2 件
  - クロージャ (Closure)                  : 2 件
  - メモリ・ライフサイクル (Memory/Drop)        : 1 件
  - 非同期処理 (Async)                    : 1 件
```

---

## 📖 開発方針 & 原則

1. **rustc の診断を最優先する (rustc is the Source of Truth)**: コンパイラの型・借用・ライフタイム診断を100%尊重。
2. **直訳ではなく仕組みを解説する (Explain, Don't Just Translate)**: 初心者でも「なぜエラーになったのか」が分かるように解説。
3. **コンテキスト情報を失わない (Preserve Error Code & Location)**: エラーコード、行番号、列番号、スニペットを完全に維持。
4. **未対応エラーでも停止しない (Graceful Fallback)**: どのような出力でもパニックを起こさず綺麗に表示。
5. **Cargo の透過的互換性 (Transparent Wrapper)**: 通常の Cargo コマンド・引数をそのまま透過実行。

---

## 📜 ライセンス

MIT OR Apache-2.0
