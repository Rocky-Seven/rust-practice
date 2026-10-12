# Rust入門（7）「スライス」

前回は、値を貸し借りする**借用**について学んだ。今回は、文字列や配列の「一部分」だけを借用する**スライス**について学ぶ。これで、所有権シリーズ（第5〜7回）は完結となる。

## 学習の進め方

今回も同じく、2段階で進める。

まだ、GitHub Codespacesで環境構築をしていない方は、ブログ記事[第1回「GitHub Codespaces で Rust を始めよう！」](https://my-studies.org/get-started-with-rust-on-github-codespaces/)を先にお読みください。

1. 体験する（trials/）
ブログ記事を読みながら、trials/07_slices/で自由にコードを試す
自分で書き換えたり、わざとエラーを起こしたりして実験する
2. 練習する（exercises/）
体験が終わったら、exercises/07_slices/で構造化された練習問題に挑戦する
todo!()の部分を自分のコードに書き換えて完成させる
解答例も用意されている


## 準備

```bash
# リポジトリを最新の状態にする
cd /workspaces/rust-practice
git pull
# 体験用プロジェクトに移動
cd trials/06_borrowing
# VSCodeでファイルを開く
code src/main.rs
# 動作確認
cargo run
```
=== 第07回：スライスを学ぼう！ ===

と表示されれば成功。

## 文字列スライス

文字列の一部分を取り出したいとき、`&文字列[開始..終了]`という書き方を使う。

```rust
fn main() {
    let s = String::from("Hello, Rust!");

    let hello = &s[0..5];
    let rust = &s[7..11];

    println!("{}", hello);
    println!("{}", rust);
}
```

```
Hello
Rust
```

`&s[0..5]`は「`s`の0バイト目から5バイト目の手前まで」を指すスライスである。前回学んだ範囲（`0..5`）の書き方と同じで、終了位置は含まれない。

**ポイント：スライスは「借用」の一種である**

スライスは、元の文字列データを指す参照であり、データを複製しているわけではない。そのため、スライスが存在する間は、元の文字列を変更できない（前回学んだ借用のルールがここでも働く）。

## 文字列全体を受け取る関数

スライスを使うと、「`String`でも文字列リテラルでも受け取れる」柔軟な関数が書ける。

```rust
fn main() {
    let s1 = String::from("こんにちは Rust");
    let s2 = "Hello World"; // 文字列リテラル（&str型）

    println!("{}", first_word(&s1));
    println!("{}", first_word(s2));
}

fn first_word(text: &str) -> &str {
    let bytes = text.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &text[0..i];
        }
    }

    text
}
```

```
こんにちは
Hello
```

実は、これまで何度も登場してきた`&str`という型は、「文字列スライス」の正式な型名である。関数のパラメータを`&str`型にしておくと、`String`からも文字列リテラルからも、どちらからでも呼び出せる。

## 配列スライス

スライスは、文字列だけでなく配列にも使える。

```rust
fn main() {
    let numbers = [10, 20, 30, 40, 50];

    let middle = &numbers[1..4];

    println!("{:?}", middle);
    println!("合計: {}", sum(middle));
}

fn sum(nums: &[i32]) -> i32 {
    let mut total = 0;
    for n in nums {
        total += n;
    }
    total
}
```

```
[20, 30, 40]
合計: 90
```

`&[i32]`が、整数のスライスを表す型である。`sum`関数は配列の一部分だけでも、配列全体（`&numbers[..]`または単に`&numbers`）でも受け取れる。

## 練習問題

配列スライスを受け取り、その中の最大値を返す`find_max`関数を作ってみよう。

**解答例を見る**

```rust
fn main() {
    let numbers = [3, 7, 2, 9, 4];
    println!("最大値: {}", find_max(&numbers));
    println!("一部分の最大値: {}", find_max(&numbers[0..3]));
}

fn find_max(nums: &[i32]) -> i32 {
    let mut max = nums[0];
    for &n in nums {
        if n > max {
            max = n;
        }
    }
    max
}
```

配列全体でも、一部分のスライスでも、同じ関数で扱えることが確認できる。

## exercises/07_slicesの解答例

GitHubの`exercises/07_slices/`にある練習問題2問の解答例である。

### ex01_str_slice：文字列スライス

```rust
fn first_word(text: &str) -> &str {
    let bytes = text.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &text[0..i];
        }
    }

    text
}
```

記事内の`first_word`関数と同じ考え方である。先頭からバイトを順に見ていき、スペース（`b' '`）が見つかった位置までをスライスとして返す。スペースが1つも見つからなければ、文字列全体を返す。

### ex02_array_slice：配列スライス

```rust
fn average(nums: &[i32]) -> f64 {
    if nums.is_empty() {
        return 0.0;
    }

    let sum: i32 = nums.iter().sum();
    sum as f64 / nums.len() as f64
}
```

`nums.is_empty()`で空のスライスを先に処理し、0.0を返す（0で割るのを避けるため）。そうでなければ`.iter().sum()`で合計を求め、要素数で割って平均を計算する。整数同士の割り算は小数点以下が切り捨てられるため、`as f64`で先に浮動小数点数に変換している。

## 保存しよう

```bash
cd /workspaces/rust-practice
git add .
git commit -m "Complete lesson 7: slices"
git push
```

## 所有権シリーズのまとめ

3回にわたり、Rustの核心である所有権を学んだ。

- 第5回：値には必ず1つの所有者がいる。代入や関数呼び出しで、所有権は**ムーブ**する
- 第6回：複製したいなら**クローン**、所有権を移さず貸したいなら**借用（`&`、`&mut`）**
- 第7回：一部分だけを借用するのが**スライス**。`&str`はその代表例

これらのルールにより、Rustはガベージコレクションなしに、メモリの二重解放や不正アクセスをコンパイル時に防いでいる。CやPythonと見比べながら、挙動の違いを意識すると理解が深まるはずである。

## 次回予告

次回からは、複数のデータをまとめて扱う**構造体（struct）**について学んでいく予定である。
