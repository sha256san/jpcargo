# CHANGELOG.md

すべての重要な変更は本ファイルに記録されます。
フォーマットは [Keep a Changelog](https://keepachangelog.com/ja/1.0.0/) に基づいています。

## [0.3.1] - 2026-08-15

### Changed
- 診断出力レイアウトの改善: `【修正方法】` セクションを削除し、`【修正例】` セクション内に修正Diffと一言解説をスマートに統合。

---

## [0.3.0] - 2026-08-15

### Added
- 全518種類のエラーコード（E0001 〜 E0806）および主要 Rust 標準 Lint / 警告（`unused_assignments`, `unused_variables`, `dead_code` 等）の完全網羅日本語診断。
- 診断結果末尾の番号付きサマリー一覧テーブル表示機能。
- `jpcargo update` によるワンコマンド自動自己アップデート。
- Apple Silicon (macOS aarch64) および Linux ARM64 (aarch64) への完全対応と GitHub Actions 自動リリース。
- 1 秒高速インストーラー (`install.sh`) の提供。
- クリーンかつ直感的な文字ベースUI（`[v]`, `[x]`, `[!]`）。

---

## [0.2.0] - 2026-08-15

### Changed
- インストーラー（`install.sh`）、自己アップデート（`jpcargo update`）、環境診断（`jpcargo doctor`）、診断レンダラー全体のテキストメッセージを極力シンプルに整理。
- すべての絵文字を削除し、チェックと×も文字表記（`[v]`, `[x]`, `[!]`）に統一。

---

## [0.1.9] - 2026-08-15

### Added
- **Apple Silicon (macOS aarch64 - M1/M2/M3/M4) & Linux ARM (aarch64) ビルドの最適化**:
  - GitHub Actions (`.github/workflows/release.yml`) にて、`gcc-aarch64-linux-gnu` クロスリンカーを用いた高速 Linux ARM64 ビルドおよび `macos-latest` ネイティブ Apple Silicon ビルドを完備。
  - Raspberry Pi、AWS Graviton、ARM Linux VPS、M1〜M4 Mac での 1 秒インストール・アップデートに完全対応。

---

## [0.1.8] - 2026-08-14

### Removed
- `--level` オプション（初心者向け・専門家向け解説モード切り替え機能）を削除し、常に最も簡潔で直感的な診断表示に統一。
- インストーラーおよびヘルプメッセージから `--level beginner` の案内を削除。

---

## [0.1.7] - 2026-08-14

### Added
- **診断サマリー一覧テーブル機能**:
  - `jpcargo run`, `check`, `build` 等のコンパイル終了時に、検出されたすべてのエラー・警告を整理した番号付きサマリー表を末尾に自動生成する機能を追加。
  - 番号・発生箇所・区分（エラー/警告）・コード・タイトルが一目で把握可能に。

---

## [0.1.6] - 2026-08-14

### Added
- エラー発生時に詳細解説をすぐに参照できるよう、`ℹ️ エラーコード詳細: jpcargo explain <CODE>` の案内を表示。

### Changed
- 英語の重複したコンパイラヒントを完全に排除し、エラーコードと修正方法に集中したスッキリした出力に改善。

---

## [0.1.5] - 2026-08-14

### Removed
- ターミナル出力から `【コンパイラからのヒント】` セクションを削除し、よりダイレクトな修正ガイダンスのみに整理。

---

## [0.1.4] - 2026-08-14

### Added
- **`jpcargo update` コマンド**:
  - ワンコマンドで GitHub Releases から最新バイナリを自動取得し、自己アップデートを完了する機能を実装。
- **インストーラーの強制上書き対応**:
  - `install.sh` において既存の `jpcargo` バイナリが存在していても確実に強制上書き（`rm -f` / `cp -f`）してインストールするよう強化。

---

## [0.1.3] - 2026-08-14

### Changed
- 診断出力から `【概要】` および `【原因とRustの仕組み】` を削除し、**発生箇所・コードスニペット・修正方法・修正例（Diff）** に特化した洗練されたレイアウトに変更。

---

## [0.1.2] - 2026-08-14

### Added
- **主要 Rust 標準 Lint / 警告診断ルールの追加**:
  - `unused_assignments` (代入値の未使用・上書き)
  - `unused_variables` (未使用変数)
  - `unused_mut` (不要な mut 修飾子)
  - `dead_code` (未使用コード・関数・構造体)
  - `unused_imports` (未使用の use インポート)
  - `unused_must_use` (Result 等の戻り値破棄)
  - `non_snake_case` (スネークケース命名違反)
  - `non_camel_case_types` (キャメルケース型名命名違反)
  - `unreachable_code` (到達不能コード)
  - `unreachable_patterns` (到達不能 match パターン)
- Clippy や未知の Lint に対する高品質なフォールバック日本語ガイダンス機能。

---

## [0.1.1] - 2026-08-14

### Added
- **GitHub Actions マルチプラットフォーム 自動リリース CI/CD (`.github/workflows/release.yml`)**:
  - Linux (x86_64, aarch64)
  - macOS (Intel x86_64, Apple Silicon aarch64)
  - Windows (x86_64)
- **1 秒高速インストーラー (`install.sh`)**:
  - 手元での `cargo build` を行わず、GitHub Releases の事前ビルド済みバイナリを直接取得して 1 秒で配置するインストーラーを提供。

---

## [0.1.0] - 2026-08-14

### Added
- **全518種類のエラーコード（`E0001` 〜 `E0806`）の完全網羅実装**:
  - 全518件の個別ルールファイル (`src/japanese/rules/e0001.rs` 〜 `e0806.rs`) および `mod.rs` の作成
  - 静的メタデータデータベース (`src/japanese/database.rs`)
  - カテゴリ分類器 (`src/diagnostic/classifier.rs`)
  - 10個の全網羅テストスイート (`tests/rules_test.rs`)
- **CLI コマンド体系**:
  - `jpcargo run`, `build`, `check`, `test`, `clippy`, `doc`, `fix`
  - `jpcargo explain <CODE>` (指定エラーの日本語解説)
  - `jpcargo list` (全518件のエラー一覧)
  - `jpcargo stats` (カテゴリ別統計情報)
  - `jpcargo search <KEYWORD>` (エラー・用語の日本語検索)
  - `jpcargo doctor` (Rust/Cargo/リンカー/OS 総合環境診断)
  - `--level <beginner|normal|expert>` (難易度切り替え)
- 用語対訳辞書 (`dictionary/rust_terms.json`) の整備。
