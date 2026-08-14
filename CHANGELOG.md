# CHANGELOG.md

すべての重要な変更は本ファイルに記録されます。
フォーマットは [Keep a Changelog](https://keepachangelog.com/ja/1.0.0/) に基づいています。

## [0.3.0] - 2026-08-14

### Added
- `err1-407.md`, `err408-666.md`, `err667-806.md` に記載された **全518種類のエラーコード（E0001 〜 E0806）を完全網羅実装**:
  - 全518件のエラーコードデータベース (`src/japanese/database.rs`) の自動統合
  - 動的変数抽出・差分提示を行う56件の高精度カスタムルールと、462件の包括的公式診断ルールのシームレスな統合
  - `jpcargo list` による全518件の一覧表示
  - `jpcargo stats` による全518件のカテゴリ別集計
  - `jpcargo explain <CODE>` で全518個のエラーコードすべてに対する詳細解説・サンプルコード表示に対応
  - コンパイルエラー発生時の全518件完全フォールバック＆日本語診断対応
  - **Sランク**: `E0308`, `E0382`, `E0499`, `E0502`, `E0505`, `E0507`, `E0596`, `E0597`, `E0277`, `E0599`
  - **Aランク**: `E0425`, `E0432`, `E0433`, `E0061`, `E0004`, `E0072`, `E0133`, `E0282`, `E0283`, `E0384`, `E0603`
  - **Bランク & 主要エラー**: `E0594`, `E0506`, `E0106`, `E0621`, `E0271`, `E0119`, `E0117`, `E0207`, `E0210`, `E0062`, `E0069`, `E0063`, `E0609`, `E0027`, `E0026`, `E0428`, `E0252`, `E0255`, `E0616`, `E0606`, `E0614`, `E0608`, `E0369`, `E0368`, `E0600`, `E0521`, `E0593`, `E0728`, `E0015`, `E0080`, `E0161`, `E0040`, `E0391`, `E0659`, `E0658`
- `jpcargo doctor` コマンド: Rust, Cargo, rustup, リンカー(cc/gcc/clang), システム環境の総合診断
- `jpcargo fix` コマンド: rustc の安全な提案の自動適用
- `--level <beginner|normal|expert>` フラグ: 初心者向け／通常／専門家向けの解説難易度切り替え機能
- リンカーエラー（`linker \`cc\` not found` 等）および Cargo 依存関係解決エラーの自動検知と日本語ガイダンス
- 用語対訳辞書 (`dictionary/rust_terms.json`) の大幅拡充（Drop, Sized, Orphan Rule, Unsafe, Closure 等）
- 全機能・全エラーコードのテストスイート構築 (`tests/rules_test.rs`)

## [0.1.0] - 2026-08-14

### Added
- プロジェクト仕様書 (`SPEC.md`)、未実装タスク管理 (`TODO.md`)、知識ベース (`MEMORY.md`)、開発規約 (`AGENTS.md`) の策定。
- Rust 日本語診断 Cargo ラッパーの基本 CLI アーキテクチャ構築:
  - `jpcargo run`, `build`, `check`, `test`, `clippy`, `doc`
  - `jpcargo explain <CODE>`, `list`, `stats`, `search <KEYWORD>`
- rustc JSON 診断パーサーおよびデータモデルの実装 (`src/diagnostic/`)
- ターミナル向けリッチデザインレンダラー (`src/renderer/`)
