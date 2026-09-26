# Rustの関数とスコープを学ぼう！（第3回）

> **この記事は「Rust入門（2）変数と型を学ぼう！」の続編です。**
> まだ環境構築をしていない方は、[第1回の記事](https://my-studies.org/get-started-with-rust-on-github-codespaces/)を先にお読みください。

Codespacesをいったん削除した場合、Rustのインストールから始めてください。
```
1. インストールコマンドを実行
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
2. 環境を読み込む
source $HOME/.cargo/env
3. 確認
rustc --version
```
---

## はじめに

前回は、Rustの**変数と型**について学んだ。

今回は、プログラムを部品に分けて整理するための**関数**と、変数が使える範囲を決める**スコープ**について学んでいこう。

### 今回学ぶこと

- 関数の定義と呼び出し
- パラメータ（引数）
- 戻り値
- 複数の値を返す（タプル）
- 式と文の違い
- スコープ（変数が有効な範囲）

## 準備：体験用プロジェクトを開く

**作業場所:** trials/03_functions/
前回と同じように、このディレクトリで自由にコードを試してみよう。失敗しても問題ない。

Codespacesを開いて、ターミナルで以下のコマンドを実行しよう。

```bash
# リポジトリを最新の状態にする
cd /workspaces/rust-practice
git pull

# 体験用プロジェクトに移動
cd trials/03_functions

# VSCodeでファイルを開く
code src/main.rs

# 動作確認
cargo run

# 練習問題
cd /workspaces/rust-practice/exercises/03_functions
cargo run

```

これで、体験用プロジェクトの準備が整った。

## 関数の基本

### 関数を定義して呼び出す

実は、これまで書いてきた`fn main()`も関数である。`main`関数はプログラムの入口となる特別な関数だが、自分で好きな名前の関数をつくることもできる。

src/main.rsを開いて、以下のコードを書いてみよう。

```rust
fn main() {
    println!("main関数から始まる");
    say_hello();
    say_hello();
}

fn say_hello() {
    println!("こんにちは、Rust！");
}
```

```
# 実行
cargo run

# 出力
main関数から始まる
こんにちは、Rust！
こんにちは、Rust！
```

**ポイント**

- 関数は`fn`キーワードで定義する
- 関数名は`snake_case`（小文字とアンダースコア）で書く
- 呼び出すときは、`関数名()`と書く
- 同じ処理を何度も書かずに、関数にまとめて再利用できる

**定義の順番は自由**

上のコードでは、`main`関数の後ろで`say_hello`関数を定義している。Rustでは、呼び出す側より後ろに関数を書いても問題ない。C言語のようにプロトタイプ宣言をする必要はない。

**補足：println!の「!」は何？**

`println!`は関数ではなく**マクロ**と呼ばれるものである。名前の最後に`!`が付いているのが目印だ。今のところは「関数によく似た仕組み」と覚えておけばよい。

### 練習問題 1

以下のプログラムを作成してみよう。

1. 区切り線（`====================`）を出力する`print_line`関数を作る
2. `main`関数で、`print_line`を呼び出す
3. その後に「Rustの関数を学ぼう！」と出力する
4. もう一度`print_line`を呼び出す

**解答例を見る**

```rust
fn main() {
    print_line();
    println!("Rustの関数を学ぼう！");
    print_line();
}

fn print_line() {
    println!("====================");
}
```

## パラメータ（引数）

### パラメータを受け取る

関数に値を渡したいときは、**パラメータ**（引数）を使う。

```rust
fn main() {
    greet("太郎");
    greet("花子");
}

fn greet(name: &str) {
    println!("こんにちは、{}さん！", name);
}
```

```
# 出力
こんにちは、太郎さん！
こんにちは、花子さん！
```

**ポイント**

- パラメータは`名前: 型`の形で、関数名の後ろの`()`の中に書く
- 関数のパラメータには、**型を必ず明示する**（変数の`let`のような型推論は働かない）
- `&str`は文字列リテラル（`"太郎"`のような文字列）の型である。今は「文字列を受け取るときは`&str`と書く」と覚えておこう（詳しくは第5回の所有権で学ぶ）

### 複数のパラメータ

パラメータが複数あるときは、カンマ（`,`）で区切る。

```rust
fn main() {
    print_sum(3, 5);
    print_rectangle_area(4.5, 2.0);
}

fn print_sum(a: i32, b: i32) {
    println!("{} + {} = {}", a, b, a + b);
}

fn print_rectangle_area(width: f64, height: f64) {
    println!("横{:.1}、縦{:.1}の長方形の面積: {:.1}", width, height, width * height);
}
```

```
# 出力
3 + 5 = 8
横4.5、縦2.0の長方形の面積: 9.0
```

`{:.1}`は、小数点以下1桁で表示する指定である。

**型が合わないとエラー**

パラメータの型と違う値を渡すと、コンパイルエラーになる。たとえば、`f64`のパラメータに整数の`4`を渡してみよう。

```rust
fn main() {
    print_rectangle_area(4, 2); // ← エラー！
}

fn print_rectangle_area(width: f64, height: f64) {
    println!("面積: {:.1}", width * height);
}
```

```
error[E0308]: mismatched types
（一部抜粋）expected `f64`, found integer
```

`4.0`のように、小数点を付けて渡せば解決する。Rustは、整数から浮動小数点数への変換を自動ではしてくれない。

### 練習問題 2

以下のプログラムを作成してみよう。

1. 名前（`&str`）と年齢（`i32`）を受け取る`introduce`関数を作る
2. 「太郎さんは25歳です」のように出力する
3. `main`関数から、2人分を呼び出す

**解答例を見る**

```rust
fn main() {
    introduce("太郎", 25);
    introduce("花子", 30);
}

fn introduce(name: &str, age: i32) {
    println!("{}さんは{}歳です", name, age);
}
```

## 戻り値

### 値を返す関数

関数から呼び出し元に値を返したいときは、`->`の後ろに**戻り値の型**を書く。

```rust
fn main() {
    let result = add(3, 5);
    println!("3 + 5 = {}", result);
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

```
# 出力
3 + 5 = 8
```

**ポイント**

- `-> i32`は、「この関数は`i32`型の値を返す」という意味である
- 関数の**最後の式**（セミコロンを付けない行）の値が、そのまま戻り値になる
- 他の言語のように`return`を書かなくても値を返せる

### セミコロンを付けるとエラー

戻り値にしたい行の最後にセミコロン（`;`）を付けると、エラーになる。

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b; // ← セミコロンを付けるとエラー！
}
```

```
error[E0308]: mismatched types
（一部抜粋）expected `i32`, found `()`
（一部抜粋）help: remove this semicolon to return this value
```

セミコロンを付けると、`a + b`は「値を返さない文」になってしまう。すると、この関数は何も返さない（`()`を返す）ことになり、`-> i32`と矛盾するためエラーになるのだ。

エラーメッセージにも「セミコロンを外せば値を返せる」というヒントが出ている。Rustのエラーメッセージは親切なので、よく読んでみよう。

### returnで途中で抜ける

関数の途中で値を返したいときは、`return`キーワードを使う。

```rust
fn main() {
    println!("-5の絶対値: {}", absolute(-5));
    println!("3の絶対値: {}", absolute(3));
}

fn absolute(n: i32) -> i32 {
    if n < 0 {
        return -n;
    }
    n
}
```

```
# 出力
-5の絶対値: 5
3の絶対値: 3
```

`if`は「条件によって処理を分ける」ための構文である。次回、詳しく学ぶので、今は「`n`が0より小さいときだけ`return -n;`が実行される」と理解しておけばよい。

**使い分けの目安**

- 通常は、最後の式で値を返す
- 条件によって早めに関数を終わらせたいときに`return`を使う

### 複数の値を返す（タプル）

前回学んだ**タプル**を使うと、複数の値をまとめて返せる。

```rust
fn main() {
    let (sum, product) = calc(4, 5);
    println!("和: {}, 積: {}", sum, product);

    let (quotient, remainder) = divide(17, 5);
    println!("17 ÷ 5 = {} あまり {}", quotient, remainder);
}

fn calc(a: i32, b: i32) -> (i32, i32) {
    (a + b, a * b)
}

fn divide(a: i32, b: i32) -> (i32, i32) {
    (a / b, a % b)
}
```

```
# 出力
和: 9, 積: 20
17 ÷ 5 = 3 あまり 2
```

戻り値を受け取るときも、前回学んだ**分割代入**（`let (sum, product) = ...`）が使える。

### 練習問題 3

以下のプログラムを作成してみよう。

1. 整数を受け取り、その2乗を返す`square`関数
2. 整数を受け取り、偶数なら`true`、奇数なら`false`を返す`is_even`関数
3. `main`関数で、5の2乗と、4と7が偶数かどうかを出力

**ヒント：** 偶数かどうかは、2で割ったあまり（`n % 2`）が0かどうかで判定できる。

**解答例を見る**

```rust
fn main() {
    println!("5の2乗: {}", square(5));
    println!("4は偶数？ {}", is_even(4));
    println!("7は偶数？ {}", is_even(7));
}

fn square(n: i32) -> i32 {
    n * n
}

fn is_even(n: i32) -> bool {
    n % 2 == 0
}
```

## 式と文の違い

### 式と文とは？

戻り値の仕組みを理解するために、Rustの**式（expression）**と**文（statement）**の違いを押さえておこう。

| **種類** | **特徴** | **例** |
| ------ | ------ | ------ |
| 文 | 何かを実行するが、値を返さない | `let x = 5;` |
| 式 | 計算して、値になる | `5`、`x + 1`、`add(1, 2)` |

Rustの関数は、いくつかの文と、最後に置く式で構成される。「最後の式が戻り値になる」というのは、この仕組みによるものだ。

### ブロックも式

`{ }`で囲んだ**ブロック**も、最後に式があれば、その値を持つ式になる。

```rust
fn main() {
    let y = {
        let x = 3;
        x + 1 // セミコロンなし → ブロックの値になる
    };

    println!("yの値は: {}", y);
}
```

```
# 出力
yの値は: 4
```

ブロックの中の`x + 1`にセミコロンがないので、その値（4）がブロック全体の値になり、`y`に代入される。

なお、`let`は文なので、値を返さない。そのため、`let x = (let y = 6);`のような書き方はできない。

### 練習問題 4

ブロックを使って、以下のプログラムを作成してみよう。

1. `total`という変数に、ブロックの値を代入する
2. ブロックの中で、10、20、30を別々の変数に入れて、合計を最後の式にする
3. `total`を出力

**解答例を見る**

```rust
fn main() {
    let total = {
        let a = 10;
        let b = 20;
        let c = 30;
        a + b + c
    };

    println!("合計: {}", total);
}
```

## スコープ

### スコープとは？

変数が使える範囲のことを**スコープ**という。Rustでは、変数は宣言された位置から、それを含むブロック（`{ }`）の終わりまで有効である。

```rust
fn main() {
    let x = 10;

    {
        let y = 20;
        println!("内側: x = {}, y = {}", x, y);
    }

    println!("外側: x = {}", x);
}
```

```
# 出力
内側: x = 10, y = 20
外側: x = 10
```

- 内側のブロックからは、外側の`x`も使える
- `y`は内側のブロックの中だけで有効である

### スコープの外では使えない

内側で宣言した変数を、ブロックの外で使おうとするとエラーになる。

```rust
fn main() {
    {
        let y = 20;
        println!("内側: y = {}", y);
    }

    println!("外側: y = {}", y); // ← エラー！
}
```

```
error[E0425]: cannot find value `y` in this scope
```

ブロックが終わった時点で、`y`は使えなくなる。これが「スコープを抜ける」ということだ。

### 内側でのシャドーイング

前回学んだシャドーイングを、内側のブロックで使うとどうなるだろうか。

```rust
fn main() {
    let x = 1;

    {
        let x = 2;
        println!("内側のx: {}", x);
    }

    println!("外側のx: {}", x);
}
```

```
# 出力
内側のx: 2
外側のx: 1
```

内側の`x`は、ブロックの中だけで外側の`x`を隠している。ブロックを抜けると、外側の`x`（1）に戻る。

### 関数の間で変数は共有されない

関数の中で宣言した変数は、その関数の中だけで有効である。別の関数からは使えない。

```rust
fn main() {
    let message = "main関数の変数";
    show();
}

fn show() {
    println!("{}", message); // ← エラー！
}
```

```
error[E0425]: cannot find value `message` in this scope
```

別の関数に値を渡したいときは、**パラメータ**を使う。

```rust
fn main() {
    let message = "main関数の変数";
    show(message);
}

fn show(message: &str) {
    println!("{}", message);
}
```

```
# 出力
main関数の変数
```

**定数は例外**

関数の外側で宣言した定数（`const`）は、どの関数からも使える。前回学んだ定数を、関数と組み合わせてみよう。

```rust
const TAX_PERCENT: u32 = 10;

fn main() {
    let price = 1000;
    println!("税込み価格: {}円", price_with_tax(price));
}

fn price_with_tax(price: u32) -> u32 {
    price + price * TAX_PERCENT / 100
}
```

```
# 出力
税込み価格: 1100円
```

### 練習問題 5

以下のプログラムを実行すると、どのように出力されるだろうか。実行する前に予想してみよう。

```rust
fn main() {
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("A: {}", x);
    }

    println!("B: {}", x);
}
```

**解答を見る**

```
A: 12
B: 6
```

**解説**

1. 最初の`x`は5
2. シャドーイングで、`x`は6（5 + 1）になる
3. 内側のブロックでは、さらにシャドーイングして`x`は12（6 * 2）になる。これが「A」として出力される
4. ブロックを抜けると、内側の`x`（12）は消えて、外側の`x`（6）に戻る。これが「B」として出力される

---

## 総合練習問題

ここまでの内容を理解できたら、構造化された練習問題に挑戦しよう！

### 練習問題の場所

```bash
cd /workspaces/rust-practice/exercises/03_functions

# 練習問題一覧を表示
cargo run

# 個別の練習問題を実行
cargo run --bin ex01_basic
cargo run --bin ex02_return
cargo run --bin ex03_scope
cargo run --bin challenge01_bmi
cargo run --bin challenge02_temperature
cargo run --bin challenge03_circle
```

> **📁 練習問題ディレクトリ:** `exercises/03_functions/`
> ファイルの中の `todo!()` を自分のコードに書き換えて完成させよう。
> `cargo test --bin ファイル名` でテストすると、正解かどうかを確認できる。

---

### チャレンジ1：BMI計算関数

前回のBMI計算を、関数にまとめてみよう。

**仕様：**

1. 体重（kg）と身長（cm）を受け取り、BMIを返す`calc_bmi`関数を作る
2. `main`関数で、体重70kg、身長170cmのBMIを小数点以下2桁で出力

**解答例を見る**

```rust
fn main() {
    let bmi = calc_bmi(70.0, 170.0);
    println!("BMI: {:.2}", bmi);
}

fn calc_bmi(weight_kg: f64, height_cm: f64) -> f64 {
    let height_m = height_cm / 100.0;
    weight_kg / (height_m * height_m)
}
```

---

### チャレンジ2：温度変換関数

前回の温度変換を、関数にまとめてみよう。

**仕様：**

1. 摂氏を華氏に変換する`c_to_f`関数（F = C × 9/5 + 32）
2. 華氏を摂氏に変換する`f_to_c`関数（C = (F − 32) × 5/9）
3. 25℃を華氏に変換し、その結果を摂氏に戻して、両方を出力

**解答例を見る**

```rust
fn main() {
    let celsius = 25.0;
    let fahrenheit = c_to_f(celsius);

    println!("摂氏{:.1}°C = 華氏{:.1}°F", celsius, fahrenheit);
    println!("華氏{:.1}°F = 摂氏{:.1}°C", fahrenheit, f_to_c(fahrenheit));
}

fn c_to_f(c: f64) -> f64 {
    c * 9.0 / 5.0 + 32.0
}

fn f_to_c(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}
```

---

### チャレンジ3：円の面積と円周

定数と関数を組み合わせて、円の情報を計算しよう。

**仕様：**

1. 円周率を定数`PI`として宣言
2. 半径から面積を返す`circle_area`関数
3. 半径から円周を返す`circle_circumference`関数
4. 面積と円周をタプルで返す`circle_info`関数
5. 半径5の円の面積と円周を出力

**解答例を見る**

```rust
const PI: f64 = 3.141592653589793;

fn main() {
    let (area, circumference) = circle_info(5.0);
    println!("半径5の円 面積: {:.2}, 円周: {:.2}", area, circumference);
}

fn circle_area(r: f64) -> f64 {
    PI * r * r
}

fn circle_circumference(r: f64) -> f64 {
    2.0 * PI * r
}

fn circle_info(r: f64) -> (f64, f64) {
    (circle_area(r), circle_circumference(r))
}
```

---

## 🎯 学習の進め方

### 1. 体験する（trials/）

- ブログ記事を読みながら `trials/03_functions/` で試す
- 自由にコードを書いて実験
- エラーを恐れず、どんどん試す

### 2. 練習する（exercises/）

- 体験が終わったら `exercises/03_functions/` で練習
- 構造化された問題を順番に解く
- テストで答え合わせができる

---

## まとめ

この記事では、Rustの関数とスコープについて学んだ。

### 学んだこと

- ✅ 関数の定義と呼び出し（`fn`）
- ✅ パラメータ（型の明示が必要）
- ✅ 戻り値（`->`、最後の式、`return`）
- ✅ 複数の値を返す（タプル）
- ✅ 式と文の違い
- ✅ スコープ（ブロック、関数、定数）

### GitHubに保存しよう

学習内容をGitHubに保存しよう。

```bash
cd /workspaces/rust-practice
git add .
git commit -m "Complete functions and scope practice"
git push
```

---

## 次回予告

次回は、**制御構文**について学ぶ予定である。

- if式
- loop、while、for
- match式

お楽しみに！

---

**Happy Coding! 🦀**
