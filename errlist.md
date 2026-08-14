# jpcargo Rustエラー一覧・日本語診断データベース計画

## 1. 目的

`jpcargo`でRustのコンパイラ・Cargo・環境関連エラーを取得し、

- エラーコード
- エラーの分類
- 発生原因
- 具体的なコード例
- 日本語による説明
- 修正例

まで整理して表示できるようにする。

単純な英語翻訳ではなく、Rustの**所有権・借用・ライフタイム・型・Trait**などの概念まで説明することを目標とする。

---

# 2. エラーの分類

最終的には以下の3系統に分類する。

```text
Rustエラー
├── Rust Compiler
│   ├── Syntax
│   ├── Type
│   ├── Ownership
│   ├── Borrow
│   ├── Lifetime
│   ├── Mutability
│   ├── Trait
│   ├── Generic
│   ├── Pattern
│   ├── Macro
│   ├── Async
│   └── Unsafe
│
├── Cargo
│   ├── Dependency
│   ├── Feature
│   ├── Package
│   └── Version
│
└── Environment
    ├── Linker
    ├── Compiler
    ├── Toolchain
    └── Platform
```

---

# 3. 変数・ミュータビリティ

## E0384 — 不変変数への再代入

### 具体例

```rust
fn main() {
    let a = 1;
    a = 2;
}
```

### 原因

`a`はデフォルトでイミュータブル（変更不可）として宣言されている。

### jpcargoでの説明

```text
エラー [E0384]

変数「a」はイミュータブルとして宣言されています。
そのため、値を変更できません。

修正方法:
`mut`を追加して可変変数にしてください。

修正前:
let a = 1;

修正後:
let mut a = 1;
```

---

## E0596 — 借用した値を変更できない

### 具体例

```rust
fn change(x: &i32) {
    *x = 10;
}
```

### 原因

`&i32`は不変参照なので、参照先の値を変更できない。

### 修正例

```rust
fn change(x: &mut i32) {
    *x = 10;
}
```

### jpcargoでの説明

```text
変数を変更しようとしていますが、
現在取得している参照はイミュータブル（不変）です。

変更する場合は `&mut` を使用してください。
```

---

## E0594 — 不変な値を変更しようとしている

### 例

```rust
let x = 10;
*x = 20;
```

### 原因

変更対象が可変として宣言されていない。

### 対応

`mut`や`&mut`が必要か確認する。

---

# 4. 所有権

## E0382 — Move後の値を使用

### 具体例

```rust
fn main() {
    let s = String::from("hello");

    let a = s;

    println!("{}", s);
}
```

### 原因

```text
s
 ↓ 所有権移動
a
```

`String`の所有権が`a`へ移動したため、`s`は使用できない。

### jpcargoでの説明

```text
エラー [E0382]

値「s」の所有権は「a」に移動しています。

所有権が移動した後は、
元の変数「s」を使用することはできません。

修正方法の例:

1. clone()を使用する
2. 参照を使用する
3. 所有権を移動する設計を変更する
```

### 修正例

```rust
let a = s.clone();
println!("{}", s);
```

または、

```rust
let a = &s;
println!("{}", s);
```

---

## E0505 — 借用中の値をMove

### 具体例

```rust
fn main() {
    let s = String::from("hello");

    let r = &s;

    drop(s);

    println!("{}", r);
}
```

### 原因

`r`が`s`を借用している間に、`s`をMoveしている。

### jpcargoでの説明

```text
値「s」は現在「r」によって借用されています。

借用が有効な間に「s」の所有権を移動することはできません。
```

---

## E0507 — 借用した値をMove

### 具体例

```rust
fn foo(x: &String) {
    let y = *x;
}
```

### 原因

借用している値から所有権を取り出そうとしている。

---

# 5. 借用・Borrow Checker

## E0499 — 同時に複数の可変借用

### 具体例

```rust
fn main() {
    let mut x = 10;

    let a = &mut x;
    let b = &mut x;

    println!("{}", a);
    println!("{}", b);
}
```

### 原因

同じ値に対して同時に複数の`&mut`を作っている。

### jpcargoでの説明

```text
エラー [E0499]

変数「x」はすでに可変借用されています。

その可変借用が有効な間に、
もう一度可変借用することはできません。

Rustでは、同時に複数の可変借用を作ることはできません。
```

---

## E0502 — 可変借用と不変借用の競合

### 具体例

```rust
fn main() {
    let mut x = 10;

    let a = &x;
    let b = &mut x;

    println!("{}", a);
}
```

### 原因

`x`が不変借用されている間に、可変借用している。

### 原則

```text
複数の不変借用
    または
1つの可変借用

を基本とする。
```

---

## E0506 — 借用中の値への代入

### 具体例

```rust
fn main() {
    let mut x = 10;

    let r = &x;

    x = 20;

    println!("{}", r);
}
```

### 原因

`r`が`x`を借用している間に、`x`へ代入している。

---

# 6. ライフタイム

## E0597 — 借用した値が早く破棄される

### 具体例

```rust
fn main() {
    let r;

    {
        let x = 10;
        r = &x;
    }

    println!("{}", r);
}
```

### 原因

`x`は内側のスコープ終了時に破棄される。

しかし`r`はその後も`x`を参照しようとしている。

### jpcargoでの説明

```text
エラー [E0597]

「x」への参照が、
「x」の生存期間より長く使用されようとしています。

「x」は内側のスコープ終了時に破棄されるため、
その後に「r」から参照することはできません。
```

---

## E0106 — ライフタイム指定不足

参照を戻り値として返す関数などで、ライフタイムを推論できない場合に発生する。

### 例

```rust
fn get(x: &str, y: &str) -> &str {
    if true {
        x
    } else {
        y
    }
}
```

状況によっては、どの入力参照と戻り値が結び付くのか明示する必要がある。

---

## E0621 — ライフタイム指定が必要

### 例

```rust
fn foo<'a>(x: &'a str, y: &str) -> &'a str {
    y
}
```

引数と戻り値のライフタイム関係が一致していない。

---

# 7. 型エラー

## E0308 — 型が一致しない

Rustで非常に重要なエラー。

### 具体例

```rust
fn main() {
    let x: i32 = "hello";
}
```

### jpcargoでの説明

```text
エラー [E0308]

型が一致していません。

期待されている型:
i32

実際に指定された型:
&str

発生箇所:
src/main.rs:2
```

### 修正例

```rust
let x: i32 = 123;
```

または、

```rust
let x: &str = "hello";
```

---

## E0282 — 型を推論できない

### 例

```rust
let x = Vec::new();
```

Rustは`Vec<T>`の`T`を決定できない。

### 修正

```rust
let x: Vec<i32> = Vec::new();
```

---

## E0283 — Traitの型を推論できない

### 例

```rust
let x = "10".parse();
```

どの型へ変換するか決定できない。

### 修正

```rust
let x: i32 = "10".parse().unwrap();
```

---

## E0271 — 型の関連付けが一致しない

Traitやassociated typeなどの制約が一致しない場合に発生する。

---

# 8. Trait

## E0277 — Traitを実装していない

### 具体例

```rust
struct Person;

fn print<T: std::fmt::Display>(x: T) {
    println!("{}", x);
}

fn main() {
    print(Person);
}
```

### 原因

`Person`が`Display`を実装していない。

### jpcargoでの説明

```text
エラー [E0277]

型「Person」はTrait「Display」を実装していません。

この関数はDisplayを実装している型を要求しています。

修正方法:
PersonにDisplayを実装してください。
```

---

## E0119 — Trait実装が重複

### 例

```rust
trait Foo {}

impl Foo for i32 {}
impl Foo for i32 {}
```

同じTraitを同じ型へ複数回実装している。

---

## E0117 — 外部Trait・外部型への実装制限

RustのOrphan Ruleに関連する。

---

## E0207 — 制約されていない型パラメータ

### 例

```rust
struct Foo;

impl<T> Foo {
}
```

`T`が実際の型やTraitと適切に関連付けられていない。

---

## E0210 — Trait実装に関する型パラメータ制約

外部Trait・外部型とジェネリクスを組み合わせた場合などに発生する。

---

# 9. 関数

## E0061 — 引数の数が違う

### 具体例

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    add(1);
}
```

### jpcargoでの説明

```text
エラー [E0061]

関数「add」は2個の引数を必要としますが、
1個しか指定されていません。
```

---

## E0062 — 同じ引数を複数回指定

同じ引数名を重複して指定するなど、引数指定が不正な場合。

---

## E0308 — 戻り値の型が違う

### 例

```rust
fn get_number() -> i32 {
    "hello"
}
```

---

# 10. 構造体

## E0063 — 必須フィールドがない

### 例

```rust
struct User {
    name: String,
    age: i32,
}

let user = User {
    name: "Alice".to_string(),
};
```

### 原因

`age`フィールドが不足している。

### 修正

```rust
let user = User {
    name: "Alice".to_string(),
    age: 20,
};
```

---

## E0609 — 存在しないフィールド

### 例

```rust
struct User {
    name: String,
}

let user = User {
    name: String::from("Alice"),
};

println!("{}", user.age);
```

`User`に`age`フィールドが存在しない。

---

# 11. enum / match

## E0004 — パターンが網羅されていない

### 例

```rust
enum Color {
    Red,
    Blue,
}

fn main() {
    let color = Color::Red;

    match color {
        Color::Red => println!("red"),
    }
}
```

### 原因

`Color::Blue`の処理がない。

### 修正

```rust
match color {
    Color::Red => println!("red"),
    Color::Blue => println!("blue"),
}
```

---

## E0027 — 構造体パターンのフィールド不足

### 例

```rust
struct User {
    name: String,
    age: i32,
}

match user {
    User { name } => {}
}
```

`age`の扱いが不足している。

---

## E0026 — 存在しないフィールドをパターンで指定

```rust
User {
    address,
}
```

`address`フィールドが存在しない。

---

# 12. Module / use / 名前解決

## E0425 — 名前が見つからない

### 例

```rust
fn main() {
    println!("{}", value);
}
```

### jpcargoでの説明

```text
エラー [E0425]

「value」という名前が見つかりません。

確認してください:

- 変数を宣言しているか
- スコープ内に存在するか
- 名前を間違えていないか
```

---

## E0432 — 未解決のimport

### 例

```rust
use my_module::Unknown;
```

`Unknown`が存在しない。

---

## E0433 — モジュール・型を解決できない

### 例

```rust
foo::bar();
```

`foo`が存在しない。

---

## E0428 — 同じ名前を複数定義

### 例

```rust
fn foo() {}
fn foo() {}
```

---

## E0252 — 同じ名前を複数回import

```rust
use std::io;
use std::fs as io;
```

---

## E0255 — 同じ名前を複数回定義

同一スコープ内で同じ名前の項目を複数定義した場合。

---

# 13. Visibility

## E0603 — 非公開項目へのアクセス

### 例

```rust
mod foo {
    fn secret() {}
}

fn main() {
    foo::secret();
}
```

### 修正

```rust
mod foo {
    pub fn secret() {}
}
```

---

## E0616 — privateフィールドへのアクセス

構造体の非公開フィールドをモジュール境界の外側から使用しようとした場合。

---

# 14. 型・参照

## E0606 — 不正な型変換

### 例

```rust
let x = "hello";
let y = x as i32;
```

`&str`から`i32`へ直接`as`変換することはできない。

---

## E0614 — Dereferenceできない

### 例

```rust
let x = 10;
let y = *x;
```

`i32`はそのままdereferenceできない。

---

## E0608 — Indexできない

### 例

```rust
let x = 10;
println!("{}", x[0]);
```

`i32`は配列やスライスのようにindexできない。

---

# 15. メソッド・演算子

## E0599 — メソッドが存在しない

非常に重要。

### 例

```rust
let x = String::from("hello");

x.unknown_method();
```

### jpcargoでの説明

```text
エラー [E0599]

型「String」に、
「unknown_method」というメソッドはありません。

確認してください:

- メソッド名が正しいか
- 必要なTraitをimportしているか
- 対象の型が想定した型か
```

---

## E0369 — 演算子を使用できない

### 例

```rust
let a = "hello";
let b = "world";

let c = a - b;
```

`-`演算子を`&str`に使用できない。

---

## E0368 — 代入演算子を使用できない

### 例

```rust
let mut x = "hello";
x += "world";
```

型に`AddAssign`など必要なTraitが実装されていない場合。

---

## E0600 — 演算子を使用できない

型に必要な演算子Traitが実装されていない場合。

---

# 16. Closure

## E0521 — 借用データがclosure外へ逃げる

closureの生存期間よりも長く借用データを保持しようとした場合などに発生する。

---

## E0593 — closureの引数数が違う

### 例

```rust
let f = |x| x;

f(1, 2);
```

closureは1個の引数しか受け取らない。

---

# 17. async / await

## E0728 — asyncコンテキスト外でawait

### 例

```rust
fn main() {
    foo().await;
}
```

### 修正例

```rust
async fn main() {
    foo().await;
}
```

ただし、通常のRustバイナリではasync mainを直接実行するためのruntimeも必要になる。

---

## E0277 — Future関連のTrait不足

asyncコードでは`Future`、`Send`、`Sync`などのTrait制約によってE0277が発生することがある。

---

# 18. const / static

## E0015 — constで許可されない関数呼び出し

### 例

```rust
const X: String = String::new();
```

constコンテキストで許可されない処理を使用している場合。

---

## E0080 — 定数評価中のエラー

### 例

```rust
const X: i32 = 1 / 0;
```

コンパイル時の定数評価に失敗する。

---

# 19. unsafe

## E0133 — unsafe操作を安全な場所で使用

### 例

```rust
unsafe fn dangerous() {}

fn main() {
    dangerous();
}
```

### 修正

```rust
unsafe {
    dangerous();
}
```

### jpcargoでの説明

```text
エラー [E0133]

unsafeな関数を安全なコンテキストから呼び出しています。

この操作にはunsafeブロックが必要です。

注意:
unsafeを追加するだけでなく、
その操作が安全であることを確認してください。
```

---

# 20. 再帰型

## E0072 — 再帰型のサイズが無限

### 例

```rust
struct Node {
    next: Node,
}
```

### 原因

`Node`の中に`Node`を直接含めるため、型のサイズが無限になる。

### 修正

```rust
struct Node {
    next: Box<Node>,
}
```

### jpcargoでの説明

```text
エラー [E0072]

型「Node」のサイズをコンパイル時に決定できません。

Nodeの中に直接Nodeが含まれているため、
無限に大きな型になっています。

Box<Node>などを使用して間接参照してください。
```

---

# 21. Sized

## E0161 — Sizedでない値のMove

`dyn Trait`など、コンパイル時にサイズを決定できない型を直接Moveしようとした場合などに発生する。

---

## E0277 — Sized制約

ジェネリック型に`Sized`が必要な状況で発生する。

---

# 22. Drop

## E0040 — Dropの明示的呼び出し

### 例

```rust
let x = Foo {};
x.drop();
```

Rustでは`Drop::drop`を直接呼び出すことは禁止されている。

### 修正

```rust
drop(x);
```

---

# 23. 再帰・循環定義

## E0391 — 定義の循環

### 例

```rust
type A = B;
type B = A;
```

型定義が循環している。

---

# 24. Return

## E0069 — 値を返せない場所でreturn値を使用

### 例

```rust
fn foo() {
    return 10;
}
```

### 原因

この関数の戻り値型は`()なのに、`i32`を返している。

---

# 25. Macro

## E0659 — マクロ名が曖昧

複数のマクロが同じ名前を持ち、どれを使用するのか判断できない場合。

---

# 26. Feature / unstable

## E0658 — unstable feature

stable Rustでは利用できない不安定な機能を使用した場合など。

---

# 27. Cargo関連エラー

Rustの問題は`rustc`だけではない。

## Dependencyエラー

### 例

```toml
[dependencies]
unknown_package = "1.0"
```

### jpcargo

```text
Cargoエラー

依存パッケージ「unknown_package」を
取得または解決できませんでした。

確認してください:

- パッケージ名
- バージョン
- crates.io
- ネットワーク
```

---

## Version Conflict

複数の依存パッケージが互換性のないバージョンを要求する場合。

```text
package A requires serde 1.x
package B requires serde 2.x
```

など。

---

## Feature不足

必要なCargo featureを有効にしていない場合。

---

# 28. Linker関連

Rustコードが正しくても、環境が原因でビルドに失敗する場合がある。

### 例

```text
linker `cc` not found
```

### jpcargo

```text
環境エラー

Rustコードではなく、
リンカー環境に問題がある可能性があります。

確認してください:

- C compiler
- linker
- gcc
- clang
- build-essential
```

---

# 29. Toolchain関連

確認対象：

```text
rustc
cargo
rustup
toolchain
target
host
architecture
```

例えば、

```bash
rustc --version
cargo --version
rustup show
```

などを利用する。

---

# 30. エラーDBの形式

`jpcargo`内部では、エラー情報を構造化して管理する。

例：

```json
{
  "code": "E0382",
  "category": "ownership",
  "title": "所有権が移動した値を使用しています",
  "beginner": "この値はすでに別の変数へ所有権が移動しているため、元の変数から使用できません。",
  "concept": "ownership",
  "solutions": [
    "clone()を使用する",
    "参照を使用する",
    "所有権を移動する設計を変更する"
  ]
}
```

---

# 31. エラーDBのディレクトリ構成

```text
diagnostics/
├── compiler/
│   ├── syntax/
│   ├── type/
│   ├── ownership/
│   ├── borrow/
│   ├── lifetime/
│   ├── mutability/
│   ├── trait/
│   ├── generic/
│   ├── pattern/
│   ├── macro/
│   ├── async/
│   └── unsafe/
│
├── cargo/
│   ├── dependency/
│   ├── feature/
│   ├── package/
│   └── version/
│
└── environment/
    ├── linker/
    ├── compiler/
    ├── toolchain/
    └── platform/
```

---

# 32. jpcargoで優先して実装するエラー

## Sランク

最初に対応する。

| エラー | 分類 | 重要度 |
|---|---|---|
| E0308 | 型 | S |
| E0382 | 所有権 | S |
| E0499 | 借用 | S |
| E0502 | 借用 | S |
| E0505 | 所有権 | S |
| E0507 | 所有権 | S |
| E0596 | ミュータビリティ | S |
| E0597 | ライフタイム | S |
| E0277 | Trait | S |
| E0599 | メソッド/Trait | S |

## Aランク

| エラー | 分類 |
|---|---|
| E0425 | 名前解決 |
| E0432 | import |
| E0433 | module |
| E0061 | 関数 |
| E0004 | match |
| E0072 | 再帰型 |
| E0133 | unsafe |
| E0282 | 型推論 |
| E0283 | Trait推論 |
| E0384 | mutability |
| E0603 | visibility |

## Bランク

その他のコンパイラ診断。

---

# 33. jpcargoの診断処理

最終的な処理は以下とする。

```text
jpcargo run
      │
      ▼
Cargo実行
      │
      ▼
rustc
      │
      ▼
JSON Diagnostic
      │
      ▼
診断情報Parser
      │
      ▼
エラーコード解析
      │
      ▼
カテゴリ分類
      │
      ▼
ソースコード解析
      │
      ▼
エラーDB検索
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

# 34. 日本語エラー表示の目標

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
エラー [E0384]
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

変数「a」がイミュータブル（変更不可）なのに、
値を変更しようとしています。

発生箇所:
src/main.rs:3:5

コード:

  1 | fn main() {
  2 |     let a = 1;
> 3 |     a = 2;
    |     ^ 変更しようとしています
  4 | }

原因:

Rustでは、変数はデフォルトで
イミュータブル（変更不可）です。

修正方法:

修正前:
let a = 1;

修正後:
let mut a = 1;

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

という表示を目標とする。

---

# 35. 単なる翻訳にしない

悪い例：

```text
cannot assign to `a`, as it is not declared as mutable
```

↓

```text
aに代入できません。mutableとして宣言されていません。
```

これだけでは初心者には分かりにくい。

`jpcargo`では、

```text
何が起きたか
↓
なぜ起きたか
↓
Rustのどのルールによるものか
↓
どこで起きたか
↓
どう直すか
↓
修正後のコード
```

まで説明する。

---

# 36. 原文も保持する

日本語化してもRustの原文を確認できるようにする。

```bash
jpcargo run --original
```

表示例：

```text
日本語:

変数「a」は変更できません。

原文:

cannot assign to `a`, as it is not declared as mutable
```

---

# 37. エラーコード解説

```bash
jpcargo explain E0382
```

例：

```text
E0382
━━━━━━━━━━━━━━━━━━

分類:
所有権

名称:
Use of moved value

日本語:
所有権が移動した値を使用しています。

簡単な説明:
Rustでは、所有権が別の変数へ移動した後、
元の変数を使用することはできません。

代表的な修正:
- clone()
- 参照
- 所有権設計の変更
```

---

# 38. エラー検索

```bash
jpcargo search borrow
```

将来的には自然言語検索も検討する。

```bash
jpcargo search "変数を変更できない"
```

---

# 39. 未対応エラー

未登録のエラーが発生しても、`jpcargo`は処理を停止しない。

```text
━━━━━━━━━━━━━━━━━━━━
未対応の診断
━━━━━━━━━━━━━━━━━━━━

エラーコード:
E1234

現在、このエラーの詳細な日本語解説は
登録されていません。

Rustコンパイラの原文を表示します。
```

---

# 40. 重要な注意点

Rustの「エラー一覧」を作る際には、

```text
エラーコード一覧
```

だけでは不十分。

Rustには、

- エラーコード付きdiagnostic
- エラーコードなしdiagnostic
- warning
- note
- help
- suggestion
- Cargoエラー
- linkerエラー
- OSエラー
- toolchainエラー

が存在する。

したがって`jpcargo`では、

```text
エラーコードDB
+
診断メッセージ解析
+
ソースコード解析
+
Cargo診断
+
環境診断
```

を組み合わせる。

---

# 41. 最終目標

最終的には、

```bash
jpcargo run
```

だけで、

```text
Rustコード
    ↓
Cargo
    ↓
rustc
    ↓
診断情報
    ↓
エラーコード
    ↓
エラー分類
    ↓
ソースコード解析
    ↓
日本語化
    ↓
原因説明
    ↓
修正方法
    ↓
修正コード
```

までを自動化する。

最終的な`jpcargo`は、

> Rustのエラーを日本語に翻訳するだけのツール

ではなく、

> **Rustコンパイラの診断結果を、日本語で理解・学習・修正できる形に変換する開発支援ツール**

を目指す。
