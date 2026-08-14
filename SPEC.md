# SPEC.md - jpcargo 仕様書

## 1. プロジェクト概要

### 1.1 プロジェクト名
**jpcargo** (Rust 日本語診断 Cargo ラッパー)

### 1.2 目的
Rust コンパイラ（rustc）が出力する診断情報（エラー、警告、ヒント）を日本語化し、エラーの原因、関連する Rust のコア概念、および具体的な修正方法を分かりやすく提示する CLI ツール。

### 1.3 設計哲学と基本原則
1. **rustc の診断を最優先する**: コンパイラの正確な型検査・借用検査結果を正とし、勝手な解釈を行わない。
2. **直訳ではなく、理解を促す解説**: 専門用語を無理に不自然な日本語にせず、元の用語（英語/カタカナ）と意味を併記する。
3. **完全なコンテキスト保持**: エラーコード（E0xxx）、ファイルパス、行・列番号、スニペットを失わない。
4. **原文確認の容易性**: `--original` オプションでいつでも rustc の原文出力を参照可能にする。
5. **フォールバック保証**: 未対応のエラーコードが発生してもツールが停止せず、原文を綺麗にフォールバック表示する。
6. **Cargo との透過的互換性**: Cargo 本来のビルドや実行コマンドの引数・挙動を損なわずに透過的に委譲する。

---

## 2. コマンド体系と CLI 仕様

### 2.1 Cargo 互換コマンド
以下の Cargo コマンドを透過的にラップし、ビルドエラー発生時に日本語診断を出力する。

| コマンド | 動作 |
|---|---|
| `jpcargo run [args...]` | `cargo run` を実行し、コンパイルエラーがあれば日本語診断を表示 |
| `jpcargo build [args...]` | `cargo build` を実行し、エラー/警告を日本語診断として表示 |
| `jpcargo check [args...]` | `cargo check` を実行し、高速に日本語診断を表示 |
| `jpcargo test [args...]` | `cargo test` を実行し、コンパイルエラーやテスト失敗を解析 |
| `jpcargo clippy [args...]` | `cargo clippy` を実行し、Clippy の Lint 警告を日本語で解説 |
| `jpcargo doc [args...]` | `cargo doc` を実行 |

### 2.2 共通オプション
- `--original`: 日本語診断に加えて、コンパイラの原文エラーメッセージも併記する。
- `--verbose`: 詳細な内部情報や診断コンテキストを表示する。
- `--quiet`: 必要最低限の診断メッセージのみを表示する。
- `--level <beginner|normal|expert>`: 説明の難易度・詳細度を切り替える（beginner: 初心者向け平易な解説, normal: 通常, expert: 専門家向け内部制約解説）。

### 2.3 独自コマンド
| コマンド | 引数 | 動作 |
|---|---|---|
| `jpcargo doctor` | なし | Rust, Cargo, rustup, リンカー(cc/gcc/clang), OS環境の総合日本語診断 |
| `jpcargo fix` | `[args...]` | rustc の自動修正候補（MachineApplicable）を適用 |
| `jpcargo explain <CODE>` | `E0382`, `E0596` 等 | 指定された Rust エラーコードの日本語詳細解説・サンプルコード・解決策を表示 |
| `jpcargo search <KEYWORD>` | `borrow`, `mut`, `lifetime` 等 | エラーコードや用語をキーワード検索 |
| `jpcargo list` | なし | jpcargo が対応しているエラーコードの一覧を表示（全56種） |
| `jpcargo stats` | なし | カテゴリ別の対応エラーコード数や統計情報を表示 |

---

## 3. アーキテクチャとデータフロー

### 3.1 実行フロー
```text
jpcargo run / check / build
    │
    ▼
CLI 引数解析 (cli.rs)
    │
    ▼
Cargo プロセス起動 (cargo.rs)
  - 引数に `--message-format=json-diagnostic-rendered-ansi` などを付与
    │
    ▼
標準出力 / 標準エラー出力をストリーム読み取り
    │
    ├─► 通常のプログラム出力 / ビルド進捗 ──► ターミナルへ透過出力
    │
    └─► JSON 診断メッセージ ──► パーサー (diagnostic/parser.rs)
                                   │
                                   ▼
                            Diagnostic 内部モデル
                                   │
                                   ▼
                            エラー分類器 (classifier.rs)
                                   │
                                   ▼
                            日本語翻訳エンジン (japanese/)
                               - ルール DB (rules/e*.rs)
                               - 専門用語辞書 (dictionary/)
                               - ソースコード抽出 (location.rs)
                               - 修正案生成 (suggestion/)
                                   │
                                   ▼
                            ターミナルレンダラー (renderer/)
                               - 装飾ボックス
                               - ハイライトスニペット
                               - 原因 / 修正方法 / 修正後コード
                                   │
                                   ▼
                            ユーザーへ出力
```

---

## 4. 診断情報の内部データ構造

### 4.1 Diagnostic (rustc JSON 互換)
```rust
pub struct Diagnostic {
    pub message: String,
    pub code: Option<DiagnosticCode>,
    pub level: DiagnosticLevel, // error, warning, note, help, etc.
    pub spans: Vec<DiagnosticSpan>,
    pub children: Vec<Diagnostic>,
    pub rendered: Option<String>,
}

pub struct DiagnosticCode {
    pub code: String,       // "E0596"
    pub explanation: Option<String>,
}

pub struct DiagnosticSpan {
    pub file_name: String,
    pub line_start: usize,
    pub line_end: usize,
    pub column_start: usize,
    pub column_end: usize,
    pub is_primary: bool,
    pub text: Vec<SpanText>,
    pub label: Option<String>,
    pub suggested_replacement: Option<String>,
    pub suggestion_applicability: Option<String>,
}
```

---

## 5. 出力フォーマット仕様

### 5.1 診断出力レイアウト
```text
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
エラー [E0596] 変数がミュータブル（可変）として宣言されていません
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

概要:
  変数「a」がミュータブルとして宣言されていないため、値を再代入・変更できません。

発生箇所:
  --> src/main.rs:3:5

コード:
  1 | fn main() {
  2 |     let a = 1;
> 3 |     a = 2;
    |     ^ ここで変更しようとしています
  4 | }

原因:
  Rust では、`let` で宣言された変数はデフォルトでイミュータブル（不変）です。
  一度初期化された後は、明示的に `mut` を付けない限り値を変更できません。

修正方法:
  変数宣言に `mut` キーワードを追加して、ミュータブル（可変）に変更してください。

修正例:
  - let a = 1;
  + let mut a = 1;

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

---

## 6. エラー分類と主要対応エラー

### 6.1 カテゴリ分類
- **Mutability (可変性)**: E0596 等
- **Ownership (所有権)**: E0382 等
- **Borrow (借用)**: E0502, E0506, E0597, E0499, E0505 等
- **Type (型システム)**: E0308, E0282 等
- **Trait (トレイト)**: E0277 等
- **Syntax / Name Resolution (名前解決・構文)**: E0425, E0433 等
- **Lifetime (ライフタイム)**: E0106, E0495 等
- **Other (その他)**

### 6.2 専門用語対訳表
| 英語 | 日本語表記（初心者に配慮した併記） |
|---|---|
| mutable / mutability | ミュータブル（可変 / 変更可能） |
| immutable | イミュータブル（不変 / 変更不可） |
| ownership | 所有権 |
| borrow / borrowing | 借用（参照の貸し出し） |
| lifetime | ライフタイム（生存期間） |
| trait | トレイト（共通の振る舞い・インターフェース） |
| closure | クロージャ（無名関数） |
| move | ムーブ（所有権の移動） |
| binding | バインディング（変数束縛） |
| type mismatch | 型の不一致 |
