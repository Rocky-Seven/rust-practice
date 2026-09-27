# Rustの変数と型を学ぼう！（第2回）

> **この記事は「GitHub CodespacesでRustを始めよう！」の続編です。**  
> まだ環境構築をしていない方は、[第1回の記事](https://my-studies.org/get-started-with-rust-on-github-codespaces/)を先にお読みください。

---

## 目次

1. [はじめに](#toc1)
2. [準備：新しいプロジェクトを作成](#toc2)
3. [変数の基本](#toc3)
   1. [不変（immutable）変数](#toc4)
   2. [変数を変更しようとするとエラー](#toc5)
   3. [可変（mutable）変数](#toc6)
   4. [練習問題 1-1](#toc7)
4. [シャドーイング](#toc8)
   1. [シャドーイングとは？](#toc9)
   2. [シャドーイングとmutの違い](#toc10)
   3. [練習問題 2-1](#toc11)
5. [データ型](#toc12)
   1. [整数型](#toc13)
   2. [浮動小数点数型](#toc14)
   3. [真偽値型](#toc15)
   4. [文字型](#toc16)
   5. [練習問題 3-1](#toc17)
6. [複合型](#toc18)
   1. [タプル](#toc19)
   2. [配列](#toc20)
   3. [練習問題 4-1](#toc21)
   4. [練習問題 4-2](#toc22)
7. [型変換](#toc23)
   1. [asキーワード](#toc24)
   2. [文字列への変換](#toc25)
   3. [文字列から数値への変換](#toc26)
   4. [練習問題 5-1](#toc27)
8. [定数](#toc28)
   1. [定数とは？](#toc29)
   2. [定数の宣言](#toc30)
   3. [定数の使用例](#toc31)
   4. [練習問題 6-1](#toc32)
9. [総合練習問題](#toc33)
   1. [チャレンジ1：BMI計算機](#toc34)
   2. [チャレンジ2：温度変換](#toc35)
   3. [チャレンジ3：学生情報](#toc36)
   4. [チャレンジ4：配列の操作](#toc37)
10. [まとめ](#toc38)
11. [次回予告](#toc39)

---

## はじめに

前回の記事では、GitHub Codespacesを使ってRustの環境を構築し、最初の「Hello, world!」プログラムを実行した。

今回は、プログラミングの基礎となる**変数と型**について学んでいこう。

### この記事で学ぶこと

* 変数の宣言と使い方
* 可変（mutable）と不変（immutable）の違い
* シャドーイング（shadowing）
* 基本的なデータ型（整数、浮動小数点数、真偽値、文字）
* 複合型（タプル、配列）
* 型変換
* 定数（const）

---

## 準備：体験用プロジェクトを開く

> **📁 作業場所:** `trials/02_variables/`  
> このディレクトリで自由にコードを試してみよう。失敗しても問題ない！

Codespacesを開いて、ターミナルで以下を実行しよう。

```bash
cd /workspaces/rust-practice/trials/02_variables

# VSCodeでファイルを開く
code src/main.rs

# 動作確認
cargo run
```
これで、体験用プロジェクトの準備が整った。

## トラブルシューティング
```bash
cargo run 
error: failed to parse manifest at /workspaces/rust-practice/trials/02_variables/Cargo.toml
```
(略）

02_variablesのsrc/main.rsが存在しないために起きているエラーです。

原因

trials/02_variables/.gitignoreにも、おそらくsrc/main.rsが書かれていたはずですが、  
このファイルは最初からGitHubに一度もpushされていません。

前回まで動いていたのは、最初にファイルを作ったCodespaceに、実体としてのmain.rsが残っていたからです。  
しかし、以下のようなことがあると、ファイル自体が消えてしまいます。

- Codespaceを作り直した（前のCodespaceが削除された）
- 別の環境でgit cloneまたはgit pullした

.gitignoreで除外されているファイルは、リポジトリの中に実体がないため、
新しい環境には最初から存在しません。

確認方法
```bash
ls trials/02_variables/src
```
何も表示されない、またはmain.rsがなければ、上記の理由です。

### 直し方

main.rsを作り直せば解決します。最低限、以下の内容があれば動きます。

```bash
cd /workspaces/rust-practice/trials/02_variables

cat > src/main.rs << 'EOF'
// 第2回：変数と型 - 体験用
// このファイルは自由に編集してください

fn main() {
    println!("変数と型を学ぼう！");

    // ここに自由にコードを書いてください
}
EOF

```
再度実行してください。
```bash
cargo run
```
**📝 このディレクトリについて：**
- ブログ記事を読みながら、コードを試す場所
- `src/main.rs` を自由に編集できる
- エラーが出ても大丈夫、どんどん試そう！

---

## 変数の基本

### 不変（immutable）変数

Rustの変数は、**デフォルトで不変（変更できない）**である。これは他の言語と異なる大きな特徴だ。

`src/main.rs`を開いて、以下のコードを書いてみよう。

```rust
fn main() {
    let x = 5;
    println!("xの値は: {}", x);
}
```

**実行：**

```bash
cargo run
```

**出力：**

```
xの値は: 5
```

---

### 変数を変更しようとするとエラー

次に、以下のコードを試してみよう。

```rust
fn main() {
    let x = 5;
    println!("xの値は: {}", x);
    
    x = 6; // ← エラーになる！
    println!("xの値は: {}", x);
}
```

**実行すると、エラーが出る：**

```
error[E0384]: cannot assign twice to immutable variable `x`
 --> src/main.rs:5:5
  |
2 |     let x = 5;
  |         -
  |         |
  |         first assignment to `x`
  |         help: consider making this binding mutable: `mut x`
5 |     x = 6;
  |     ^^^^^ cannot assign twice to immutable variable
```

**エラーの意味：**

* 変数`x`は不変なので、2回代入できない
* 解決方法：`mut`キーワードを使う

**なぜデフォルトで不変？**

Rustは**安全性**を重視している。変数が不変であれば、予期しない変更によるバグを防げる。変更が必要な場合のみ、明示的に`mut`を付けることで意図を明確にする。

---

### 可変（mutable）変数

変数を変更したい場合は、`mut`キーワードを使う。

```rust
fn main() {
    let mut x = 5;
    println!("xの値は: {}", x);
    
    x = 6;
    println!("xの値は: {}", x);
}
```

**実行：**

```bash
cargo run
```

**出力：**

```
xの値は: 5
xの値は: 6
```

**成功！** 🎉

---

### 練習問題 1-1

以下のプログラムを作成してみよう。

1. `age`という変数を作り、自分の年齢を代入
2. 年齢を出力
3. 次の年の年齢を計算して、`age`に代入
4. 新しい年齢を出力

**ヒント：** `mut`を使う必要がある。

<details>
<summary>解答例を見る</summary>

```rust
fn main() {
    let mut age = 25;
    println!("現在の年齢: {}", age);
    
    age = age + 1;
    println!("来年の年齢: {}", age);
}
```

</details>

---

## シャドーイング

### シャドーイングとは？

同じ変数名で、新しい変数を宣言できる。これを**シャドーイング**という。

```rust
fn main() {
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    
    println!("xの値は: {}", x);
}
```

**実行：**

```bash
cargo run
```

**出力：**

```
xの値は: 12
```

**何が起きた？**

1. 最初の`x`は5
2. 2番目の`x`は6（5 + 1）
3. 3番目の`x`は12（6 * 2）

---

### シャドーイングとmutの違い

**シャドーイングの利点：**

#### 1. 型を変更できる

```rust
fn main() {
    let spaces = "   ";        // 文字列型
    let spaces = spaces.len(); // 数値型
    
    println!("空白の数: {}", spaces);
}
```

#### 2. mutでは型変更できない

```rust
fn main() {
    let mut spaces = "   ";
    spaces = spaces.len(); // ← エラー！型が違う
}
```

**シャドーイングの使いどころ：**

* データを段階的に変換する場合
* 同じ概念を表す変数だが、型が変わる場合

---

### 練習問題 2-1

以下のプログラムを作成してみよう。

1. `name`という変数に自分の名前（文字列）を代入
2. 名前を出力
3. `name`をシャドーイングして、名前の文字数（数値）を代入
4. 文字数を出力

<details>
<summary>解答例を見る</summary>

```rust
fn main() {
    let name = "太郎";
    println!("名前: {}", name);
    
    let name = name.len();
    println!("名前のバイト数: {}", name);
}
```

**注意：** Rustの`len()`は文字数ではなくバイト数を返す。日本語は1文字3バイトなので注意しよう。

</details>

---

## データ型

Rustには、大きく分けて2種類のデータ型がある。

1. **スカラー型** - 単一の値を表す
2. **複合型** - 複数の値をグループ化

---

### 整数型

**整数型の種類：**

| 型 | 範囲 | 用途 |
|----|------|------|
| `i8` | -128 〜 127 | 非常に小さい数 |
| `i16` | -32,768 〜 32,767 | 小さい数 |
| `i32` | -2,147,483,648 〜 2,147,483,647 | **デフォルト（通常使う）** |
| `i64` | 約-9.2京 〜 9.2京 | 大きい数 |
| `i128` | 非常に大きい範囲 | 超大きい数 |
| `isize` | アーキテクチャ依存 | ポインタのサイズ |

**符号なし整数（0以上の数のみ）：**

| 型 | 範囲 | 用途 |
|----|------|------|
| `u8` | 0 〜 255 | バイトデータ |
| `u16` | 0 〜 65,535 | 小さい正数 |
| `u32` | 0 〜 4,294,967,295 | 正数 |
| `u64` | 0 〜 約18.4京 | 大きい正数 |
| `u128` | 0 〜 非常に大きい | 超大きい正数 |
| `usize` | アーキテクチャ依存 | 配列のインデックス |

**デフォルトの型：** `i32`（迷ったらこれを使う）

**例：**

```rust
fn main() {
    let x: i32 = 42;
    let y: u8 = 255;
    let z = 100; // i32と推論される
    
    println!("x = {}, y = {}, z = {}", x, y, z);
}
```

---

### 浮動小数点数型

**浮動小数点型：**

| 型 | 精度 | 用途 |
|----|------|------|
| `f32` | 単精度 | 省メモリ |
| `f64` | 倍精度 | **デフォルト（通常使う）** |

**例：**

```rust
fn main() {
    let pi: f64 = 3.14159265359;
    let e: f32 = 2.71828;
    
    println!("円周率: {}", pi);
    println!("自然対数の底: {}", e);
}
```

---

### 真偽値型

**真偽値型：** `bool`

**値：** `true` または `false`

**例：**

```rust
fn main() {
    let is_adult = true;
    let is_student = false;
    
    println!("成人？ {}", is_adult);
    println!("学生？ {}", is_student);
}
```

---

### 文字型

**文字型：** `char`

**特徴：**

* シングルクォート（`'`）で囲む
* Unicodeに対応（絵文字も使える！）
* 4バイト

**例：**

```rust
fn main() {
    let letter: char = 'A';
    let hiragana: char = 'あ';
    let emoji: char = '😀';
    
    println!("文字: {}", letter);
    println!("ひらがな: {}", hiragana);
    println!("絵文字: {}", emoji);
}
```

---

### 練習問題 3-1

以下の変数を宣言して出力してみよう。

1. 整数 `age`（型を明示）
2. 浮動小数点数 `height`（身長、単位はcm）
3. 真偽値 `is_programmer`
4. 文字 `blood_type`（血液型、例: 'A'）

<details>
<summary>解答例を見る</summary>

```rust
fn main() {
    let age: i32 = 25;
    let height: f64 = 170.5;
    let is_programmer: bool = true;
    let blood_type: char = 'O';
    
    println!("年齢: {}", age);
    println!("身長: {} cm", height);
    println!("プログラマー？ {}", is_programmer);
    println!("血液型: {} 型", blood_type);
}
```

</details>

---

## 複合型

### タプル

**タプル**は、複数の異なる型の値をグループ化できる。

**例：**

```rust
fn main() {
    let person: (String, i32, f64) = (String::from("太郎"), 25, 170.5);
    
    println!("名前: {}", person.0);
    println!("年齢: {}", person.1);
    println!("身長: {}", person.2);
}
```

**分割代入：**

```rust
fn main() {
    let person = (String::from("太郎"), 25, 170.5);
    let (name, age, height) = person;
    
    println!("名前: {}", name);
    println!("年齢: {}", age);
    println!("身長: {}", height);
}
```

---

### 配列

**配列**は、同じ型の値を固定長で格納する。

**宣言方法：**

```rust
fn main() {
    // 方法1: 値を列挙
    let numbers = [1, 2, 3, 4, 5];
    
    // 方法2: 型とサイズを明示
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    
    // 方法3: 同じ値で初期化
    let zeros = [0; 5]; // [0, 0, 0, 0, 0]
    
    println!("最初の数: {}", numbers[0]);
    println!("2番目の数: {}", numbers[1]);
}
```

**配列の長さ：**

```rust
fn main() {
    let numbers = [1, 2, 3, 4, 5];
    println!("配列の長さ: {}", numbers.len());
}
```

---

### 練習問題 4-1

以下のプログラムを作成してみよう。

1. 自分の情報（名前、年齢、身長）をタプルで作成
2. 分割代入で各値を取り出す
3. それぞれを出力

<details>
<summary>解答例を見る</summary>

```rust
fn main() {
    let my_info = (String::from("太郎"), 25, 170.5);
    let (name, age, height) = my_info;
    
    println!("名前: {}", name);
    println!("年齢: {} 歳", age);
    println!("身長: {} cm", height);
}
```

</details>

---

### 練習問題 4-2

以下のプログラムを作成してみよう。

1. 1から5までの数を配列に格納
2. 配列の長さを出力
3. 配列の各要素を出力

<details>
<summary>解答例を見る</summary>

```rust
fn main() {
    let numbers = [1, 2, 3, 4, 5];
    
    println!("配列の長さ: {}", numbers.len());
    println!("1番目: {}", numbers[0]);
    println!("2番目: {}", numbers[1]);
    println!("3番目: {}", numbers[2]);
    println!("4番目: {}", numbers[3]);
    println!("5番目: {}", numbers[4]);
}
```

</details>

---

## 型変換

### asキーワード

**数値型同士の変換：**

```rust
fn main() {
    let x: i32 = 100;
    let y: f64 = x as f64;
    
    println!("整数: {}", x);
    println!("浮動小数点: {}", y);
}
```

**注意：** 大きい型から小さい型への変換は、データが失われる可能性がある。

```rust
fn main() {
    let x: i32 = 1000;
    let y: i8 = x as i8; // ← 値が切り詰められる！
    
    println!("元の値: {}", x);
    println!("変換後: {}", y); // -24になる（オーバーフロー）
}
```

---

### 文字列への変換

**to_string() メソッド：**

```rust
fn main() {
    let number = 42;
    let text = number.to_string();
    
    println!("数値: {}", number);
    println!("文字列: {}", text);
}
```

**format! マクロ：**

```rust
fn main() {
    let name = "太郎";
    let age = 25;
    let message = format!("{}さんは{}歳です", name, age);
    
    println!("{}", message);
}
```

---

### 文字列から数値への変換

**parse() メソッド：**

```rust
fn main() {
    let text = "42";
    let number: i32 = text.parse().unwrap();
    
    println!("文字列: {}", text);
    println!("数値: {}", number);
}
```

**注意：** `unwrap()`はエラー時にパニックする。実際のコードでは適切なエラー処理が必要である（後の章で学ぶ）。

---

### 練習問題 5-1

以下のプログラムを作成してみよう。

1. 整数 `100` を浮動小数点数に変換
2. 浮動小数点数 `3.14` を整数に変換（小数点以下切り捨て）
3. 数値 `42` を文字列に変換
4. 文字列 `"123"` を数値に変換

すべて出力すること。

<details>
<summary>解答例を見る</summary>

```rust
fn main() {
    // 1. 整数 → 浮動小数点
    let int_val = 100;
    let float_val = int_val as f64;
    println!("整数 {} → 浮動小数点 {}", int_val, float_val);
    
    // 2. 浮動小数点 → 整数
    let pi = 3.14;
    let pi_int = pi as i32;
    println!("浮動小数点 {} → 整数 {}", pi, pi_int);
    
    // 3. 数値 → 文字列
    let num = 42;
    let num_str = num.to_string();
    println!("数値 {} → 文字列 \"{}\"", num, num_str);
    
    // 4. 文字列 → 数値
    let str_num = "123";
    let parsed: i32 = str_num.parse().unwrap();
    println!("文字列 \"{}\" → 数値 {}", str_num, parsed);
}
```

</details>

---

## 定数

### 定数とは？

**定数（constant）**は、プログラム全体で変更されない値である。

**変数との違い：**

| 項目 | 変数（let） | 定数（const） |
|------|------------|--------------|
| 変更 | `mut`で可能 | 不可能 |
| 型注釈 | 省略可能 | 必須 |
| 命名規則 | snake_case | UPPER_SNAKE_CASE |
| 評価 | 実行時 | コンパイル時 |

---

### 定数の宣言

```rust
const MAX_POINTS: u32 = 100_000;

fn main() {
    println!("最大ポイント: {}", MAX_POINTS);
}
```

**ポイント：**

* `const` キーワードを使う
* 型を明示する必要がある
* 大文字のスネークケース（UPPER_SNAKE_CASE）
* 数値リテラルにアンダースコア（`_`）を使える（読みやすさのため）

---

### 定数の使用例

```rust
const PI: f64 = 3.141592653589793;
const MAX_USERS: u32 = 1000;
const APP_NAME: &str = "My Rust App";

fn main() {
    println!("円周率: {}", PI);
    println!("最大ユーザー数: {}", MAX_USERS);
    println!("アプリ名: {}", APP_NAME);
    
    // 計算に使用
    let radius = 5.0;
    let area = PI * radius * radius;
    println!("半径{}の円の面積: {}", radius, area);
}
```

---

### 練習問題 6-1

以下の定数を宣言して使用してみよう。

1. `SECONDS_IN_HOUR`（1時間の秒数：3600）
2. `DAYS_IN_WEEK`（1週間の日数：7）
3. `GREETING`（挨拶メッセージ）

これらを使って、1週間の秒数を計算すること。

<details>
<summary>解答例を見る</summary>

```rust
const SECONDS_IN_HOUR: u32 = 3600;
const HOURS_IN_DAY: u32 = 24;
const DAYS_IN_WEEK: u32 = 7;
const GREETING: &str = "こんにちは、Rust！";

fn main() {
    println!("{}", GREETING);
    
    let seconds_in_day = SECONDS_IN_HOUR * HOURS_IN_DAY;
    let seconds_in_week = seconds_in_day * DAYS_IN_WEEK;
    
    println!("1日の秒数: {}", seconds_in_day);
    println!("1週間の秒数: {}", seconds_in_week);
}
```

</details>

---

## 総合練習問題

ここまでの内容を理解できたら、構造化された練習問題に挑戦しよう！

### 練習問題の場所

```bash
cd /workspaces/rust-practice/exercises/02_variables

# 練習問題一覧を表示
cargo run

# 個別の練習問題を実行
cargo run --bin ex01_basic
cargo run --bin ex02_shadowing
cargo run --bin challenge01_bmi
```

> **📁 練習問題ディレクトリ:** `exercises/02_variables/`  
> こちらには段階的な練習問題が用意されている。詳しくは[README.md](https://github.com/Rocky-Seven/rust-practice/tree/main/exercises/02_variables)を参照。

---

## 🎯 学習の進め方

### 1. 体験する（trials/）
- ブログ記事を読みながら `trials/02_variables/` で試す
- 自由にコードを書いて実験
- エラーを恐れず、どんどん試す

### 2. 練習する（exercises/）
- 体験が終わったら `exercises/02_variables/` で練習
- 構造化された問題を順番に解く
- 解答例も用意されている

---

### チャレンジ1：BMI計算機

以下の仕様でBMI計算プログラムを作成してみよう。

**仕様：**

1. 体重（kg）と身長（cm）を変数で宣言
2. 身長をメートルに変換
3. BMIを計算（体重 ÷ 身長の2乗）
4. 結果を出力

<details>
<summary>解答例を見る</summary>

```rust
fn main() {
    let weight: f64 = 70.0; // kg
    let height_cm: f64 = 170.0; // cm
    
    // 身長をメートルに変換
    let height_m = height_cm / 100.0;
    
    // BMI計算
    let bmi = weight / (height_m * height_m);
    
    println!("体重: {} kg", weight);
    println!("身長: {} cm", height_cm);
    println!("BMI: {:.2}", bmi);
}
```

</details>

---

### チャレンジ2：温度変換

以下の仕様で温度変換プログラムを作成してみよう。

**仕様：**

1. 摂氏温度を変数で宣言
2. 華氏に変換（F = C × 9/5 + 32）
3. 両方の温度を出力

<details>
<summary>解答例を見る</summary>

```rust
fn main() {
    let celsius: f64 = 25.0;
    let fahrenheit = celsius * 9.0 / 5.0 + 32.0;
    
    println!("摂氏: {}°C", celsius);
    println!("華氏: {}°F", fahrenheit);
}
```

</details>

---

### チャレンジ3：学生情報

以下の仕様で学生情報プログラムを作成してみよう。

**仕様：**

1. 学生情報をタプルで作成（名前、学年、GPA）
2. 分割代入で取り出す
3. 整形して出力

<details>
<summary>解答例を見る</summary>

```rust
fn main() {
    let student = (String::from("田中太郎"), 2, 3.75);
    let (name, grade, gpa) = student;
    
    println!("=== 学生情報 ===");
    println!("名前: {}", name);
    println!("学年: {} 年生", grade);
    println!("GPA: {:.2}", gpa);
}
```

</details>

---

### チャレンジ4：配列の操作

以下の仕様で配列操作プログラムを作成してみよう。

**仕様：**

1. テストの点数を配列で宣言（5科目）
2. 合計点を計算
3. 平均点を計算
4. すべて出力

<details>
<summary>解答例を見る</summary>

```rust
fn main() {
    let scores = [85, 92, 78, 95, 88];
    
    let total = scores[0] + scores[1] + scores[2] + scores[3] + scores[4];
    let average = total as f64 / scores.len() as f64;
    
    println!("=== テスト結果 ===");
    for i in 0..scores.len() {
        println!("科目{}: {} 点", i + 1, scores[i]);
    }
    println!("合計: {} 点", total);
    println!("平均: {:.1} 点", average);
}
```

</details>

---

## まとめ

この記事では、Rustの変数と型について学んだ。

### 学んだこと

* ✅ 変数の宣言（`let`）と可変変数（`let mut`）
* ✅ シャドーイング
* ✅ 基本的なデータ型
  * 整数型（i8〜i128、u8〜u128）
  * 浮動小数点型（f32、f64）
  * 真偽値型（bool）
  * 文字型（char）
* ✅ 複合型
  * タプル
  * 配列
* ✅ 型変換（`as`、`parse()`）
* ✅ 定数（`const`）

### GitHubに保存しよう

学習内容をGitHubに保存しよう。

```bash
cd /workspaces/rust-practice
git add .
git commit -m "Complete variables and types practice"
git push
```

---

## 次回予告

次回は、**関数**について学ぶ予定である。

* 関数の定義と呼び出し
* パラメータと戻り値
* 式と文の違い

お楽しみに！

---

**Happy Coding! 🦀**