# jpcargo

## Rust日本語診断Cargoラッパー 開発計画書

---

# 1. プロジェクト概要

## 1.1 プロジェクト名

**jpcargo**

Rustの標準的なCargoコマンドを日本語診断付きで実行するCLIツール。

基本的な使い方は、

```bash
cargo run
```

の代わりに、

```bash
jpcargo run
```

とする。

---

# 2. プロジェクトの目的

Rustコンパイラは非常に詳細なエラー情報を出力する。

しかし、エラーメッセージの多くは英語であり、Rust初心者にとっては、

* エラーの意味
* Rustのどの仕組みが原因なのか
* どこを修正すればよいのか
* なぜその修正が必要なのか

を理解することが難しい。

そこで`jpcargo`では、Cargoとrustcの間に日本語診断レイヤーを追加する。

```text
通常

ユーザー
  ↓
cargo run
  ↓
rustc
  ↓
英語のエラー
```

これを、

```text
jpcargo

ユーザー
  ↓
jpcargo run
  ↓
cargo run
  ↓
rustc
  ↓
診断情報
  ↓
jpcargo解析エンジン
  ↓
日本語エラー
```

とする。

---

# 3. なぜjpcargoなのか

候補として、

```bash
jprust run
```

も考えられる。

しかし、本プロジェクトでは、

```bash
jpcargo run
```

を採用する。

## 理由

Rustのプロジェクト管理は基本的にCargoを中心に行われるため。

例えば、

```bash
cargo build
cargo run
cargo check
cargo test
cargo clippy
cargo doc
```

など、多くの開発作業がCargoを入口としている。

そのため、

```bash
jpcargo
```

を「Cargoに日本語診断機能を追加したもの」として設計すると、ユーザーにとって理解しやすい。

---

# 4. jpcargoの基本コンセプト

`jpcargo`はCargoを置き換えるものではない。

```text
jpcargo
   ↓
cargo
   ↓
rustc
```

という関係にする。

つまり、

```bash
jpcargo run
```

は内部的に、

```bash
cargo run
```

を実行する。

ただし、コンパイルエラーを取得して日本語診断に変換する。

---

# 5. 基本コマンド

## 5.1 jpcargo run

```bash
jpcargo run
```

内部：

```text
jpcargo
  ↓
cargo run
  ↓
コンパイル
  ↓
エラー？
  ↓
日本語化
```

---

# 6. jpcargo build

```bash
jpcargo build
```

通常の、

```bash
cargo build
```

に対応する。

コンパイルエラーが発生した場合、

```text
ERROR[E0308]

型が一致していません。

期待されている型:
i32

実際に指定された型:
String

発生箇所:
src/main.rs:10:5
```

のように表示する。

---

# 7. jpcargo check

```bash
jpcargo check
```

通常の、

```bash
cargo check
```

に対応する。

Rust開発では非常に頻繁に使用されるため、優先的に対応する。

---

# 8. jpcargo test

```bash
jpcargo test
```

通常の、

```bash
cargo test
```

に対応する。

コンパイルエラーだけでなく、将来的にはテスト失敗についても日本語で説明する。

例えば、

```text
test failed

テスト「test_add」が失敗しました。

期待値:
10

実際の値:
9

発生箇所:
src/lib.rs:42
```

など。

---

# 9. jpcargo clippy

```bash
jpcargo clippy
```

通常の、

```bash
cargo clippy
```

に対応する。

Clippyの警告も日本語化対象にする。

例えば、

```text
warning: this expression creates a needless borrow
```

を、

```text
警告:

この式では不要な借用が発生しています。

不要な`&`を削除できる可能性があります。
```

などとする。

---

# 10. jpcargo doc

```bash
jpcargo doc
```

通常の、

```bash
cargo doc
```

に対応する。

基本的にはCargoへそのまま渡す。

---

# 11. 引数の扱い

`jpcargo`独自の引数とCargoの引数を区別する。

基本：

```bash
jpcargo <cargo-command> [arguments]
```

例えば、

```bash
jpcargo run
```

```bash
jpcargo build
```

```bash
jpcargo check
```

```bash
jpcargo test
```

など。

Cargoに引数が必要な場合も可能な限りそのまま渡す。

---

# 12. プロジェクト構造

```text
jpcargo/
├── Cargo.toml
├── README.md
├── LICENSE
│
├── src/
│   ├── main.rs
│   │
│   ├── cli.rs
│   ├── cargo.rs
│   ├── rustc.rs
│   │
│   ├── diagnostic/
│   │   ├── mod.rs
│   │   ├── parser.rs
│   │   ├── classifier.rs
│   │   └── location.rs
│   │
│   ├── japanese/
│   │   ├── mod.rs
│   │   ├── translator.rs
│   │   ├── explanation.rs
│   │   └── template.rs
│   │
│   ├── suggestion/
│   │   ├── mod.rs
│   │   └── fixer.rs
│   │
│   └── renderer/
│       ├── mod.rs
│       └── terminal.rs
│
├── diagnostics/
│   ├── ownership/
│   ├── borrow/
│   ├── lifetime/
│   ├── type/
│   ├── mutability/
│   ├── trait/
│   ├── syntax/
│   ├── macro/
│   ├── async/
│   └── other/
│
├── dictionary/
│   └── rust_terms.json
│
└── tests/
    ├── E0308/
    ├── E0382/
    ├── E0502/
    ├── E0596/
    └── ...
```

---

# 13. 処理フロー

```text
                    jpcargo run
                         │
                         ▼
                 CLI引数を解析
                         │
                         ▼
                  cargo runを実行
                         │
                         ▼
                  Cargoの出力取得
                         │
                         ▼
                 rustc診断情報取得
                         │
                         ▼
                  JSON診断を解析
                         │
             ┌───────────┴───────────┐
             │                       │
          成功                    エラー
             │                       │
             ▼                       ▼
       通常の結果表示         エラー分類
                                     │
                                     ▼
                              エラーコード解析
                                     │
                                     ▼
                              ソースコード解析
                                     │
                                     ▼
                              日本語説明生成
                                     │
                                     ▼
                              修正方法生成
                                     │
                                     ▼
                              ターミナル表示
```

---

# 14. rustcの診断情報を取得する

通常のエラー文字列を直接解析するのではなく、可能な限りJSON形式の診断情報を利用する。

概念的には、

```bash
rustc --error-format=json
```

などの形式を利用する。

これにより、

```text
エラーメッセージ
エラーコード
ファイル
行
列
ラベル
help
note
suggestion
```

などを構造化された状態で取得する。

---

# 15. Cargoからrustcへの診断情報

Cargoは内部でrustcを呼び出す。

したがって、

```text
jpcargo
   ↓
Cargo
   ↓
rustc
```

の診断情報を取得できる構造を作る。

重要なのは、Cargo自体の動作を変更しないこと。

---

# 16. 診断情報の内部構造

例えば、

```rust
struct Diagnostic {
    level: Level,
    code: Option<String>,
    message: String,
    spans: Vec<Span>,
    children: Vec<ChildDiagnostic>,
}
```

を基本構造とする。

---

# 17. Span情報

Rustの診断には、エラーが発生した位置を示す情報が含まれる。

取得する情報：

```text
ファイル名
開始行
終了行
開始列
終了列
ラベル
```

例えば、

```text
src/main.rs:3:5
```

を取得する。

---

# 18. 日本語化エンジン

日本語化は、

```text
エラーコード
+
メッセージ
+
span
+
help
+
note
+
ソースコード
```

を利用して行う。

---

# 19. エラーコード辞書

例えば、

```text
E0596
```

を、

```text
カテゴリ:
Mutability

日本語:
変更できない変数を変更しようとしています。
```

へ変換する。

---

# 20. E0596の具体例

コード：

```rust
fn main() {
    let a = 1;
    a = 2;
}
```

通常：

```text
error[E0596]
cannot assign to `a`, as it is not declared as mutable
```

jpcargo：

```text
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
エラー [E0596]
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

変数「a」がミュータブルとして宣言されていないため、
値を変更できません。

発生箇所:
src/main.rs:3:5

原因:

Rustでは、変数はデフォルトでは
イミュータブル（変更不可）です。

現在:

let a = 1;

この変数に対して、

a = 2;

と値を変更しようとしています。

修正方法:

変数を変更可能にするため、
`mut`を追加してください。

修正後:

let mut a = 1;

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

---

# 21. 変数名を自動取得する

エラー情報から、

```text
a
```

を取得できる場合、

```text
変数「a」が...
```

と表示する。

固定文：

```text
変数が変更できません。
```

ではなく、

```text
変数「a」が変更できません。
```

とする。

---

# 22. 行番号を表示する

必ず可能な限り、

```text
src/main.rs:3:5
```

を表示する。

さらにコードを表示する。

```text
  1 | fn main() {
  2 |     let a = 1;
> 3 |     a = 2;
    |     ^ 変更しようとしています
  4 | }
```

---

# 23. Rustの専門用語

日本語化では専門用語を無理に完全な日本語に置き換えない。

例えば、

| Rust用語    | 表示           |
| --------- | ------------ |
| mutable   | ミュータブル（可変）   |
| immutable | イミュータブル（不変）  |
| ownership | 所有権          |
| borrow    | 借用           |
| lifetime  | ライフタイム       |
| trait     | トレイト         |
| closure   | クロージャ        |
| move      | ムーブ / 所有権の移動 |
| binding   | バインディング      |

初心者には日本語の意味を補足し、Rust開発者には元の用語も分かるようにする。

---

# 24. エラー分類

エラーを以下のカテゴリに分類する。

```text
Syntax
Type
Ownership
Borrow
Lifetime
Mutability
Trait
Generic
Pattern
Function
Struct
Enum
Module
Visibility
Macro
Async
Concurrency
Dependency
Linker
Build
Clippy
Test
Other
```

---

# 25. 主要エラーから実装する

最初から全エラーを実装しない。

優先順位：

```text
第1優先
E0308
E0382
E0502
E0506
E0596
E0597

第2優先
E0425
E0433
E0277
E0282
E0499
E0505

第3優先
その他の主要エラー

第4優先
特殊ケース
```

---

# 26. 未対応エラー

対応していないエラーが発生した場合でも、`jpcargo`は停止しない。

```text
━━━━━━━━━━━━━━━━━━━━
未対応のエラー
━━━━━━━━━━━━━━━━━━━━

エラーコード:
E1234

原文:
...

発生箇所:
src/main.rs:10:5

現在、このエラーの詳細な日本語解説は
登録されていません。

Rustコンパイラの原文を表示します。
```

とする。

---

# 27. 原文表示

日本語化しても、原文を確認できるようにする。

```bash
jpcargo run --original
```

などを提供する。

出力：

```text
日本語:

変数「a」が変更できません。

原文:

cannot assign to `a`, as it is not declared as mutable
```

---

# 28. 出力モード

以下のモードを実装する。

```bash
jpcargo run
```

通常の日本語診断。

```bash
jpcargo run --original
```

原文付き。

```bash
jpcargo run --verbose
```

詳細情報。

```bash
jpcargo run --quiet
```

必要最低限の診断。

---

# 29. 日本語レベル

将来的に、

```bash
jpcargo run --level beginner
```

を提供する。

初心者向け：

```text
Rustでは変数は基本的に変更できません。
変更したい場合はmutを付けます。
```

通常：

```text
変数「a」はimmutable bindingであるため、
assignmentできません。
```

専門家向け：

```text
immutable bindingへのassignmentが検出されました。
```

---

# 30. 修正方法

エラーだけではなく修正方法を提示する。

例えば、

```text
修正前:

let a = 1;

修正後:

let mut a = 1;
```

とする。

---

# 31. rustcのsuggestionを優先する

Rustコンパイラが、

```text
help:
consider making the binding mutable
```

などのsuggestionを提供している場合は、それを利用する。

優先順位：

```text
rustc suggestion
       ↓
jpcargo既知の修正方法
       ↓
一般的な修正方法
```

---

# 32. 自動修正

将来的に、

```bash
jpcargo fix
```

を実装する。

または、

```bash
jpcargo run --fix
```

とする。

ただし、自動修正は安全性を考慮する。

---

# 33. 自動修正レベル

```text
SAFE
```

安全性が高い修正。

```text
REVIEW
```

ユーザーによる確認が必要。

```text
NO-FIX
```

自動修正しない。

例えば、

```text
mutの追加
```

は比較的安全だが、

```text
clone()の追加
```

は所有権設計や性能に影響するため慎重に扱う。

---

# 34. Cargoコマンドの透過性

`jpcargo`はCargoの代替ではなくラッパーである。

したがって、

```bash
jpcargo build --release
```

なら内部的には、

```bash
cargo build --release
```

を実行する。

同様に、

```bash
jpcargo run --release
```

は、

```bash
cargo run --release
```

に相当する。

---

# 35. Cargo引数とjpcargo引数

例えば、

```bash
jpcargo run --release
```

では、

```text
jpcargo
 ↓
run
 ↓
--release
```

を解析する。

必要に応じて、

```text
jpcargo独自オプション
```

と、

```text
Cargoオプション
```

を分離する。

---

# 36. 将来的な特殊コマンド

Cargo互換だけでなく、日本語診断専用コマンドも追加する。

```bash
jpcargo explain E0382
```

エラーコードを日本語で解説する。

例えば、

```text
jpcargo explain E0382
```

↓

```text
E0382

名前:
Use of moved value

日本語:
所有権が移動した値を再び使用しています。

Rustでは...
```

---

# 37. jpcargo doctor

環境診断機能。

```bash
jpcargo doctor
```

以下を確認する。

```text
Rust
Cargo
rustc
rustup
toolchain
target
linker
OS
architecture
```

これは将来的な機能とする。

---

# 38. jpcargo explain

最重要の独自機能の一つ。

```bash
jpcargo explain E0596
```

でエラーコードを日本語解説する。

---

# 39. jpcargo search

エラーコードだけでなく、

```bash
jpcargo search borrow
```

のようにRustエラーを検索できるようにする。

将来的には、

```text
jpcargo search "変数を変更できない"
```

のような自然言語検索も検討する。

---

# 40. jpcargo list

対応しているエラーを一覧表示する。

```bash
jpcargo list
```

例：

```text
対応済みエラー:

E0308  型が一致しない
E0382  所有権が移動した値を使用
E0502  可変借用と不変借用が競合
E0596  不変変数を変更
...
```

---

# 41. jpcargo stats

診断統計を表示する。

```bash
jpcargo stats
```

例：

```text
対応エラー数:
128

Ownership:
18

Borrow:
24

Type:
31

Lifetime:
12

Trait:
22

Other:
21
```

---

# 42. テスト

各エラーに対して、

```text
input.rs
expected.txt
```

を用意する。

例：

```text
tests/
└── E0596/
    ├── input.rs
    └── expected.txt
```

---

# 43. Golden Test

```bash
cargo test
```

で、

```text
Rustコード
 ↓
rustc
 ↓
jpcargo
 ↓
期待される日本語
```

を比較する。

---

# 44. Rustバージョン対応

Rustコンパイラの診断形式が変更される可能性があるため、

```text
stable
beta
nightly
```

をCIでテストする。

---

# 45. エラー網羅率

単純なエラーコード数だけではなく、

```text
診断取得率
日本語化率
原因説明率
修正案提示率
```

を測定する。

例えば、

```text
Rust診断:
1000種類

認識:
950

日本語化:
850

原因説明:
700

修正案:
500
```

のように評価する。

---

# 46. 開発Phase

## Phase 1 — プロトタイプ

実装：

```text
jpcargo run
```

だけ。

目標：

```text
jpcargo run
 ↓
cargo run
 ↓
rustc
 ↓
JSON
 ↓
E0596
 ↓
日本語表示
```

---

# 47. Phase 2 — Cargoコマンド対応

対応：

```bash
jpcargo run
jpcargo build
jpcargo check
jpcargo test
```

---

# 48. Phase 3 — エラー解析

実装：

```text
エラーコード
行番号
列番号
ファイル
変数名
help
note
suggestion
```

---

# 49. Phase 4 — 主要エラー対応

重点：

```text
E0308
E0382
E0502
E0506
E0596
E0597
E0277
E0282
E0425
E0433
```

---

# 50. Phase 5 — 日本語説明エンジン

```text
原文
 ↓
エラー分類
 ↓
Rust概念
 ↓
日本語説明
 ↓
修正方法
```

を実装する。

---

# 51. Phase 6 — jpcargo explain

```bash
jpcargo explain E0382
```

を実装する。

---

# 52. Phase 7 — jpcargo search

```bash
jpcargo search borrow
```

などを実装する。

---

# 53. Phase 8 — エラー網羅率向上

Rustの標準診断を可能な限り登録する。

```text
E0001
E0002
...
```

のように網羅していく。

ただし、存在しない・廃止されたエラーコードまで無理に実装しない。

---

# 54. Phase 9 — 自動修正

```bash
jpcargo fix
```

を実装。

rustcのsuggestionを利用する。

---

# 55. Phase 10 — IDE連携

将来的にVS Code拡張を作る。

```text
VS Code
 ↓
rust-analyzer
 ↓
jpcargo
 ↓
日本語診断
```

を目指す。

---

# 56. Phase 11 — AIによる高度な説明

将来的には、ルールベースだけでは説明が難しいエラーについて、

```text
rustc診断
+
ソースコード
+
Rust型情報
+
エラー履歴
```

を利用した高度な説明機能を検討する。

ただし、**コアの診断結果そのものはrustcを正とする**。

AIがコンパイラの診断を勝手に変更しないようにする。

---

# 57. AI機能の位置付け

AIは、

```text
コンパイル結果を生成するもの
```

ではなく、

```text
コンパイラが出した診断を人間に説明するもの
```

として使用する。

```text
rustc
 ↓
正確な診断
 ↓
jpcargo
 ↓
必要に応じてAI
 ↓
自然な日本語説明
```

とする。

---

# 58. 重要な設計原則

## 原則1

**rustcの診断を最優先する。**

## 原則2

**単純な英語翻訳にしない。**

## 原則3

**エラーコードを保持する。**

## 原則4

**ファイル・行・列を保持する。**

## 原則5

**原文を確認できるようにする。**

## 原則6

**未対応エラーでも動作する。**

## 原則7

**Cargoの互換性をできるだけ維持する。**

---

# 59. 最初の完成目標

最初のMVPでは、

```bash
jpcargo run
```

だけを完成させる。

例えば、

```rust
fn main() {
    let a = 1;
    a = 2;
}
```

に対して、

```text
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
エラー [E0596]
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

変数「a」がミュータブルではないため、
値を変更できません。

発生箇所:
src/main.rs:3:5

原因:
Rustでは変数はデフォルトで
イミュータブル（変更不可）です。

修正方法:

let a = 1;

↓

let mut a = 1;

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

を出せれば、MVP完成とする。

---

# 60. 最終的なコマンド体系

最終的には以下を目標とする。

```bash
# Cargo互換
jpcargo build
jpcargo run
jpcargo check
jpcargo test
jpcargo clippy
jpcargo doc

# 日本語診断
jpcargo explain E0382
jpcargo search borrow
jpcargo list
jpcargo stats

# 診断・環境
jpcargo doctor

# 自動修正
jpcargo fix
```

---

# 61. 完成時の全体像

```text
                         ┌──────────────┐
                         │   jpcargo    │
                         └──────┬───────┘
                                │
               ┌────────────────┼────────────────┐
               │                │                │
               ▼                ▼                ▼
            run              build            check
               │                │                │
               └────────────────┼────────────────┘
                                ▼
                              Cargo
                                │
                                ▼
                              rustc
                                │
                                ▼
                       JSON Diagnostic
                                │
                                ▼
                    ┌─────────────────────┐
                    │ Diagnostic Parser   │
                    └──────────┬──────────┘
                               │
                               ▼
                    ┌─────────────────────┐
                    │ Error Classifier    │
                    └──────────┬──────────┘
                               │
                ┌──────────────┼──────────────┐
                ▼              ▼              ▼
            Error DB       Source Code     Suggestions
                │              │              │
                └──────────────┼──────────────┘
                               ▼
                    Japanese Explanation
                               │
                               ▼
                         Terminal Output
```

---

# 62. 最終目標

`jpcargo`の最終的な目的は、

> **Cargoを日本語化することではなく、Rustコンパイラが持つ高度な診断能力を、日本語で理解できる形に変換すること**

である。

通常のRust開発：

```bash
cargo run
```

ではなく、

```bash
jpcargo run
```

とするだけで、

```text
コンパイル
    ↓
エラー検出
    ↓
エラーコード取得
    ↓
発生箇所特定
    ↓
Rust概念の特定
    ↓
日本語説明
    ↓
原因説明
    ↓
修正方法
    ↓
必要に応じて修正コード
```

までを一連の処理として提供する。

最終的には、

**「Rustのエラーを日本語に翻訳するツール」ではなく、「Rustのコンパイラエラーを日本語で理解できるようにする開発環境」**

として完成させる。
