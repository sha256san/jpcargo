# MEMORY.md - jpcargo AI 知識ベース・設計上の決定事項

## 1. Cargo と rustc の JSON 診断インターフェース仕様

### 1.1 `--message-format=json` の出力構造
Cargo を `--message-format=json-diagnostic-rendered-ansi` または `--message-format=json` で起動すると、標準出力（stdout）に 1 行 1 JSON オブジェクト（NDJSON）形式で様々なメッセージが出力される。

主な `reason` フィールド:
- `"compiler-artifact"`: クレートのビルド成果物情報
- `"compiler-message"`: コンパイラ（rustc）からの診断メッセージ（`message` フィールドに Diagnostic オブジェクトが入る）
- `"build-script-executed"`: build.rs の実行結果
- `"build-finished"`: ビルド完了通知 (`"success": true/false`)

### 1.2 `Diagnostic` オブジェクトの重要な要素
- `message`: 英語のエラーメッセージ本文 (例: `"cannot assign to 'a', as it is not declared as mutable"`)
- `code`: `{ "code": "E0596", "explanation": "..." }` の形式。警告や一部の構文エラーでは `None` になる場合がある。
- `level`: `"error"`, `"warning"`, `"note"`, `"help"`, `"failure-note"` 等。
- `spans`: 配列形式。
  - `file_name`: ソースファイル名（相対パス）
  - `line_start`, `line_end`, `column_start`, `column_end`: 発生行・列
  - `is_primary`: この診断の主たる発生箇所（True のものを優先して表示）
  - `label`: スニペット上に表示される注記（例: `"cannot assign to immutable variable"`, `"cannot borrow as mutable"`)
  - `suggested_replacement`: rustc による修正候補テキスト
  - `text`: 対象行のソースコード断片
- `children`: サブ診断（`note: ...`, `help: ...`）のツリー構造。

### 1.3 ストリーム処理と透過性の両立
- `cargo run` 実行時、コンパイルエラーだけでなく、ビルドが成功した後のターゲット実行ファイルの標準入出力も適切にリレーする必要がある。
- コンパイル中の出力のみをパースし、実行時の stdout / stderr はユーザーのターミナルに直結させる。

---

## 2. 日本語診断エンジンの設計決定

### 2.1 エラーメッセージからコンテキスト（変数名・型名）の抽出
固定的な説明文だけではなく、エラーメッセージやスニペットの正規表現マッチングにより、以下の動的コンテキストを抽出して説明文に組み込む：
- 変数名: `` `([a-zA-Z0-9_]+)` `` や `` cannot assign to `([^`]+)` `` から抽出
- 期待される型 / 実際の型: `` expected `([^`]+)`, found `([^`]+)` `` (E0308)
- 所有権の移動元 / 移動先 (E0382)
- 借用箇所 (E0502)

### 2.2 専門用語の翻訳ポリシー
- Rust 開発者が公式ドキュメントや外部記事を読む際に乖離が生じないよう、英語のコアキーワードを完全消去しない。
- `ミュータブル（可変）`, `イミュータブル（不変）`, `所有権 (Ownership)`, `借用 (Borrow)`, `トレイト境界 (Trait Bound)` のように、初心者がイメージしやすい日本語と技術用語を併記する。

### 2.3 フォールバック処理の重要性
- 全てのエラーコードに対して手動ルールを即座に作成することは困難（数百種類存在）。
- 未登録のエラーコードに遭遇した場合は、rustc の `rendered` または `message` を綺麗にフォーマットして出力し、「日本語解説が未登録のエラーです」と親切に案内する。

---

## 3. レンダラーデザインシステム

### 3.1 視覚的ヒエラルキー
1. **ヘッダー**: エラー種別、エラーコード、日本語タイトル（太字 + 赤 / 黄）
2. **概要**: 1〜2 行で「何が起きたか」を簡潔に要約
3. **発生箇所**: ファイルパス、行番号、列番号（`--> src/main.rs:10:5`）
4. **コードスニペット**: 周辺行を含むソースコード表示（該当箇所の強調とポインタ `^`）
5. **原因解説**: なぜ Rust コンパイラがこれを拒絶したのか（言語仕様・安全性の理由）
6. **修正方法**: どう直せばよいか（具体的な diff 形式の提示）
7. **フッター / 原文**: `--original` 有効時の rustc 原文出力
