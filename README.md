# jpcargo — Rust 日本語診断 Cargo ラッパー

[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange.svg)](https://www.rust-lang.org)
[![Error Codes](https://img.shields.io/badge/Supported%20Errors-518-green.svg)](https://doc.rust-lang.org/error_codes/error-index.html)

**`jpcargo`** は、Rust コンパイラ（`rustc`）および `cargo` の出力を解析し、英語のコンパイルエラーを**直感的でわかりやすい日本語解説・複数の具体例 Diff・修正方法**に変換してターミナルに表示する CLI ツールです。

公式エラーインデックスに登録されている **全518種類のエラーコード（`E0001` 〜 `E0806`）** および主要な Rust 標準 Lint / 警告を網羅しています。

---

## 📸 出力プレビュー

### 例1: 所有権ムーブエラー (`E0382`)

```text
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
エラー [E0382] 所有権が移動（ムーブ）した値を再び使用しています (所有権 (Ownership))
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

【発生箇所】
  --> src/main.rs:4:20

【コード】
     4 |     println!("{}", s);
       |                    ^ value borrowed here after move

【修正例】
  // 方法1: 所有権を渡す代わりに参照（借用）を渡す（推奨）
  - let s2 = s;
  + let s2 = &s;

  // 方法2: データを複製（クローン）して独立した値を持たせる
  - let s2 = s;
  + let s2 = s.clone();

  エラーコード詳細: jpcargo explain E0382

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

---

### 例2: 不変変数の再代入エラー (`E0384`)

```text
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
エラー [E0384] 不変（イミュータブル）変数に値を再代入しようとしています (可変性 (Mutability))
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

【発生箇所】
  --> src/main.rs:9:5

【コード】
     9 |     x = 20;
       |     ^^^^^^ cannot assign twice to immutable variable

【修正例】
  // 方法1: 変数宣言に `mut` を追加して可変変数にする（推奨）
  - let x = ...;
  + let mut x = ...;

  // 方法2: `let` を付けて新しい変数として再定義（シャドーイング）する
  - x = ...;
  + let x = ...;

  エラーコード詳細: jpcargo explain E0384

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

---

### 例3: 診断サマリー一覧テーブル（コンパイル終了時に自動生成）

```text
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 診断サマリー一覧 (エラー: 4 件, 警告: 3 件)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  1 | src/main.rs:13:20      | エラー | E0308              | 型が一致していません (Mismatched Types)
  2 | src/main.rs:4:20       | エラー | E0382              | 所有権が移動（ムーブ）した値を再び使用しています
  3 | src/main.rs:3:9        | 警告   | unused_variables   | 宣言された変数が一度も使用されていません
  4 | src/main.rs:9:5        | エラー | E0384              | 不変（イミュータブル）変数に値を再代入しようとしています
  5 | src/main.rs:8:9        | 警告   | unused_variables   | 宣言された変数が一度も使用されていません
  6 | src/main.rs:9:5        | 警告   | unused_assignments | 代入された値が一度も使われないまま上書き（再代入）されています
  7 | src/main.rs:19:5       | エラー | E0502              | 可変借用（&mut）と不変借用（&）が同時に存在しています
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

---

## 🚀 インストール方法

### 方法 1: `curl` ワンライナー（コンパイル不要・最速 1秒）

ターミナルで以下のコマンドを実行するだけで、**お使いのPCでコンパイルすることなく**、GitHub Actions で事前ビルドされたバイナリを直接ダウンロードして **わずか 1 秒** でインストールが完了します:

```bash
curl -fsSL https://raw.githubusercontent.com/sha256san/jpcargo/main/install.sh | bash
```

> **Note**: キャッシュをバイパスして最新版を確実に取得したい場合はこちら:
> ```bash
> curl -fsSL "https://raw.githubusercontent.com/sha256san/jpcargo/main/install.sh?v=$(date +%s)" | bash
> ```

---

### またはバイナリを直接 1行で配置（コンパイル不要・最速）

```bash
# Linux (x86_64 / Intel & AMD)
curl -fsSL https://github.com/sha256san/jpcargo/releases/latest/download/jpcargo-x86_64-unknown-linux-gnu.tar.gz | tar -xz -C ~/.cargo/bin

# Linux ARM64 (aarch64 / Raspberry Pi, AWS Graviton, ARM VPS)
curl -fsSL https://github.com/sha256san/jpcargo/releases/latest/download/jpcargo-aarch64-unknown-linux-gnu.tar.gz | tar -xz -C ~/.cargo/bin

# macOS Apple Silicon (aarch64 / M1, M2, M3, M4)
curl -fsSL https://github.com/sha256san/jpcargo/releases/latest/download/jpcargo-aarch64-apple-darwin.tar.gz | tar -xz -C ~/.cargo/bin

# macOS Intel (x86_64)
curl -fsSL https://github.com/sha256san/jpcargo/releases/latest/download/jpcargo-x86_64-apple-darwin.tar.gz | tar -xz -C ~/.cargo/bin
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

### 1. 基本コマンド（いつもの Cargo の代わりに実行）

```bash
# cargo run の代わりに
jpcargo run

# cargo check の代わりに（高速構文・型チェック）
jpcargo check

# cargo build の代わりに
jpcargo build

# cargo test の代わりに
jpcargo test

# cargo clippy の代わりに（詳細Lint診断）
jpcargo clippy

# コンパイラの修正提案を自動適用
jpcargo fix
```

---

### 2. 学習・診断・検索コマンド

```bash
# 1. jpcargo を最新バージョンに自動アップデート
jpcargo update

# 2. 開発環境の総合ヘルスチェック
jpcargo doctor

# 3. 指定したエラーコードの日本語詳細解説を表示
jpcargo explain E0596
jpcargo explain E0382
jpcargo explain E0277

# 4. キーワードでエラーや用語を日本語検索
jpcargo search borrow
jpcargo search 所有権
jpcargo search ライフタイム

# 5. 対応している全エラーコード一覧（518件）
jpcargo list

# 6. エラーカテゴリ別の統計情報を表示
jpcargo stats
```

---

## ⚙️ コマンドラインオプション

| オプション | 説明 |
|---|---|
| `--original` | 日本語診断に加えて、コンパイラの英語原文エラーも併記します |
| `-q, --quiet` | 簡潔な出力モード（概要と発生箇所のみ） |
| `-v, --verbose` | 検出エラー・警告件数などの詳細ログを表示 |

---

## 🩺 開発環境診断 (`jpcargo doctor`)

`jpcargo doctor` を実行すると、Rust/Cargo/リンカー/OS環境の整合性を一括診断します。

```bash
$ jpcargo doctor

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 jpcargo doctor — Rust & Cargo 開発環境診断
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  Checking Rust コンパイラ (rustc)             ... OK (rustc 1.97.1)
  Checking Cargo パッケージマネージャ               ... OK (cargo 1.97.1)
  Checking rustup ツールチェーンマネージャ            ... OK (rustup 1.29.0)
  Checking C リンカー (cc / gcc / clang)      ... OK (cc - 15.2.0)

【システム環境情報】
  - OS:            linux
  - アーキテクチャ: x86_64
  - ファミリ:      unix

【診断結果】
  [v] [正常] すべての必須コンポーネントが正常に利用可能です。

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

---

## 📊 対応エラーコード統計 (全518件)

```bash
$ jpcargo stats

=== jpcargo 診断統計 ===
対応エラー総数: 518 件

  - 型システム (Type)                     : 167 件
  - 型定義・宣言 (Decl)                   : 81 件
  - 構文・文法 (Syntax)                    : 45 件
  - 借用 (Borrow)                         : 44 件
  - マクロ・属性 (Attribute)               : 40 件
  - モジュール・可視性 (Module)             : 37 件
  - 所有権 (Ownership)                     : 34 件
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
2. **直訳ではなく仕組みと具体的な修正例を提供する**: 「なぜエラーになったのか」と「どう直せばよいか（Diff・コメント）」を明確に表示。
3. **コンテキスト情報を失わない (Preserve Error Code & Location)**: エラーコード、行番号、列番号、スニペットを完全に維持。
4. **未対応エラーでも停止しない (Graceful Fallback)**: どのような出力でもパニックを起こさず綺麗に表示。
5. **Cargo の透過的互換性 (Transparent Wrapper)**: 通常の Cargo コマンド・引数をそのまま透過実行。

---

## 📜 ライセンス

MIT OR Apache-2.0
