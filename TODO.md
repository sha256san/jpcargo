# TODO.md - jpcargo 開発タスク・未実装一覧

## 進行中 / フェーズ別タスク

### Phase 1: MVP（基本プロトタイプ）
- [x] プロジェクト構成定義 (`Cargo.toml`, `SPEC.md`, `TODO.md`, `MEMORY.md`, `AGENTS.md`, `CHANGELOG.md`)
- [x] CLI フレームワーク構築 (`clap` による引数解析、サブコマンド設定)
- [x] Cargo プロセス実行 & JSON 診断ストリーム解析 (`cargo.rs`, `parser.rs`)
- [x] リッチターミナルレンダラー (`renderer/terminal.rs`)
- [x] 第1優先エラー（初期実装）:
  - [x] `E0384`: cannot assign twice to immutable variable
  - [x] `E0596`: cannot assign to immutable variable
  - [x] `E0308`: mismatched types
  - [x] `E0382`: use of moved value
  - [x] `E0502`: cannot borrow as mutable because also borrowed as immutable
  - [x] `E0506`: cannot assign to borrowed value
  - [x] `E0597`: value does not live long enough
- [x] `jpcargo explain <CODE>` コマンド
- [x] `jpcargo list` コマンド
- [x] `jpcargo stats` コマンド
- [x] 未対応エラー発生時のクリーンフォールバック表示

### Phase 2: Cargo コマンド全対応
- [x] `jpcargo run`
- [x] `jpcargo build`
- [x] `jpcargo check`
- [x] `jpcargo test`
- [x] `jpcargo clippy` (Clippy Lint 警告の日本語化)
- [x] `jpcargo doc` (透過的実行)

### Phase 3: errlist.md に基づくエラー診断ルールの完全網羅実装
#### 3.1 Sランク エラー群
- [x] `E0308`: 型が一致しない (Mismatched Types)
- [x] `E0382`: Move後の値を使用 (Use of moved value)
- [x] `E0499`: 同時に複数の可変借用 (Cannot borrow as mutable more than once)
- [x] `E0502`: 可変借用と不変借用の競合
- [x] `E0505`: 借用中の値をMove (Cannot move out of value because it is borrowed)
- [x] `E0507`: 借用した値をMove (Cannot move out of borrowed context)
- [x] `E0596`: 借用した値を変更できない (Cannot assign to immutable borrowed content)
- [x] `E0597`: 借用した値が早く破棄される (Does not live long enough)
- [x] `E0277`: Traitを実装していない (Trait bound not satisfied)
- [x] `E0599`: メソッドが存在しない / Trait未インポート (No method found)

#### 3.2 Aランク エラー群
- [x] `E0425`: 名前が見つからない (Cannot find value in scope)
- [x] `E0432`: 未解決のimport (Unresolved import)
- [x] `E0433`: モジュール・型を解決できない (Failed to resolve module)
- [x] `E0061`: 関数の引数の数が違う (Function takes N arguments but M supplied)
- [x] `E0004`: matchのパターンが網羅されていない (Non-exhaustive patterns)
- [x] `E0072`: 再帰型のサイズが無限 (Has infinite size)
- [x] `E0133`: unsafe操作を安全な場所で使用 (Call to unsafe function requires unsafe block)
- [x] `E0282`: 型を推論できない (Type annotations needed)
- [x] `E0283`: Traitの型を推論できない (Type annotations needed for trait)
- [x] `E0384`: 不変変数への再代入
- [x] `E0603`: 非公開項目へのアクセス (Item is private)

#### 3.3 Bランク & 追加主要エラー群
- [x] `E0594`: 不変な値を変更しようとしている (Cannot assign to immutable value)
- [x] `E0506`: 借用中の値への代入
- [x] `E0106`: ライフタイム指定不足 (Missing lifetime specifier)
- [x] `E0621`: ライフタイム指定が必要 (Lifetime mismatch in parameters)
- [x] `E0271`: 型の関連付けが一致しない (Type mismatch resolving trait associated type)
- [x] `E0119`: Trait実装が重複 (Conflicting implementations of trait)
- [x] `E0117`: 外部Trait・外部型への実装制限 (Orphan rule violation)
- [x] `E0207`: 制約されていない型パラメータ (Unconstrained type parameter)
- [x] `E0210`: Trait実装に関する型パラメータ制約 (Type parameter before local type)
- [x] `E0062`: 同じ引数を複数回指定 (Field specified more than once)
- [x] `E0069`: 値を返せない場所でreturn (Return statement in function returning ())
- [x] `E0063`: 構造体の必須フィールドがない (Missing fields in struct)
- [x] `E0609`: 構造体に存在しないフィールド (No field on struct)
- [x] `E0027`: 構造体パターンのフィールド不足 (Pattern does not mention field)
- [x] `E0026`: 存在しないフィールドをパターンで指定 (Struct pattern has no field)
- [x] `E0428`: 同じ名前を複数定義 (A value for identifier already exists)
- [x] `E0252`: 同じ名前を複数回import (A value is already imported)
- [x] `E0255`: 同じ名前を複数回定義 (Import shadows existing item)
- [x] `E0616`: privateフィールドへのアクセス (Field is private)
- [x] `E0606`: 不正な型変換 (Cannot cast type)
- [x] `E0614`: Dereferenceできない (Type cannot be dereferenced)
- [x] `E0608`: Indexできない (Cannot index into value)
- [x] `E0369`: 二項演算子を使用できない (Binary operation cannot be applied)
- [x] `E0368`: 代入演算子を使用できない (Binary assignment operator cannot be applied)
- [x] `E0600`: 単項演算子を使用できない (Unary operation cannot be applied)
- [x] `E0521`: 借用データがclosure外へ逃げる (Borrowed data escapes closure)
- [x] `E0593`: closureの引数数が違う (Closure takes N arguments but M expected)
- [x] `E0728`: asyncコンテキスト外でawait (`await` is only allowed inside `async` functions)
- [x] `E0015`: constで許可されない関数呼び出し (Calls in constants are limited)
- [x] `E0080`: 定数評価中のエラー (Evaluation of constant value failed)
- [x] `E0161`: Sizedでない値のMove (Cannot move a value of type unsized)
- [x] `E0040`: Dropの明示的呼び出し (Explicit use of destructor method)
- [x] `E0391`: 定義の循環 (Cycle detected)
- [x] `E0659`: マクロ名が曖昧 (Macro is ambiguous)
- [x] `E0658`: unstable feature (Use of unstable library feature)

### Phase 4: 独自サブコマンド・環境診断・自動修正
- [x] `jpcargo search <KEYWORD>` (借用、所有権、キーワードによるエラー検索)
- [x] `jpcargo doctor` (Rust/Cargo/Toolchain/Linker (cc/gcc/clang)/OS 環境の総合日本語診断)
- [x] `jpcargo fix` (rustc suggestion の安全な自動適用)
- [x] Cargo / Linker エラーの検知と日本語ガイダンス (linker `cc` not found, 依存パッケージ解決失敗など)

### Phase 5: 日本語解説・学習支援機能
- [x] `--level <beginner|normal|expert>` オプションによる解説難易度切り替え
- [x] `--original` フラグの完全統合（原文と日本語の並行表示）
- [x] 日本語用語辞典の大幅拡充 (`dictionary/rust_terms.json`)

### Phase 6: テスト・ドキュメント・CI連携
- [x] 全エラーコードのテストスイート (`tests/rules_test.rs`)
- [x] `SPEC.md`, `MEMORY.md`, `CHANGELOG.md` の同期更新

---

## エラーコード別 対応状況

| エラーコード | カテゴリ | 概要 | 重要度 | 状態 |
|---|---|---|---|---|
| `E0308` | Type | 型が一致しない (Mismatched Types) | S | ✅ 実装済 |
| `E0382` | Ownership | 所有権移動済み値の再使用 (Use of moved value) | S | ✅ 実装済 |
| `E0499` | Borrow | 同時に複数の可変借用 | S | ✅ 実装済 |
| `E0502` | Borrow | 可変借用と不変借用の競合 | S | ✅ 実装済 |
| `E0505` | Ownership | 借用中の値をMove | S | ✅ 実装済 |
| `E0507` | Ownership | 借用した値をMove | S | ✅ 実装済 |
| `E0596` | Mutability | 不変変数の変更・再代入 | S | ✅ 実装済 |
| `E0597` | Borrow/Lifetime | 参照先の生存期間不足 (Does not live long enough) | S | ✅ 実装済 |
| `E0277` | Trait | トレイト境界が満たされていない | S | ✅ 実装済 |
| `E0599` | Trait/Method | メソッドが存在しない / Trait未インポート | S | ✅ 実装済 |
| `E0384` | Mutability | 不変変数の2回目代入・再代入 | A | ✅ 実装済 |
| `E0425` | Name Resolution | スコープ内に値・変数が見つからない | A | ✅ 実装済 |
| `E0432` | Module/Import | 未解決のimport | A | ✅ 実装済 |
| `E0433` | Name Resolution | 未宣言のモジュール・クレート | A | ✅ 実装済 |
| `E0061` | Function | 引数の個数不一致 | A | ✅ 実装済 |
| `E0004` | Pattern/Match | パターンが網羅されていない | A | ✅ 実装済 |
| `E0072` | Type/Struct | 再帰型のサイズが無限 | A | ✅ 実装済 |
| `E0133` | Safety/Unsafe | unsafe 操作のブロック外実行 | A | ✅ 実装済 |
| `E0282` | Type | 型推論不能（型注釈が必要） | A | ✅ 実装済 |
| `E0283` | Trait | Traitの型を推論できない | A | ✅ 実装済 |
| `E0603` | Visibility | 非公開項目へのアクセス | A | ✅ 実装済 |
| `E0594` | Mutability | 不変な値を変更しようとしている | B | ✅ 実装済 |
| `E0506` | Borrow | 借用中変数の再代入 | B | ✅ 実装済 |
| `E0106` | Lifetime | ライフタイム指定子の欠落 | B | ✅ 実装済 |
| `E0621` | Lifetime | ライフタイム指定が必要 | B | ✅ 実装済 |
| `E0271` | Trait | 型の関連付けが一致しない | B | ✅ 実装済 |
| `E0119` | Trait | Trait実装が重複 | B | ✅ 実装済 |
| `E0117` | Trait | 外部Trait・外部型への実装制限 (Orphan Rule) | B | ✅ 実装済 |
| `E0207` | Generic | 制約されていない型パラメータ | B | ✅ 実装済 |
| `E0210` | Trait | Trait実装に関する型パラメータ制約 | B | ✅ 実装済 |
| `E0062` | Function | 同じ引数を複数回指定 | B | ✅ 実装済 |
| `E0069` | Function | 値を返せない場所でreturn | B | ✅ 実装済 |
| `E0063` | Struct | 構造体フィールドの指定漏れ | B | ✅ 実装済 |
| `E0609` | Struct | 存在しないフィールド | B | ✅ 実装済 |
| `E0027` | Pattern | 構造体パターンのフィールド不足 | B | ✅ 実装済 |
| `E0026` | Pattern | 存在しないフィールドをパターン指定 | B | ✅ 実装済 |
| `E0428` | Module | 同じ名前を複数定義 | B | ✅ 実装済 |
| `E0252` | Module | 同じ名前を複数回import | B | ✅ 実装済 |
| `E0255` | Module | 同じ名前を複数回定義 | B | ✅ 実装済 |
| `E0616` | Visibility | privateフィールドへのアクセス | B | ✅ 実装済 |
| `E0606` | Type | 不正な型変換 (`as`) | B | ✅ 実装済 |
| `E0614` | Type | Dereferenceできない | B | ✅ 実装済 |
| `E0608` | Type | Indexできない | B | ✅ 実装済 |
| `E0369` | Operator | 二項演算子を使用できない | B | ✅ 実装済 |
| `E0368` | Operator | 代入演算子を使用できない | B | ✅ 実装済 |
| `E0600` | Operator | 単項演算子を使用できない | B | ✅ 実装済 |
| `E0521` | Closure | 借用データがclosure外へ逃げる | B | ✅ 実装済 |
| `E0593` | Closure | closureの引数数が違う | B | ✅ 実装済 |
| `E0728` | Async | asyncコンテキスト外でawait | B | ✅ 実装済 |
| `E0015` | Const | constで許可されない関数呼び出し | B | ✅ 実装済 |
| `E0080` | Const | 定数評価中のエラー | B | ✅ 実装済 |
| `E0161` | Sized | Sizedでない値のMove | B | ✅ 実装済 |
| `E0040` | Memory | Dropの明示的呼び出し | B | ✅ 実装済 |
| `E0391` | Type | 定義の循環 | B | ✅ 実装済 |
| `E0659` | Macro | マクロ名が曖昧 | B | ✅ 実装済 |
| `E0658` | Feature | unstable feature | B | ✅ 実装済 |
