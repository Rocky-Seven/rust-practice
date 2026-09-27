# Rustの制御構文を学ぼう！（第4回）

> **この記事は「Rust入門（3）関数とスコープを学ぼう！」の続編です。**
> まだ環境構築をしていない方は、[第1回の記事](https://my-studies.org/get-started-with-rust-on-github-codespaces/)を先にお読みください。

---
Codespacesをいったん削除した場合、Rustのインストールから始めてください。
```
1. インストールコマンドを実行
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
2. 環境を読み込む
source $HOME/.cargo/env
3. 確認
rustc --version
```

## はじめに

前回は、Rustの**関数とスコープ**について学んだ。

今回は、条件によって処理を分けたり、同じ処理を繰り返したりするための**制御構文**を学んでいこう。

### 今回学ぶこと

- if式（条件分岐）
- loop（無限ループ）
- while（条件付きループ）
- for（繰り返し処理）
- match式（パターンマッチ）

## 準備：体験用プロジェクトを開く

**作業場所:** trials/04_control_flow/
前回と同じように、このディレクトリで自由にコードを試してみよう。

```bash
# リポジトリを最新の状態にする
cd /workspaces/rust-practice
git pull

# 体験用プロジェクトに移動
cd trials/04_control_flow

# VSCodeでファイルを開く
code src/main.rs

# 動作確認
cargo run
```

## if式

### 基本のif

```rust
fn main() {
    let number = 7;

    if number % 2 == 0 {
        println!("{}は偶数である", number);
    } else {
        println!("{}は奇数である", number);
    }
}
```

```
# 出力
7は奇数である
```

**ポイント**

- 条件式は`bool`型でなければならない。他の言語のように`if 1`のような書き方はできない
- 条件を`()`で囲む必要はない（囲んでもエラーにはならないが、Rustの流儀ではない）

### 条件を数値と間違えるとエラー

```rust
fn main() {
    let number = 3;

    if number { // ← エラー！
        println!("何か");
    }
}
```

```
error[E0308]: mismatched types
（一部抜粋）expected `bool`, found integer
```

C言語では「0以外は真」として扱われるが、Rustではこの自動変換が行われない。`number != 0`のように、明示的に`bool`型の式を書く必要がある。

### else if で複数の条件

```rust
fn main() {
    let score = 75;

    if score >= 90 {
        println!("評価: A");
    } else if score >= 70 {
        println!("評価: B");
    } else if score >= 50 {
        println!("評価: C");
    } else {
        println!("評価: D");
    }
}
```

```
# 出力
評価: B
```

条件は上から順に評価され、最初に`true`になったブロックだけが実行される。

### 練習問題 1

以下のプログラムを作成してみよう。

1. `n`という変数（整数）を用意する
2. `n`が3で割り切れて、かつ5でも割り切れるなら「FizzBuzz」
3. 3でだけ割り切れるなら「Fizz」
4. 5でだけ割り切れるなら「Buzz」
5. どちらでもなければ、その数自体を出力

**解答例を見る**

```rust
fn main() {
    let n = 15;

    if n % 3 == 0 && n % 5 == 0 {
        println!("FizzBuzz");
    } else if n % 3 == 0 {
        println!("Fizz");
    } else if n % 5 == 0 {
        println!("Buzz");
    } else {
        println!("{}", n);
    }
}
```

`&&`は「かつ」（AND）、`||`は「または」（OR）を表す。

## ifは式である

### let文でifの結果を受け取る

前回、ブロック（`{ }`）は式になると学んだ。実は`if`も式であり、値を返せる。

```rust
fn main() {
    let number = 7;

    let description = if number % 2 == 0 {
        "偶数"
    } else {
        "奇数"
    };

    println!("{}は{}である", number, description);
}
```

```
# 出力
7は奇数である
```

**ポイント**

- `if`と`else`、両方のブロックが**同じ型**の値を返す必要がある
- 各ブロックの最後の式にセミコロンを付けない（前回学んだ「式」のルールと同じ）
- 他の言語の三項演算子（`条件 ? 値1 : 値2`）に近い使い方だが、Rustには三項演算子自体は存在しない

### 型が合わないとエラー

```rust
fn main() {
    let number = 7;

    let result = if number % 2 == 0 {
        "偶数" // 文字列
    } else {
        0 // 整数
    };

    println!("{}", result);
}
```

```
error[E0308]: `if` and `else` have incompatible types
```

`if`ブロックは`&str`、`else`ブロックは`i32`を返そうとしており、型が一致しないためエラーになる。

### 練習問題 2

以下のプログラムを作成してみよう。

1. `a`と`b`、2つの整数を用意する
2. `if`式を使って、大きいほうの値を`larger`という変数に代入する
3. `larger`を出力する

**解答例を見る**

```rust
fn main() {
    let a = 12;
    let b = 25;

    let larger = if a > b { a } else { b };

    println!("大きいほうの値: {}", larger);
}
```

## loop：無限ループ

### 基本のloop

`loop`は、明示的に止めるまで無限に繰り返す構文である。止めるには`break`を使う。

```rust
fn main() {
    let mut count = 0;

    loop {
        count += 1;
        println!("count = {}", count);

        if count == 3 {
            break;
        }
    }

    println!("ループ終了");
}
```

```
# 出力
count = 1
count = 2
count = 3
ループ終了
```

### loopは値を返せる

`break`の後ろに値を書くと、その値が`loop`全体の値になる。これも、`loop`が式であることの現れである。

```rust
fn main() {
    let mut count = 0;

    let result = loop {
        count += 1;

        if count == 10 {
            break count * 2;
        }
    };

    println!("result = {}", result);
}
```

```
# 出力
result = 20
```

`count`が10になった時点で`break count * 2;`が実行され、20が`result`に代入される。

### 練習問題 3

以下のプログラムを作成してみよう。

1. `n`という変数を1で初期化する
2. `loop`を使って、`n`を2倍し続ける
3. `n`が100を超えたら、その時点の`n`の値を`break`で返す
4. 結果を出力

**解答例を見る**

```rust
fn main() {
    let mut n = 1;

    let result = loop {
        n *= 2;

        if n > 100 {
            break n;
        }
    };

    println!("100を超えた最初の値: {}", result);
}
```

## while：条件付きループ

### 基本のwhile

条件が`true`の間だけ繰り返したいときは、`while`を使う。

```rust
fn main() {
    let mut count = 3;

    while count > 0 {
        println!("{}", count);
        count -= 1;
    }

    println!("発射！");
}
```

```
# 出力
3
2
1
発射！
```

**loopとの違い**

- `loop`は無限ループで、`break`で自分で止める
- `while`は、条件を毎回チェックして、`false`になったら自動的に止まる

条件判定が先にある分、`while`のほうが「何回繰り返すか」が読み取りやすい場面が多い。

### 練習問題 4

以下のプログラムを作成してみよう。

1. `n`という変数を10で初期化する
2. `while`を使って、`n`が0より大きい間、`n`を出力しながら1ずつ減らす
3. ただし、3の倍数のときだけは出力をスキップする（`continue`を使う）

**ヒント：** `continue`は、ループの残りをスキップして次の周回に進む構文である。

**解答例を見る**

```rust
fn main() {
    let mut n = 10;

    while n > 0 {
        if n % 3 == 0 {
            n -= 1;
            continue;
        }

        println!("{}", n);
        n -= 1;
    }
}
```

## for：繰り返し処理

### 範囲（Range）を使う

決まった回数を繰り返すときは、`for`と**範囲**（`開始..終了`）を組み合わせるのが最も一般的である。

```rust
fn main() {
    for i in 1..5 {
        println!("i = {}", i);
    }
}
```

```
# 出力
i = 1
i = 2
i = 3
i = 4
```

**ポイント**

- `1..5`は「1以上5未満」を表す（5は含まれない）
- 5を含めたい場合は、`1..=5`（`=`を付ける）と書く

```rust
fn main() {
    for i in 1..=5 {
        println!("i = {}", i);
    }
}
```

```
# 出力
i = 1
i = 2
i = 3
i = 4
i = 5
```

### 逆順に繰り返す

`.rev()`を付けると、範囲を逆順にできる。

```rust
fn main() {
    for i in (1..=5).rev() {
        println!("{}", i);
    }
    println!("発射！");
}
```

```
# 出力
5
4
3
2
1
発射！
```

先ほど`while`で書いたカウントダウンは、`for`を使うとこのように簡潔に書ける。

### 配列を繰り返す

`for`は、配列や前回学んだ`Vec`の要素を順番に取り出すこともできる。

```rust
fn main() {
    let numbers = [10, 20, 30, 40, 50];

    for n in numbers {
        println!("{}", n);
    }
}
```

```
# 出力
10
20
30
40
50
```

`while`で配列を扱う場合、添字（インデックス）の範囲を自分で管理する必要があり、範囲を間違えるとエラーになる可能性がある。`for`はその心配がなく安全である。

### 練習問題 5

以下のプログラムを作成してみよう。

1. 1から30までの整数について、`for`で繰り返す
2. 3の倍数のときは「Fizz」
3. 5の倍数のときは「Buzz」
4. 両方の倍数のときは「FizzBuzz」
5. それ以外はその数自体を出力

これは、練習問題1で作った条件分岐を、`for`で30回分繰り返すFizzBuzzである。

**解答例を見る**

```rust
fn main() {
    for n in 1..=30 {
        if n % 3 == 0 && n % 5 == 0 {
            println!("FizzBuzz");
        } else if n % 3 == 0 {
            println!("Fizz");
        } else if n % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", n);
        }
    }
}
```

## match式

### matchの基本

`match`は、値がどのパターンに一致するかで処理を分ける構文である。`if`の`else if`が増えすぎて読みにくいときに、特に効果を発揮する。

```rust
fn main() {
    let day = 3;

    let name = match day {
        1 => "月曜日",
        2 => "火曜日",
        3 => "水曜日",
        4 => "木曜日",
        5 => "金曜日",
        6 => "土曜日",
        7 => "日曜日",
        _ => "無効な値",
    };

    println!("{}", name);
}
```

```
# 出力
水曜日
```

**ポイント**

- `match`も式であり、値を返せる（`if`式と同じ考え方）
- `_`は「それ以外のすべて」を表すワイルドカードである
- **すべてのパターンを網羅する必要がある**。`_`を書き忘れると、コンパイルエラーになる

### 網羅していないとエラー

```rust
fn main() {
    let day = 3;

    let name = match day {
        1 => "月曜日",
        2 => "火曜日",
        3 => "水曜日",
        // _ がない！
    };

    println!("{}", name);
}
```

```
error[E0004]: non-exhaustive patterns: `i32::MIN..=0_i32` and `4_i32..=i32::MAX` not covered
```

`day`は`i32`型なので、1、2、3以外にも無数の値がありうる。すべてを書ききれないため、`_`で「それ以外」を必ず受け止める必要がある。このチェックのおかげで、パターンの書き漏れをコンパイル時に発見できる。

### 範囲でパターンを書く

`match`のパターンには、範囲（`1..=5`のような書き方）も使える。

```rust
fn main() {
    let score = 75;

    let grade = match score {
        90..=100 => "A",
        70..=89 => "B",
        50..=69 => "C",
        _ => "D",
    };

    println!("評価: {}", grade);
}
```

```
# 出力
評価: B
```

先ほど`if`と`else if`で書いた成績判定は、`match`を使うとこのように書ける。条件の候補が増えても見通しがよい。

### 複数の値を1つのパターンにまとめる

`|`（パイプ）を使うと、複数の値を1つのパターンにまとめられる。

```rust
fn main() {
    let n = 4;

    match n {
        1 | 3 | 5 | 7 | 9 => println!("{}は奇数（1桁）", n),
        0 | 2 | 4 | 6 | 8 => println!("{}は偶数（1桁）", n),
        _ => println!("{}は1桁の数字ではない", n),
    }
}
```

```
# 出力
4は偶数（1桁）
```

### 練習問題 6

以下のプログラムを作成してみよう。`match`を使うこと。

1. `month`という変数（1〜12の整数）を用意する
2. 3〜5月なら「春」
3. 6〜8月なら「夏」
4. 9〜11月なら「秋」
5. 12、1、2月なら「冬」
6. それ以外の値なら「無効な月」

**解答例を見る**

```rust
fn main() {
    let month = 8;

    let season = match month {
        3..=5 => "春",
        6..=8 => "夏",
        9..=11 => "秋",
        12 | 1 | 2 => "冬",
        _ => "無効な月",
    };

    println!("{}月は{}である", month, season);
}
```

`|`で複数の値をまとめられるので、12月・1月・2月のように連続しない値も1つのパターンにできる。

## 関数と組み合わせる

制御構文は、前回学んだ関数と組み合わせることで、より実践的なコードになる。

```rust
fn main() {
    for n in 1..=5 {
        println!("{}は素数？ {}", n, is_prime(n));
    }
}

fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }

    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 1;
    }

    true
}
```

```
# 出力
1は素数？ false
2は素数？ true
3は素数？ true
4は素数？ false
5は素数？ true
```

`is_prime`関数の中で、`while`ループと`return`（前回学んだ、途中で値を返す構文）を組み合わせている。2で割り切れる、3で割り切れる……と、`i`を増やしながら順番に調べ、途中で割り切れる数が見つかった時点で`false`を返している。

## 総合練習問題

ここまでの内容を理解できたら、練習問題に挑戦しよう！

### 練習問題の場所

```bash
cd /workspaces/rust-practice/exercises/04_control_flow

# 練習問題一覧を表示
cargo run

# 個別の練習問題を実行
cargo run --bin ex01_if
cargo run --bin ex02_loop
cargo run --bin ex03_match
cargo run --bin challenge01_fizzbuzz
cargo run --bin challenge02_prime
cargo run --bin challenge03_grade
```

練習問題のファイルには、`todo!()`と書かれた部分がある（`ex01_if`は出力を予想する問題なので`todo!()`はない）。ここを自分のコードに書き換えて完成させよう。完成したかどうかは、次のコマンドでテストして確認できる。

```bash
# 例：ex02_loopのテストを実行
cargo test --bin ex02_loop
```

`test ... ok`と表示されれば正解である。

---

### チャレンジ1：FizzBuzz関数

FizzBuzzを、関数として独立させてみよう。

**仕様：**

1. 整数を受け取り、それに対応する文字列（"Fizz"、"Buzz"、"FizzBuzz"、または数値そのもの）を返す`fizzbuzz`関数を作る
2. `main`関数で、1から30まで`for`で繰り返し、`fizzbuzz`の結果を出力

**解答例を見る**

```rust
fn main() {
    for n in 1..=30 {
        println!("{}", fizzbuzz(n));
    }
}

fn fizzbuzz(n: u32) -> String {
    if n % 3 == 0 && n % 5 == 0 {
        String::from("FizzBuzz")
    } else if n % 3 == 0 {
        String::from("Fizz")
    } else if n % 5 == 0 {
        String::from("Buzz")
    } else {
        n.to_string()
    }
}
```

`String::from`と`.to_string()`は、どちらも文字列を作る方法である。`if`の各ブロックは同じ型（`String`）を返す必要があるため、数値の`n`もそのままでは使えず、`.to_string()`で文字列に変換している。

---

### チャレンジ2：素数判定関数

先ほどの`is_prime`関数を使って、素数を一覧表示してみよう。

**仕様：**

1. `is_prime(n)`関数（記事内のものを利用）
2. `main`関数で、2から50までの整数のうち、素数だけを出力する

**解答例を見る**

```rust
fn main() {
    for n in 2..=50 {
        if is_prime(n) {
            print!("{} ", n);
        }
    }
    println!();
}

fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }

    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 1;
    }

    true
}
```

`print!`は`println!`と違い、改行しない出力である。すべての素数を1行に並べたいときに使える。

---

### チャレンジ3：成績判定関数（match版）

複数人分の点数を、`match`を使った関数で判定してみよう。

**仕様：**

1. 点数（`u32`）を受け取り、評価（`&str`）を返す`grade`関数を`match`で作る（90以上A、70以上B、50以上C、それ以外D）
2. `[95, 82, 61, 40]`の配列を`for`で繰り返し、それぞれの点数と評価を出力

**解答例を見る**

```rust
fn main() {
    let scores = [95, 82, 61, 40];

    for score in scores {
        println!("点数: {}, 評価: {}", score, grade(score));
    }
}

fn grade(score: u32) -> &'static str {
    match score {
        90..=100 => "A",
        70..=89 => "B",
        50..=69 => "C",
        _ => "D",
    }
}
```

`&'static str`は、「プログラム全体を通じて有効な文字列」を表す型である。今は`&str`の仲間だと理解しておけば十分で、詳しくは次回の所有権で扱う。

---

## 🎯 学習の進め方

### 1. 体験する（trials/）

- ブログ記事を読みながら `trials/04_control_flow/` で試す
- 自由にコードを書いて実験
- エラーを恐れず、どんどん試す

### 2. 練習する（exercises/）

- 体験が終わったら `exercises/04_control_flow/` で練習
- 構造化された問題を順番に解く
- テストで答え合わせができる

---

## まとめ

この記事では、Rustの制御構文について学んだ。

### 学んだこと

- ✅ if式（条件分岐、値を返す式としてのif）
- ✅ loop（無限ループ、breakで値を返す）
- ✅ while（条件付きループ）
- ✅ for（範囲、逆順、配列の繰り返し）
- ✅ match式（パターン、範囲、`|`、網羅性チェック）

### GitHubに保存しよう

学習内容をGitHubに保存しよう。

```bash
cd /workspaces/rust-practice
git add .
git commit -m "Complete control flow practice"
git push
```

---

## 次回予告

次回は、Rustの最大の特徴である**所有権**について学ぶ予定である。

- 所有権のルール
- ムーブとクローン
- 借用と参照
- スライス

お楽しみに！

---

**Happy Coding! 🦀**
